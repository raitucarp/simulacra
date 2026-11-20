use super::*;

/// The smallest costs Argon2id accepts, so the tests stay fast (real snapshots use
/// [`KdfParams::DEFAULT`]).
const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
const CREATED_AT: i64 = 1_795_000_000;
const PASSPHRASE: &str = "lamp not a mirror 42";
const PAYLOAD: &[u8] = b"SQLite format 3\0 every post, document and journal entry";

fn sealed() -> Vec<u8> {
    seal(PAYLOAD, PASSPHRASE, FAST, CREATED_AT).expect("sealed")
}

#[test]
fn round_trip_returns_the_payload_and_header() {
    let file = sealed();
    assert_eq!(file.len(), HEADER_LEN + PAYLOAD.len() + 16);
    let opened = open(&file, PASSPHRASE).expect("opened");
    assert_eq!(opened.plaintext.as_slice(), PAYLOAD);
    assert_eq!((opened.header.version, opened.header.kdf, opened.header.created_at), (FORMAT_VERSION, FAST, CREATED_AT));
    assert_eq!(inspect(&file).expect("header"), opened.header);
}

#[test]
fn round_trip_with_default_costs() {
    let file = seal(PAYLOAD, PASSPHRASE, KdfParams::DEFAULT, CREATED_AT).expect("sealed");
    assert_eq!(open(&file, PASSPHRASE).expect("opened").plaintext.as_slice(), PAYLOAD);
}

#[test]
fn wrong_passphrase_is_rejected() {
    let file = sealed();
    assert_eq!(open(&file, "lamp not a mirror 43").err(), Some(SnapshotError::WrongPassphrase));
    assert_eq!(open(&file, "").err(), Some(SnapshotError::WrongPassphrase));
}

#[test]
fn every_tampered_byte_is_detected() {
    let file = sealed();
    // The magic and version bytes fail with their own errors; every other header byte is
    // associated data, every payload byte is sealed.
    for at in MAGIC.len() + 2..file.len() {
        let mut changed = file.clone();
        changed[at] ^= 0x01;
        assert!(open(&changed, PASSPHRASE).is_err(), "byte {at} changed unnoticed");
    }
}

#[test]
fn a_tampered_payload_byte_reads_as_a_wrong_passphrase() {
    let mut file = sealed();
    let last = file.len() - 1;
    file[last] ^= 0x80;
    assert_eq!(open(&file, PASSPHRASE).err(), Some(SnapshotError::WrongPassphrase));
}

#[test]
fn truncated_files_are_rejected() {
    let file = sealed();
    for len in [3, HEADER_LEN - 1, HEADER_LEN, file.len() - 1] {
        assert_eq!(open(&file[..len], PASSPHRASE).err(), Some(SnapshotError::Truncated), "cut at {len}");
    }
    assert_eq!(open(&[], PASSPHRASE).err(), Some(SnapshotError::NotASnapshot));
}

#[test]
fn trailing_bytes_are_corrupt() {
    let mut file = sealed();
    file.push(0);
    assert_eq!(open(&file, PASSPHRASE).err(), Some(SnapshotError::Corrupt));
}

#[test]
fn unknown_version_is_rejected_before_key_derivation() {
    let mut file = sealed();
    file[MAGIC.len()..MAGIC.len() + 2].copy_from_slice(&2u16.to_le_bytes());
    assert_eq!(open(&file, PASSPHRASE).err(), Some(SnapshotError::UnknownVersion(2)));
    assert_eq!(inspect(&file).err(), Some(SnapshotError::UnknownVersion(2)));
}

#[test]
fn other_files_are_not_snapshots() {
    assert_eq!(open(b"PK\x03\x04 a zip archive, not a snapshot", PASSPHRASE).err(), Some(SnapshotError::NotASnapshot));
}

#[test]
fn absurd_costs_are_corrupt_not_attempted() {
    let header = Header { version: FORMAT_VERSION, kdf: KdfParams { memory_kib: u32::MAX, iterations: 1, lanes: 1 }, salt: [0; SALT_LEN], nonce: [0; NONCE_LEN], created_at: 0, payload_len: 0 };
    assert_eq!(open(&header.encode(), PASSPHRASE).err(), Some(SnapshotError::Corrupt));
}

#[test]
fn every_snapshot_gets_a_fresh_salt_and_nonce() {
    let (a, b) = (inspect(&sealed()).unwrap(), inspect(&sealed()).unwrap());
    assert_ne!(a.salt, b.salt);
    assert_ne!(a.nonce, b.nonce);
}

#[test]
fn fixed_salt_and_nonce_are_deterministic() {
    let make = || seal_with(PAYLOAD, PASSPHRASE, FAST, CREATED_AT, [7; SALT_LEN], [9; NONCE_LEN]).unwrap();
    assert_eq!(make(), make());
    assert!(!make().windows(PAYLOAD.len()).any(|window| window == PAYLOAD), "the payload is not stored in the clear");
}

#[test]
fn strength_hint_grows_with_effort() {
    assert_eq!(strength(""), Strength::Empty);
    assert_eq!(strength("password"), Strength::Weak);
    assert_eq!(strength("12345678"), Strength::Weak);
    assert_eq!(strength("aaaaaaaaaaaaaaaaaaaaaaaa"), Strength::Weak);
    assert_eq!(strength("correct horse battery staple"), Strength::Fair);
    assert_eq!(strength("correct horse battery staple violet anchor"), Strength::Strong);
    assert_eq!(strength("q7#Vt9!mZ2&rLx4@pW8s"), Strength::VeryStrong);
    assert!(Strength::Weak.level() < Strength::VeryStrong.level());
}

#[test]
fn minimum_length_counts_characters() {
    assert!(!meets_minimum("short"));
    assert!(meets_minimum("ありがとうございます"));
}

#[test]
fn error_keys_are_distinct() {
    let errors = [SnapshotError::NotASnapshot, SnapshotError::UnknownVersion(9), SnapshotError::Truncated, SnapshotError::Corrupt, SnapshotError::WrongPassphrase, SnapshotError::KeyDerivation];
    let keys: std::collections::HashSet<&str> = errors.iter().map(|e| e.key()).collect();
    assert_eq!(keys.len(), errors.len());
}

/// `seal_with(PAYLOAD, PASSPHRASE, FAST, CREATED_AT, [7; 16], [9; 24])` as written by the
/// snapshot module's own Argon2id + XChaCha20-Poly1305 code before it moved to
/// `domain::crypto` (sb X1, main at 0f56f23).
const SEALED_BEFORE_THE_MOVE: &str = concat!(
    "53494d534e41500001000108000000010000000100000007070707070707070707070707070707",
    "090909090909090909090909090909090909090909090909c086fd6a000000004700000000000000",
    "c71605b3ac9796d38a9a2f12236421eb52f714168cb82cb6f6ffa419df959f0fc05debb1998dd42a",
    "02fd68e8587a16dfa98f1a4cc69abe27fceb748a9441f54b79b425c77c2c2b",
);

fn bytes(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(|at| u8::from_str_radix(&hex[at..at + 2], 16).expect("hex")).collect()
}

#[test]
fn a_snapshot_sealed_before_the_crypto_move_still_opens() {
    let file = bytes(SEALED_BEFORE_THE_MOVE);
    let opened = open(&file, PASSPHRASE).expect("opened");
    assert_eq!(opened.plaintext.as_slice(), PAYLOAD);
    assert_eq!((opened.header.kdf, opened.header.created_at), (FAST, CREATED_AT));
    assert_eq!((opened.header.salt, opened.header.nonce), ([7; SALT_LEN], [9; NONCE_LEN]));
    assert_eq!(open(&file, "lamp not a mirror 43").err(), Some(SnapshotError::WrongPassphrase));
}

#[test]
fn sealing_still_writes_the_bytes_written_before_the_crypto_move() {
    let file = seal_with(PAYLOAD, PASSPHRASE, FAST, CREATED_AT, [7; SALT_LEN], [9; NONCE_LEN]).expect("sealed");
    assert_eq!(file, bytes(SEALED_BEFORE_THE_MOVE));
}

#[test]
fn the_payload_is_the_shared_seal_over_the_header() {
    use crate::domain::crypto::{open_with, SealParams};
    let file = bytes(SEALED_BEFORE_THE_MOVE);
    let params = SealParams { kdf: FAST, salt: [7; SALT_LEN], nonce: [9; NONCE_LEN] };
    let plaintext = open_with(&params, &file[HEADER_LEN..], PASSPHRASE, &file[..HEADER_LEN]).expect("opened by the crypto module");
    assert_eq!(plaintext.as_slice(), PAYLOAD);
}
