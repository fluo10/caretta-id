#![cfg(feature = "redb")]

use std::sync::LazyLock;

use grain_id::GrainId;
use redb::{Database, ReadableDatabase, TableDefinition, backends::InMemoryBackend};

const DATABASE: LazyLock<redb::Database> = LazyLock::new(|| {
    Database::builder()
        .create_with_backend(InMemoryBackend::new())
        .unwrap()
});

const TABLE: TableDefinition<GrainId, GrainId> = TableDefinition::new(stringify!($mod_name));

fn assert_insert(key: GrainId, value: GrainId) {
    let database = DATABASE;
    {
        let write_txn = database.begin_write().unwrap();
        {
            let mut table = write_txn.open_table(TABLE).unwrap();
            let _ = table.insert(&key, &value).unwrap();
        }
        write_txn.commit().unwrap();
    }
    {
        let read_txn = database.begin_read().unwrap();
        {
            let table = read_txn.open_table(TABLE).unwrap();
            assert_eq!(table.get(key).unwrap().unwrap().value(), value);
        }
    }
}
#[test]
fn nil() {
    assert_insert(<GrainId>::NIL, <GrainId>::random());
}
#[test]
fn max() {
    assert_insert(<GrainId>::MAX, <GrainId>::random());
}
#[test]
fn random() {
    for _ in 0..10 {
        assert_insert(<GrainId>::random(), <GrainId>::random());
    }
}
