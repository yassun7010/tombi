use super::{leading_comments, peek_leading_comments, tailing_comment, Grammer};
use crate::{parser::Parser, token_set::TokenSet};
use syntax::{SyntaxKind::*, T};

impl Grammer for ast::Value {
    fn parse(p: &mut Parser<'_>) {
        let n = peek_leading_comments(p);
        match p.nth(n) {
            BASIC_STRING
            | MULTI_LINE_BASIC_STRING
            | LITERAL_STRING
            | MULTI_LINE_LITERAL_STRING
            | INTEGER_DEC
            | INTEGER_BIN
            | INTEGER_OCT
            | INTEGER_HEX
            | FLOAT
            | BOOLEAN
            | OFFSET_DATE_TIME
            | LOCAL_DATE_TIME
            | LOCAL_DATE
            | LOCAL_TIME => parse_literal_value(p),
            T!('[') => ast::Array::parse(p),
            T!('{') => ast::InlineTable::parse(p),
            _ => parse_invalid_value(p),
        }
    }
}

fn parse_literal_value(p: &mut Parser<'_>) {
    let m = p.start();

    leading_comments(p);

    let kind = p.current();

    p.bump(kind);

    tailing_comment(p);

    m.complete(p, kind);
}

fn parse_invalid_value(p: &mut Parser<'_>) {
    let m = p.start();

    leading_comments(p);

    while !p.at_ts(TokenSet::new(&[LINE_BREAK, COMMENT, EOF])) {
        p.bump_any();
    }
    p.error(crate::Error::ExpectedValue);

    tailing_comment(p);

    m.complete(p, INVALID_TOKEN);
}
