#![cfg(feature = "prost")]

use grain_id::GrainIdProto;

#[test]
fn nil() {
    let nil = GrainIdProto { value: 0 };
    assert_eq!(
        <grain_id::GrainId>::NIL,
        <grain_id::GrainId>::try_from(nil).unwrap()
    );
}

#[test]
fn max() {
    let max = GrainIdProto {
        value: <u64>::from(<grain_id::GrainId>::MAX),
    };
    assert_eq!(
        <grain_id::GrainId>::MAX,
        <grain_id::GrainId>::try_from(max).unwrap()
    );
}

#[test]
#[should_panic]
fn oversized() {
    let oversized = GrainIdProto {
        value: <u64>::from(<grain_id::GrainId>::MAX) + 1,
    };
    let _ = <grain_id::GrainId>::try_from(oversized).unwrap();
}
