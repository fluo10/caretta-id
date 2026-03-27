//! Test for serde feature
#![cfg(feature = "serde")]

use grain_id::GrainId;

use serde_test::assert_de_tokens;
use serde_test::{Compact, Configure, Readable, Token, assert_ser_tokens, assert_tokens};

fn assert_tokens_readable<'de>(value: &Readable<GrainId>, tokens: &'de [Token]) {
    assert_ser_tokens(value, tokens);
    #[cfg(feature = "std")]
    assert_de_tokens(value, tokens);
}
fn assert_tokens_compact<'de>(value: &Compact<GrainId>, tokens: &'de [Token]) {
    assert_tokens(value, tokens);
}

#[test]
fn nil_readable() {
    assert_tokens_readable(&GrainId::NIL.readable(), &[Token::Str("0000000")])
}

#[test]
fn max_readable() {
    assert_tokens_readable(&GrainId::MAX.readable(), &[Token::Str("zzzzzzz")]);
}

#[test]
fn nil_compact() {
    assert_tokens_compact(&GrainId::NIL.compact(), &[Token::U64(0)])
}

#[test]
fn max_compact() {
    assert_tokens_compact(&GrainId::MAX.compact(), &[Token::U64(0x7FFFFFFFF)]);
}
