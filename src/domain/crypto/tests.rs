use super::*;

/// The smallest costs Argon2id accepts, so the tests stay fast.
const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
const PASSWORD: &str = "lamp not a mirror 42";
const PLAINTEXT: &[u8] = b"a private thought, sealed";
const AAD: &[u8] = b"header bytes";

fn fixed() -> SealParams {
    SealParams { kdf: FAST, salt: [7; SALT_LEN], nonce: [9; NONCE_LEN] }
}

#[test]
fn default_seal_round_trips() {
    let envelope = seal(PLAINTEXT, PASSWORD).expect("sealed");
    assert_eq!(envelope.params.kdf, KdfParams::DEFAULT);
    assert_eq!(envelope.ciphertext.len(), PLAINTEXT.len() + TAG_LEN);
    assert_eq!(open(&envelope, PASSWORD).expect("opened").as_slice(), PLAINTEXT);
}

#[test]
fn associated_data_must_match() {
    let envelope = seal_with(PLAINTEXT, PASSWORD, fixed(), AAD).expect("sealed");
    let opened = open_with(&envelope.params, &envelope.ciphertext, PASSWORD, AAD).expect("opened");
    assert_eq!(opened.as_slice(), PLAINTEXT);
    assert_eq!(open_with(&envelope.params, &envelope.ciphertext, PASSWORD, b"other header").err(), Some(CryptoError::Rejected));
    assert_eq!(open(&envelope, PASSWORD).err(), Some(CryptoError::Rejected), "sealed with associated data");
}

#[test]
fn wrong_password_and_tampering_are_rejected() {
    let envelope = seal_with(PLAINTEXT, PASSWORD, fixed(), &[]).expect("sealed");
    assert_eq!(open(&envelope, "lamp not a mirror 43").err(), Some(CryptoError::Rejected));
    assert_eq!(open(&envelope, "").err(), Some(CryptoError::Rejected));
    for at in 0..envelope.ciphertext.len() {
        let mut changed = envelope.clone();
        changed.ciphertext[at] ^= 0x01;
        assert_eq!(open(&changed, PASSWORD).err(), Some(CryptoError::Rejected), "byte {at}");
    }
    let mut other_salt = envelope.clone();
    other_salt.params.salt[0] ^= 0x01;
    assert_eq!(open(&other_salt, PASSWORD).err(), Some(CryptoError::Rejected));
}

#[test]
fn fixed_parameters_are_deterministic_and_hide_the_plaintext() {
    let make = || seal_with(PLAINTEXT, PASSWORD, fixed(), AAD).expect("sealed");
    assert_eq!(make(), make());
    assert!(!make().ciphertext.windows(PLAINTEXT.len()).any(|window| window == PLAINTEXT));
}

#[test]
fn every_seal_gets_a_fresh_salt_and_nonce() {
    let (a, b) = (SealParams::fresh(FAST), SealParams::fresh(FAST));
    assert_ne!(a.salt, b.salt);
    assert_ne!(a.nonce, b.nonce);
}

#[test]
fn absurd_costs_are_refused_before_deriving() {
    let envelope = seal_with(PLAINTEXT, PASSWORD, fixed(), &[]).expect("sealed");
    for kdf in [
        KdfParams { memory_kib: MAX_MEMORY_KIB + 1, ..FAST },
        KdfParams { iterations: 0, ..FAST },
        KdfParams { iterations: MAX_ITERATIONS + 1, ..FAST },
        KdfParams { lanes: MAX_LANES + 1, ..FAST },
    ] {
        assert!(!kdf.is_sane());
        let params = SealParams { kdf, ..envelope.params };
        assert_eq!(open_with(&params, &envelope.ciphertext, PASSWORD, &[]).err(), Some(CryptoError::KeyDerivation));
    }
    assert!(KdfParams::DEFAULT.is_sane() && FAST.is_sane());
}

#[test]
fn costs_argon2_rejects_fail_as_key_derivation() {
    let params = SealParams { kdf: KdfParams { memory_kib: 1, ..FAST }, ..fixed() };
    assert_eq!(seal_with(PLAINTEXT, PASSWORD, params, &[]).err(), Some(CryptoError::KeyDerivation));
}
