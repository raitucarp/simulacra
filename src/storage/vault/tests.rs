use super::Vault;
use crate::domain::{Post, Profile};
use crate::error::AppError;
use crate::storage::{Database, SettingKey};

fn vault() -> Vault {
    Vault::spawn(Database::in_memory().unwrap()).unwrap()
}

#[test]
fn reads_observe_every_earlier_queued_write() {
    let vault = vault();
    let author = Profile::new_primary("u1", "Ada", "@ada");
    vault.save_profile(&author).unwrap();
    for i in 0..50 {
        vault.save_post(&Post::create(format!("p{i}"), "net", &author, "queued").unwrap()).unwrap();
    }
    assert_eq!(vault.load_posts().unwrap().len(), 50);
    assert_eq!(vault.load_profiles().unwrap().len(), 1);
}

#[test]
fn failed_writes_reach_the_failure_channel() {
    let vault = vault();
    let mut failures = vault.take_failures().expect("first take");
    assert!(vault.take_failures().is_none(), "the receiver is handed out once");
    vault.write(|_| Err(AppError::DatabaseError("boom".into()))).unwrap();
    vault.flush().unwrap();
    assert_eq!(failures.try_recv().unwrap(), AppError::DatabaseError("boom".into()));
}

#[test]
fn dropping_the_last_handle_flushes_pending_writes() {
    let path = std::env::temp_dir().join(format!("simulacra_vault_flush_{}.db", uuid::Uuid::new_v4()));
    {
        let vault = Vault::spawn(Database::open(&path).unwrap()).unwrap();
        vault.set_setting(SettingKey::SchedulerEnabled, "v").unwrap();
    }
    let reopened = Database::open(&path).unwrap();
    assert_eq!(reopened.get_setting(SettingKey::SchedulerEnabled).unwrap().as_deref(), Some("v"));
    drop(reopened);
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
    }
}
