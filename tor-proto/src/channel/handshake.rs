//! Implementations for the channel handshake

use arrayref::array_ref;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use crate::channel::codec::ChannelCodec;
use crate::channel::LogId;
use crate::{Error, Result};
use tor_cell::chancell::{msg, ChanCmd};

use std::net;
use tor_bytes::Reader;
use tor_linkspec::ChanTarget;
use tor_llcrypto as ll;
use tor_llcrypto::pk::ed25519::Ed25519Identity;
use tor_llcrypto::pk::rsa::RSAIdentity;

use digest::Digest;

use super::CellFrame;

use log::{debug, trace};

/// A list of the link protocols that we support.
// We only support version 4 for now, since we don't do padding right.
static LINK_PROTOCOLS: &[u16] = &[4];

/// A raw client channel on which nothing has been done.
pub struct OutboundClientHandshake<T: AsyncRead + AsyncWrite + Send + Unpin + 'static> {
    /// Underlying TLS stream.
    ///
    /// (We don't enforce that this is actually TLS, but if it isn't, the
    /// connection won't be secure.)
    tls: T,

    /// Declared target for this stream, if any.  (Used for logging.)
    target: Option<std::net::SocketAddr>,

    /// Logging identifier for this stream.  (Used for logging only.)
    logid: LogId,
}

/// A client channel on which versions have been negotiated and the
/// server's handshake has been read, but where the certs have not
/// been checked.
pub struct UnverifiedChannel<T: AsyncRead + AsyncWrite + Send + Unpin + 'static> {
    /// The negotiated link protocol.  Must be a member of LINK_PROTOCOLS
    link_protocol: u16,
    /// The Source+Sink on which we're reading and writing cells.
    tls: CellFrame<T>,
    /// The certs cell that we got from the relay.
    certs_cell: msg::Certs,
    /// The netinfo cell that we got from the relay.
    #[allow(dead_code)] // Relays will need this.
    netinfo_cell: msg::Netinfo,
    /// Logging identifier for this stream.  (Used for logging only.)
    logid: LogId,
}

/// A client channel on which versions have been negotiated,
/// server's handshake has been read, but the client has not yet
/// finished the handshake.
///
/// This type is separate from UnverifiedChannel, since finishing the
/// handshake requires a bunch of CPU, and you might want to do it as
/// a separate task or after a yield.
pub struct VerifiedChannel<T: AsyncRead + AsyncWrite + Send + Unpin + 'static> {
    /// The negotiated link protocol.
    link_protocol: u16,
    /// The Source+Sink on which we're reading and writing cells.
    tls: CellFrame<T>,
    /// Logging identifier for this stream.  (Used for logging only.)
    logid: LogId,
    /// Validated Ed25519 identity for this peer.
    ed25519_id: Ed25519Identity,
    /// Validated RSA identity for this peer.
    rsa_id: RSAIdentity,
}

impl<T: AsyncRead + AsyncWrite + Send + Unpin + 'static> OutboundClientHandshake<T> {
    /// Construct a new OutboundClientHandshake.
    pub(crate) fn new(tls: T, target: Option<std::net::SocketAddr>) -> Self {
        Self {
            tls,
            target,
            logid: LogId::new(),
        }
    }

    /// Negotiate a link protocol version with the relay, and read
    /// the relay's handshake information.
    pub async fn connect(mut self) -> Result<UnverifiedChannel<T>> {
        match self.target {
            Some(addr) => debug!("{}: starting Tor handshake with {}", self.logid, addr),
            None => debug!("{}: starting Tor handshake", self.logid),
        }
        trace!("{}: sending versions", self.logid);
        // Send versions cell
        {
            let my_versions = msg::Versions::new(LINK_PROTOCOLS);
            self.tls.write(&my_versions.encode_for_handshake()).await?;
            self.tls.flush().await?;
        }

        // Get versions cell.
        trace!("{}: waiting for versions", self.logid);
        let their_versions: msg::Versions = {
            // TODO: this could be turned into another function, I suppose.
            let mut hdr = [0u8; 5];
            self.tls.read(&mut hdr).await?;
            if hdr[0..3] != [0, 0, ChanCmd::VERSIONS.into()] {
                return Err(Error::ChanProto("Doesn't seem to be a tor relay".into()));
            }
            let msglen = u16::from_be_bytes(*array_ref![hdr, 3, 2]);
            let mut msg = vec![0; msglen as usize];
            self.tls.read_exact(&mut msg).await?;
            let mut reader = Reader::from_slice(&msg);
            reader.extract()?
        };
        trace!("{}: received {:?}", self.logid, their_versions);

        // Determine which link protocol we negotiated.
        let link_protocol = their_versions
            .best_shared_link_protocol(LINK_PROTOCOLS)
            .ok_or_else(|| Error::ChanProto("No shared link protocols".into()))?;
        trace!("{}: negotiated version {}", self.logid, link_protocol);

        // Now we can switch to using a "Framed". We can ignore the
        // AsyncRead/AsyncWrite aspects of the tls, and just treat it
        // as a stream and a sink for cells.
        let codec = ChannelCodec::new(link_protocol);
        let mut tls = futures_codec::Framed::new(self.tls, codec);

        // Read until we have the netinfo cells.
        let mut certs: Option<msg::Certs> = None;
        let mut netinfo: Option<msg::Netinfo> = None;
        let mut seen_authchallenge = false;

        // Loop: reject duplicate and unexpected cells
        trace!("{}: waiting for rest of handshake.", self.logid);
        while let Some(m) = tls.next().await {
            use msg::ChanMsg::*;
            let (_, m) = m?.into_circid_and_msg();
            trace!("{}: received a {} cell.", self.logid, m.cmd());
            match m {
                // Are these technically allowed?
                Padding(_) | VPadding(_) => (),
                // Unrecognized cells get ignored.
                Unrecognized(_) => (),
                // Clients don't care about AuthChallenge
                AuthChallenge(_) => {
                    if seen_authchallenge {
                        return Err(Error::ChanProto("Duplicate authchallenge cell".into()));
                    }
                    seen_authchallenge = true;
                }
                Certs(c) => {
                    if certs.is_some() {
                        return Err(Error::ChanProto("Duplicate certs cell".into()));
                    }
                    certs = Some(c);
                }
                Netinfo(n) => {
                    assert!(netinfo.is_none());
                    netinfo = Some(n);
                    break;
                }
                // No other cell types are allowed.
                m => {
                    return Err(Error::ChanProto(format!(
                        "Unexpected cell type {}",
                        m.cmd()
                    )))
                }
            }
        }

        // If we have certs and netinfo, we can finish authenticating.
        match (certs, netinfo) {
            (Some(_), None) => Err(Error::ChanProto("Missing netinfo or closed stream".into())),
            (None, _) => Err(Error::ChanProto("Missing certs cell".into())),
            (Some(certs_cell), Some(netinfo_cell)) => {
                trace!("{}: receieved handshake, ready to verify.", self.logid);
                Ok(UnverifiedChannel {
                    link_protocol,
                    tls,
                    certs_cell,
                    netinfo_cell,
                    logid: self.logid,
                })
            }
        }
    }
}

impl<T: AsyncRead + AsyncWrite + Send + Unpin + 'static> UnverifiedChannel<T> {
    /// Validate the certificates and keys in the relay's handshake.
    ///
    /// 'peer' is the peer that we want to make sure we're connecting to.
    ///
    /// 'peer_cert' is the x.509 certificate that the peer presented during
    /// its handshake.
    ///
    /// 'now' can be used for testing to override the current view of the
    ///
    /// This is a separate function because it's likely to be somewhat
    /// CPU-intensive.
    pub fn check<U: ChanTarget>(self, peer: &U, peer_cert: &[u8]) -> Result<VerifiedChannel<T>> {
        self.check_internal(peer, peer_cert, None)
    }

    /// Same as `check`, but allows the caller to override the time with
    /// respect to which the validitity should be checked.
    fn check_internal<U: ChanTarget>(
        self,
        peer: &U,
        peer_cert: &[u8],
        now: Option<std::time::SystemTime>,
    ) -> Result<VerifiedChannel<T>> {
        use tor_cert::CertType;
        use tor_checkable::*;
        // We need to check the following lines of authentication:
        //
        // First, to bind the ed identity to the channel.
        //    peer.ed_identity() matches the key in...
        //    IDENTITY_V_SIGNING cert, which signs...
        //    SIGNING_V_TLS_CERT cert, which signs peer_cert.
        //
        // Second, to bind the rsa identity to the ed identity:
        //    peer.rsa_identity() matches the key in...
        //    the x.509 RSA identity certificate (type 2), which signs...
        //    the RSA->Ed25519 crosscert (type 7), which signs...
        //    peer.ed_identity().

        let c = &self.certs_cell;
        let id_sk = c.parse_ed_cert(CertType::IDENTITY_V_SIGNING)?;
        let sk_tls = c.parse_ed_cert(CertType::SIGNING_V_TLS_CERT)?;

        let mut sigs = Vec::new();

        // Part 1: validate ed25519 stuff.

        // Check the identity->signing cert
        let (id_sk, id_sk_sig) = id_sk.check_key(&None)?.dangerously_split()?;
        sigs.push(&id_sk_sig);
        let id_sk = id_sk
            .check_valid_at_opt(now)
            .map_err(|_| Error::ChanProto("Certificate expired or not yet valid".into()))?;

        // Take the identity key from the identity->signing cert
        let identity_key = id_sk.signing_key().ok_or_else(|| {
            Error::ChanProto("Missing identity key in identity->signing cert".into())
        })?;

        // Take the signing key from the identity->signing cert
        let signing_key = id_sk
            .subject_key()
            .as_ed25519()
            .ok_or_else(|| Error::ChanProto("Bad key type in identity->signing cert".into()))?;

        // Now look at the signing->TLS cert and check it against the
        // peer certificate.
        let (sk_tls, sk_tls_sig) = sk_tls
            .check_key(&Some(*signing_key))? // this is a bad interface XXXX
            .dangerously_split()?;
        sigs.push(&sk_tls_sig);
        let sk_tls = sk_tls
            .check_valid_now()
            .map_err(|_| Error::ChanProto("Certificate expired or not yet valid".into()))?;

        let cert_sha256 = ll::d::Sha256::digest(peer_cert);
        if &cert_sha256[..] != sk_tls.subject_key().as_bytes() {
            return Err(Error::ChanProto(
                "Peer cert did not authenticate TLS cert".into(),
            ));
        }

        // Batch-verify the ed25519 certificates in this handshake.
        //
        // In theory we could build a list of _all_ the certificates here
        // and call pk::validate_all_sigs() instead, but that doesn't gain
        // any performance.
        if !ll::pk::ed25519::validate_batch(&sigs[..]) {
            return Err(Error::ChanProto(
                "Invalid ed25519 signature in handshake.".into(),
            ));
        }

        let ed25519_id: Ed25519Identity = identity_key.into();

        // Part 2: validate rsa stuff.

        // What is the RSA identity key, according to the X.509 certificate
        // in which it is self-signed?
        //
        // (We don't actually check this self-signed certificate, and we use
        // a kludge to extract the RSA key)
        let pkrsa = c
            .cert_body(CertType::RSA_ID_X509)
            .map(ll::util::x509_extract_rsa_subject_kludge)
            .flatten()
            .ok_or_else(|| Error::ChanProto("Couldn't find RSA identity key".into()))?;

        // Now verify the RSA identity -> Ed Identity crosscert.
        //
        // This proves that the RSA key vouches for the Ed key.  Note that
        // the Ed key does not vouch for the RSA key: The RSA key is too
        // weak.
        let rsa_cert = c
            .cert_body(CertType::RSA_ID_V_IDENTITY)
            .ok_or_else(|| Error::ChanProto("No RSA->Ed crosscert".into()))?;
        let rsa_cert = tor_cert::rsa::RSACrosscert::decode(rsa_cert)?
            .check_signature(&pkrsa)
            .map_err(|_| Error::ChanProto("Bad RSA->Ed crosscert signature".into()))?
            .check_valid_now()
            .map_err(|_| Error::ChanProto("RSA->Ed crosscert expired or invalid".into()))?;

        if !rsa_cert.subject_key_matches(identity_key) {
            return Err(Error::ChanProto(
                "RSA->Ed crosscert certifies incorrect key".into(),
            ));
        }

        let rsa_id = pkrsa.to_rsa_identity();

        trace!(
            "{}: Validated identity as {} [{}]",
            self.logid,
            ed25519_id,
            rsa_id
        );

        // Now that we've done all the verification steps on the
        // certificates, we know who we are talking to.  It's time to
        // make sure that the peer we are talking to is the peer we
        // actually wanted.
        //
        // We do this _last_, since "this is the wrong peer" is
        // usually a different situation than "this peer couldn't even
        // identify itself right."
        if *peer.ed_identity() != ed25519_id {
            return Err(Error::ChanProto("Peer ed25519 id not as expected".into()));
        }

        if *peer.rsa_identity() != rsa_id {
            return Err(Error::ChanProto("Peer RSA id not as expected".into()));
        }

        Ok(VerifiedChannel {
            link_protocol: self.link_protocol,
            tls: self.tls,
            logid: self.logid,
            ed25519_id,
            rsa_id,
        })
    }
}

impl<T: AsyncRead + AsyncWrite + Send + Unpin + 'static> VerifiedChannel<T> {
    /// Send a 'Netinfo' message to the relay to finish the handshake,
    /// and create an open channel and reactor.
    ///
    /// The channel is used to send cells, and to create outgoing circuits.
    /// The reactor is used to route incoming messages to their appropriate
    /// circuit.
    pub async fn finish(
        mut self,
        peer_addr: &net::IpAddr,
    ) -> Result<(super::Channel, super::reactor::Reactor<T>)> {
        trace!("{}: Sending netinfo cell.", self.logid);
        let netinfo = msg::Netinfo::for_client(*peer_addr);
        self.tls.send(netinfo.into()).await?;

        debug!(
            "{}: Completed handshake with {} [{}]",
            self.logid, self.ed25519_id, self.rsa_id
        );
        Ok(super::Channel::new(
            self.link_protocol,
            self.tls,
            self.logid,
            self.ed25519_id,
            self.rsa_id,
        ))
    }
}

#[cfg(test)]
pub(super) mod test {
    use futures_await_test::async_test;
    use hex_literal::hex;

    use super::*;
    use crate::channel::codec::test::MsgBuf;
    use crate::Result;
    use tor_cell::chancell::msg;

    const VERSIONS: &[u8] = &hex!("0000 07 0006 0003 0004 0005");
    // no certificates in this cell, but connect() doesn't care.
    const NOCERTS: &[u8] = &hex!("00000000 81 0001 00");
    const NETINFO_PREFIX: &[u8] = &hex!(
        "00000000 08 085F9067F7
         04 04 7f 00 00 02
         01
         04 04 7f 00 00 03"
    );
    const AUTHCHALLENGE: &[u8] = &hex!(
        "00000000 82 0026
         FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
         FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
         0002 0003 00ff"
    );

    const VPADDING: &[u8] = &hex!("00000000 80 0003 FF FF FF");

    fn add_padded(buf: &mut Vec<u8>, cell: &[u8]) {
        let len_prev = buf.len();
        buf.extend_from_slice(cell);
        buf.resize(len_prev + 514, 0);
    }
    fn add_netinfo(buf: &mut Vec<u8>) {
        add_padded(buf, NETINFO_PREFIX);
    }

    #[async_test]
    async fn connect_ok() -> Result<()> {
        let mut buf = Vec::new();
        // versions cell
        buf.extend_from_slice(VERSIONS);
        // certs cell -- no certs in it, but this function doesn't care.
        buf.extend_from_slice(NOCERTS);
        // netinfo cell -- quite minimal.
        add_netinfo(&mut buf);
        let mb = MsgBuf::new(&buf[..]);
        let handshake = OutboundClientHandshake::new(mb, None);
        let unverified = handshake.connect().await?;

        assert_eq!(unverified.link_protocol, 4);

        // Try again with an authchallenge cell and some padding.
        let mut buf = Vec::new();
        buf.extend_from_slice(VERSIONS);
        buf.extend_from_slice(NOCERTS);
        buf.extend_from_slice(VPADDING);
        buf.extend_from_slice(AUTHCHALLENGE);
        buf.extend_from_slice(VPADDING);
        add_netinfo(&mut buf);
        let mb = MsgBuf::new(&buf[..]);
        let handshake = OutboundClientHandshake::new(mb, None);
        let _unverified = handshake.connect().await?;

        Ok(())
    }

    async fn connect_err<T: Into<Vec<u8>>>(input: T) -> Error {
        let mb = MsgBuf::new(input);
        let handshake = OutboundClientHandshake::new(mb, None);
        handshake.connect().await.err().unwrap()
    }

    #[async_test]
    async fn connect_badver() {
        let err = connect_err(&b"HTTP://"[..]).await;
        assert!(matches!(err, Error::ChanProto(_)));
        assert_eq!(
            format!("{}", err),
            "channel protocol violation: Doesn't seem to be a tor relay"
        );

        let err = connect_err(&hex!("0000 07 0004 1234 ffff")[..]).await;
        assert!(matches!(err, Error::ChanProto(_)));
        assert_eq!(
            format!("{}", err),
            "channel protocol violation: No shared link protocols"
        );
    }

    #[async_test]
    async fn connect_cellparse() {
        let mut buf = Vec::new();
        buf.extend_from_slice(VERSIONS);
        // Here's a certs cell that will fail.
        buf.extend_from_slice(&hex!("00000000 81 0001 01")[..]);
        let err = connect_err(buf).await;
        assert!(matches!(
            err,
            Error::CellErr(tor_cell::Error::BytesErr(tor_bytes::Error::Truncated))
        ));
    }

    #[async_test]
    async fn connect_duplicates() {
        let mut buf = Vec::new();
        buf.extend_from_slice(VERSIONS);
        buf.extend_from_slice(NOCERTS);
        buf.extend_from_slice(NOCERTS);
        add_netinfo(&mut buf);
        let err = connect_err(buf).await;
        assert!(matches!(err, Error::ChanProto(_)));
        assert_eq!(
            format!("{}", err),
            "channel protocol violation: Duplicate certs cell"
        );

        let mut buf = Vec::new();
        buf.extend_from_slice(VERSIONS);
        buf.extend_from_slice(NOCERTS);
        buf.extend_from_slice(AUTHCHALLENGE);
        buf.extend_from_slice(AUTHCHALLENGE);
        add_netinfo(&mut buf);
        let err = connect_err(buf).await;
        assert!(matches!(err, Error::ChanProto(_)));
        assert_eq!(
            format!("{}", err),
            "channel protocol violation: Duplicate authchallenge cell"
        );
    }

    #[async_test]
    async fn connect_missing_certs() {
        let mut buf = Vec::new();
        buf.extend_from_slice(VERSIONS);
        add_netinfo(&mut buf);
        let err = connect_err(buf).await;
        assert!(matches!(err, Error::ChanProto(_)));
        assert_eq!(
            format!("{}", err),
            "channel protocol violation: Missing certs cell"
        );
    }

    #[async_test]
    async fn connect_misplaced_cell() {
        let mut buf = Vec::new();
        buf.extend_from_slice(VERSIONS);
        // here's a create cell.
        add_padded(&mut buf, &hex!("00000001 01")[..]);
        let err = connect_err(buf).await;
        assert!(matches!(err, Error::ChanProto(_)));
        assert_eq!(
            format!("{}", err),
            "channel protocol violation: Unexpected cell type CREATE"
        );
    }

    // not used yet
    #[allow(unused)]
    fn make_unverified(certs: msg::Certs) -> UnverifiedChannel<MsgBuf> {
        let localhost = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
        let netinfo_cell = msg::Netinfo::for_client(localhost);
        UnverifiedChannel {
            link_protocol: 4,
            tls: futures_codec::Framed::new(MsgBuf::new(&b""[..]), ChannelCodec::new(4)),
            certs_cell: certs,
            netinfo_cell,
            logid: LogId::new(),
        }
    }

    struct DummyChanTarget {
        ed: Ed25519Identity,
        rsa: RSAIdentity,
    }
    impl ChanTarget for DummyChanTarget {
        fn addrs(&self) -> &[std::net::SocketAddr] {
            &[]
        }
        fn ed_identity(&self) -> &Ed25519Identity {
            &self.ed
        }
        fn rsa_identity(&self) -> &RSAIdentity {
            &self.rsa
        }
    }

    // not used yet
    #[allow(unused)]
    fn certs_test(
        certs: msg::Certs,
        peer_ed: &[u8],
        peer_rsa: &[u8],
        peer_cert: &[u8],
    ) -> Result<VerifiedChannel<MsgBuf>> {
        let unver = make_unverified(certs);
        // XXXXM3 fix this naming inconsistency
        let ed = Ed25519Identity::from_slice(peer_ed).unwrap();
        let rsa = RSAIdentity::from_bytes(peer_rsa).unwrap();
        let chan = DummyChanTarget { ed, rsa };
        unver.check(&chan, peer_cert)
    }
}
