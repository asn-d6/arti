//! Code to read directory caches generated by C Tor from disk.
//!
//! Tor writes its directory info into a set of mostly unstructured files:
//! * `cached-microdesc-consensus`: A validated microdecriptor consensus.
//! * `cached-certs`: A list of authority certificates, concatenated.
//! * `cached-microdescs{,.new}`: A list of microdescriptors, annotated with
//!   their last-listed times, and concatenated.

use log::{debug, info, warn};
use tor_checkable::{ExternallySigned, SelfSigned, Timebound};
use tor_netdoc::doc::authcert::AuthCert;
use tor_netdoc::doc::microdesc::{AnnotatedMicrodesc, Microdesc, MicrodescReader};
use tor_netdoc::doc::netstatus::MdConsensus;
use tor_netdoc::AllowAnnotations;

use std::path::{Path, PathBuf};
use std::time;

use super::InputString;
use crate::{Authority, Error, MdReceiver, PartialNetDir, Result};

/// A location on disk where we can try to find cached directory
/// information.
pub(crate) struct LegacyStore {
    /// Location of the cache.
    dir: PathBuf,
}

impl LegacyStore {
    /// Create a new LegacyStore from a provided `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        LegacyStore {
            dir: path.as_ref().into(),
        }
    }

    /// Return a path `relpath` within the configured directory for
    /// this store.  Note that the caller is responsible for making
    /// sure there are no ".." entries in this.
    fn relative_path<P: AsRef<Path>>(&self, relpath: P) -> PathBuf {
        let mut pb = self.dir.to_path_buf();
        pb.push(relpath);
        pb
    }

    /// Try loading the latest consensus, as a string.
    fn latest_consensus(&self) -> Result<InputString> {
        let p = self.relative_path("cached-microdesc-consensus");
        Ok(InputString::load(p)?)
    }

    /// Try loading the microdescriptors, as an iterator of strings.
    ///
    /// Each string will contain zero or more annotated
    /// microdescriptors.
    fn microdescs(&self) -> impl Iterator<Item = Result<InputString>> {
        // impl Iterator<Item=Result<InputString,Self::Error>> {
        let paths = vec![
            self.relative_path("cached-microdescs"),
            self.relative_path("cached-microdescs.new"),
        ];
        Box::new(paths.into_iter().map(InputString::load))
    }

    /// Try loading the microdescriptors, as an iterator of strings.
    ///
    /// Each string will contain zero or authcerts.
    fn authcerts(&self) -> impl Iterator<Item = Result<InputString>> {
        // impl Iterator<Item=Result<InputString,Self::Error>> {
        let paths = vec![self.relative_path("cached-certs")];
        Box::new(paths.into_iter().map(InputString::load))
    }

    /// Helper: Load the authority certificates from a store.
    ///
    /// Only loads the certificates that match identity keys for
    /// authorities that we believe in.
    ///
    /// Warn about invalid certs, but don't give an error unless there
    /// is a complete failure.
    fn load_certs(&self, authorities: &[Authority]) -> Result<Vec<AuthCert>> {
        let mut res = Vec::new();
        for input in self.authcerts().filter_map(Result::ok) {
            let text = input.as_str()?;

            for cert in AuthCert::parse_multiple(text) {
                let r: Result<_> = (|| {
                    let cert = cert?.check_signature()?.check_valid_now()?;

                    let found = authorities.iter().any(|a| a.matches_cert(&cert));
                    if !found {
                        return Err(Error::Unwanted("no such authority").into());
                    }
                    Ok(cert)
                })();

                match r {
                    Err(e) => warn!("unwanted certificate: {}", e),
                    Ok(cert) => {
                        debug!(
                            "adding cert for {} (SK={})",
                            cert.id_fingerprint(),
                            cert.sk_fingerprint()
                        );
                        res.push(cert);
                    }
                }
            }
        }

        info!("Loaded {} certs", res.len());
        Ok(res)
    }

    /// Read the consensus from a provided store, and check it
    /// with a list of authcerts.
    fn load_consensus(&self, certs: &[AuthCert], authorities: &[Authority]) -> Result<MdConsensus> {
        let input = self.latest_consensus()?;
        let text = input.as_str()?;
        let (_, _, consensus) = MdConsensus::parse(text)?;
        let consensus = consensus
            .extend_tolerance(time::Duration::new(86400, 0))
            .check_valid_now()?
            .set_n_authorities(authorities.len() as u16)
            .check_signature(certs)?;

        Ok(consensus)
    }

    /// Read a list of microdescriptors from a provided store.
    ///
    /// Warn about invalid microdescs, but don't give an error unless there
    /// is a complete failure.
    fn load_mds(&self) -> Result<Vec<Microdesc>> {
        let mut res = Vec::new();
        for input in self.microdescs().filter_map(Result::ok) {
            let text = input.as_str()?;
            for annotated in MicrodescReader::new(&text, AllowAnnotations::AnnotationsAllowed) {
                let r = annotated.map(AnnotatedMicrodesc::into_microdesc);
                match r {
                    Err(e) => warn!("bad microdesc: {}", e),
                    Ok(md) => res.push(md),
                }
            }
        }
        Ok(res)
    }

    /// Load and validate an entire network directory from a legacy Tor cache.
    pub fn load_legacy(&self, authorities: &[Authority]) -> Result<PartialNetDir> {
        let certs = self.load_certs(authorities)?;
        let consensus = self.load_consensus(&certs, authorities)?;
        info!("Loaded consensus");
        let mut partial = PartialNetDir::new(consensus, None); // TODO: allow parameter override.

        let mds = self.load_mds()?;
        info!("Loaded {} microdescriptors", mds.len());
        let mut n_added = 0_usize;
        for md in mds {
            if partial.add_microdesc(md) {
                n_added += 1;
            }
        }
        info!("Used {} microdescriptors", n_added);

        Ok(partial)
    }
}
