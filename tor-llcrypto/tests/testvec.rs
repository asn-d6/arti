use digest::{self, Digest, ExtendableOutput};
use hex_literal::hex;
use stream_cipher::*;
use tor_llcrypto as ll;

#[test]
fn tv_curve25519() {
    use ll::pk::curve25519::*;

    // Test vectors from RFC 7748
    let s1 = hex!(
        "a546e36bf0527c9d3b16154b82465edd
                   62144c0ac1fc5a18506a2244ba449ac4"
    );
    let u1 = hex!(
        "e6db6867583030db3594c1a424b15f7c
                   726624ec26b3353b10a903a6d0ab1c4c"
    );
    let o1 = hex!(
        "c3da55379de9c6908e94ea4df28d084f
                   32eccf03491c71f754b4075577a28552"
    );

    let s1 = StaticSecret::from(s1);
    let u1 = PublicKey::from(u1);
    let ss = s1.diffie_hellman(&u1);
    assert_eq!(ss.as_bytes(), &o1);

    let s2 = hex!(
        "4b66e9d4d1b4673c5ad22691957d6af5
                   c11b6421e0ea01d42ca4169e7918ba0d"
    );
    let u2 = hex!(
        "e5210f12786811d3f4b7959d0538ae2c
                   31dbe7106fc03c3efc4cd549c715a493"
    );
    let o2 = hex!(
        "95cbde9476e8907d7aade45cb4b873f8
                   8b595a68799fa152e6f8f7647aac7957"
    );

    let s2 = StaticSecret::from(s2);
    let u2 = PublicKey::from(u2);
    let ss = s2.diffie_hellman(&u2);
    assert_eq!(ss.as_bytes(), &o2);
}

#[test]
fn tv_ed25519() {
    use ll::pk::ed25519::*;
    use signature::{Signer, Verifier};
    // Test vectors from RFC 8032.

    // TEST 1
    let sk = SecretKey::from_bytes(&hex!(
        "9d61b19deffd5a60ba844af492ec2cc4
               4449c5697b326919703bac031cae7f60"
    ))
    .expect("Bad value");

    let pk: PublicKey = (&sk).into();

    assert_eq!(
        pk.as_bytes(),
        &hex!(
            "d75a980182b10ab7d54bfed3c964073a
                      0ee172f3daa62325af021a68f707511a"
        )
    );

    let kp = Keypair {
        public: pk,
        secret: sk,
    };
    let sig = kp.sign(&b""[..]);
    assert_eq!(
        &sig.to_bytes()[..],
        &hex!(
            "e5564300c360ac729086e2cc806e828a
                      84877f1eb8e5d974d873e06522490155
                      5fb8821590a33bacc61e39701cf9b46b
                      d25bf5f0595bbe24655141438e7a100b"
        )[..]
    );

    assert!(kp.public.verify(&b""[..], &sig).is_ok());

    // TEST 3
    let sk = SecretKey::from_bytes(&hex!(
        "c5aa8df43f9f837bedb7442f31dcb7b1
               66d38535076f094b85ce3a2e0b4458f7"
    ))
    .expect("Bad value");

    let pk: PublicKey = (&sk).into();

    assert_eq!(
        pk.as_bytes(),
        &hex!(
            "fc51cd8e6218a1a38da47ed00230f058
                      0816ed13ba3303ac5deb911548908025"
        )
    );

    let kp = Keypair {
        public: pk,
        secret: sk,
    };
    let sig = kp.sign(&hex!("af82"));
    assert_eq!(
        &sig.to_bytes()[..],
        &hex!(
            "6291d657deec24024827e69c3abe01a3
                      0ce548a284743a445e3680d7db5ac3ac
                      18ff9b538d16f290ae67f760984dc659
                      4a7c15e9716ed28dc027beceea1ec40a"
        )[..]
    );

    assert!(kp.public.verify(&hex!("af82"), &sig).is_ok());

    assert!(kp.public.verify(&hex!(""), &sig).is_err());
}

#[test]
fn tv_aes128_ctr() {
    // From NIST Special Publication 800-38A.
    // https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38a.pdf
    use ll::cipher::aes::Aes128Ctr;

    let k1 = hex!("2b7e151628aed2a6abf7158809cf4f3c").into();
    let ctr1 = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").into();

    let mut cipher = Aes128Ctr::new(&k1, &ctr1);
    let mut data = hex!(
        "6bc1bee22e409f96e93d7e117393172a
         ae2d8a571e03ac9c9eb76fac45af8e51
         30c81c46a35ce411e5fbc1191a0a52ef
         f69f2445df4f9b17ad2b417be66c3710"
    );

    cipher.encrypt(&mut data);

    assert_eq!(
        &data[..],
        &hex!(
            "874d6191b620e3261bef6864990db6ce
             9806f66b7970fdff8617187bb9fffdff
             5ae4df3edbd5d35e5b4f09020db03eab
             1e031dda2fbe03d1792170a0f3009cee"
        )[..]
    );
}

#[test]
fn tv_aes256_ctr() {
    // From NIST Special Publication 800-38A.
    // https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38a.pdf

    use ll::cipher::aes::Aes256Ctr;

    let k1 = hex!(
        "603deb1015ca71be2b73aef0857d7781
         1f352c073b6108d72d9810a30914dff4"
    )
    .into();
    let ctr1 = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").into();

    let mut cipher = Aes256Ctr::new(&k1, &ctr1);
    let mut data = hex!(
        "6bc1bee22e409f96e93d7e117393172a
         ae2d8a571e03ac9c9eb76fac45af8e51
         30c81c46a35ce411e5fbc1191a0a52ef
         f69f2445df4f9b17ad2b417be66c3710"
    );

    cipher.encrypt(&mut data);

    assert_eq!(
        &data[..],
        &hex!(
            "601ec313775789a5b7a7f504bbf3d228
             f443e3ca4d62b59aca84e990cacaf5c5
             2b0930daa23de94ce87017ba2d84988d
             dfc9c58db67aada613c2dd08457941a6"
        )[..]
    );
}

#[test]
fn tv_sha1() {
    // From RFC 3174, extracted from the example C code.
    use ll::d::Sha1;

    fn run_test(inp: &[u8], repeatcount: usize, expect: &[u8]) {
        let mut d = Sha1::new();
        for _ in 0..repeatcount {
            d.update(inp);
        }
        let res = d.finalize();
        assert_eq!(&res[..], &expect[..]);
    }

    run_test(
        b"abc",
        1,
        &hex!(
            "A9 99 3E 36 47 06 81 6A BA 3E
             25 71 78 50 C2 6C 9C D0 D8 9D"
        )[..],
    );
    run_test(
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        1,
        &hex!(
            "84 98 3E 44 1C 3B D2 6E BA AE
             4A A1 F9 51 29 E5 E5 46 70 F1"
        )[..],
    );
    run_test(
        b"a",
        1000000,
        &hex!(
            "34 AA 97 3C D4 C4 DA A4 F6 1E
             EB 2B DB AD 27 31 65 34 01 6F"
        )[..],
    );
    run_test(
        b"0123456701234567012345670123456701234567012345670123456701234567",
        10,
        &hex!(
            "DE A3 56 A2 CD DD 90 C7 A7 EC
             ED C5 EB B5 63 93 4F 46 04 52"
        )[..],
    );
}

#[test]
fn tv_sha256() {
    // From https://csrc.nist.gov/csrc/media/projects/cryptographic-standards-and-guidelines/documents/examples/sha_all.pdf

    let d = ll::d::Sha256::digest(b"abc");
    assert_eq!(
        &d[..],
        hex!(
            "BA7816BF 8F01CFEA 414140DE 5DAE2223
             B00361A3 96177A9C B410FF61 F20015AD"
        )
    );

    let d = ll::d::Sha256::digest(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
    assert_eq!(
        &d[..],
        hex!(
            "248D6A61 D20638B8 E5C02693 0C3E6039
             A33CE459 64FF2167 F6ECEDD4 19DB06C1"
        )
    );
}

#[test]
fn tv_sha512() {
    // From https://csrc.nist.gov/csrc/media/projects/cryptographic-standards-and-guidelines/documents/examples/sha_all.pdf

    let d = ll::d::Sha512::digest(b"abc");
    assert_eq!(
        &d[..],
        &hex!(
            "DDAF35A1 93617ABA CC417349 AE204131 12E6FA4E 89A97EA2
             0A9EEEE6 4B55D39A 2192992A 274FC1A8 36BA3C23 A3FEEBBD
             454D4423 643CE80E 2A9AC94F A54CA49F"
        )[..]
    );

    let d = ll::d::Sha512::digest(b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu");
    assert_eq!(
        &d[..],
        &hex!(
            "8E959B75 DAE313DA 8CF4F728 14FC143F 8F7779C6 EB9F7FA1
             7299AEAD B6889018 501D289E 4900F7E4 331B99DE C4B5433A
             C7D329EE B6DD2654 5E96E55B 874BE909"
        )[..]
    );
}

#[test]
fn tv_sha3_256() {
    // From https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/sha3/sha-3bytetestvectors.zip

    let d = ll::d::Sha3_256::digest(b"");
    assert_eq!(
        &d[..],
        &hex!("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a")
    );

    let d = ll::d::Sha3_256::digest(&hex!("e9"));
    assert_eq!(
        &d[..],
        &hex!("f0d04dd1e6cfc29a4460d521796852f25d9ef8d28b44ee91ff5b759d72c1e6d6")
    );

    let d = ll::d::Sha3_256::digest(&hex!("d16d978dfbaecf2c8a04090f6eebdb421a5a711137a6"));
    assert_eq!(
        &d[..],
        &hex!("7f497913318defdc60c924b3704b65ada7ca3ba203f23fb918c6fb03d4b0c0da")
    );

    let d = ll::d::Sha3_256::digest(&hex!(
        "3341ca020d4835838b0d6c8f93aaaebb7af60730d208c85283f6369f1ee27fd96d38f2674f
316ef9c29c1b6b42dd59ec5236f65f5845a401adceaa4cf5bbd91cac61c21102052634e99faedd6c
dddcd4426b42b6a372f29a5a5f35f51ce580bb1845a3c7cfcd447d269e8caeb9b320bb731f53fe5c
969a65b12f40603a685afed86bfe53"
    ));
    assert_eq!(
        &d[..],
        &hex!("6c3e93f2b49f493344cc3eb1e9454f79363032beee2f7ea65b3d994b5cae438f")
    );
}

fn xof_helper<X: ExtendableOutput + digest::Update>(mut x: X, input: &[u8], output: &[u8]) {
    use digest::XofReader;
    x.update(input);
    let mut r = x.finalize_xof();
    let mut buf = vec![0; output.len()];
    r.read(&mut buf);
    assert_eq!(&buf[..], &output[..]);
}

#[test]
fn tv_shake128() {
    // From https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/sha3/shakebytetestvectors.zip

    xof_helper(
        ll::d::Shake128::default(),
        &hex!("ca12721a7a44544d9518aa0d4e407529"),
        &hex!("25904657e9903ce960b56bcc42a4e9ff7b33"),
    );

    xof_helper(
        ll::d::Shake128::default(),
        &hex!("981f4788c57eb8d064805357024d3128"),
        &hex!("4c206447e85a2cbd4fab891ef3140806a32a89"),
    );

    xof_helper(
        ll::d::Shake128::default(),
        &hex!("e118cecce029b40f7883805eb19d1c09"),
        &hex!(
            "6e8f5de5c92a474a1f96bf89798a11c96637c05e6f1d21940c07
                      783b2d5da11c8f592446c12189eabfc9be2561855fa7c7c1b7fe"
        ),
    );

    xof_helper(
        ll::d::Shake128::default(),
        &hex!("c60a221c975e14bf835827c1103a2906"),
        &hex!(
            "0db7f7196eee8dd6994a16ded19cb09f05f89ccd2464333df2c0
                      17c6ca041fa0d54a4832a74ce86ce9b41d8e523e66ce6ef9df7c
                      20aa70e0ac00f54eb072a472ef46cf2a933df0d5f9fafab6388a
                      206f6bd1df50b0836500c758c557c8ac965733fdaaa59f5ed661
                      a1bda61e2952886a60f9568157e3d72e49b6e061fc08f3f1caf1
                      59e8eff77ea5221565d2"
        ),
    );

    xof_helper(
        ll::d::Shake128::default(),
        &hex!(
            "c521710a951c7f1fda05ddf7b78366976ce6f8ee7abbbf0c089d
                      b690854e6a5f8f06029c130a7cd4b68139787483bc918774af"
        ),
        &hex!("65fa398b3a99fa2c9a122f46a4ac4896"),
    );
}

#[test]
fn tv_shake256() {
    // From https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/sha3/shakebytetestvectors.zip

    xof_helper(
        ll::d::Shake256::default(),
        &hex!(
            "c61a9188812ae73994bc0d6d4021e31bf124dc72669749111232da
                      7ac29e61c4"
        ),
        &hex!("23ce"),
    );

    xof_helper(
        ll::d::Shake256::default(),
        &hex!(
            "76891a7bcc6c04490035b743152f64a8dd2ea18ab472b8d36ecf45
                      858d0b0046"
        ),
        &hex!(
            "e8447df87d01beeb724c9a2a38ab00fcc24e9bd17860e673b02122
                      2d621a7810e5d3"
        ),
    );

    xof_helper(
        ll::d::Shake256::default(),
        &hex!(
            "7d9312ffe94845ac51056c63eb3bff4a94626aafb7470ff86fa88f
                      d8f0fe45c9"
        ),
        &hex!(
            "de489392796fd3b530c506e482936afcfe6b72dcf7e9def0549538
                      42ff19076908c8a1d6a4e7639e0fdbfa1b5201095051aac3e39977
                      79e588377eac979313e39c3721dc9f912cf7fdf1a9038cbaba8e9f
                      3d95951a5d819bffd0b080319fcd12da0516baf54b779e79e437d3
                      ec565c64eb5752825f54050f93"
        ),
    );

    xof_helper(
        ll::d::Shake256::default(),
        &hex!(
            "1b6facbbeb3206ff68214b3ad5c0fcbcd37ae9e2d84347dfde7c02
                      bc5817559e6afb974859aa58e04121acf60600c7c28ceacaad2b2f
                      dd145da87e48bae318d92780d8144adbbcca41eced53936b4ed366
                      3755bcf3f81a943803adf9ec7fade2b8c61627a40e5b44d0"
        ),
        &hex!(
            "1d8599e06e505fea435eb7699b1effdc3fe76864abce4ea446824f
                      ff7869ad26"
        ),
    );
}
