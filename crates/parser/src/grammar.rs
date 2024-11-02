mod array;
mod array_of_table;
mod inline_table;
mod key;
mod key_value;
mod root;
mod table;
mod value;

use root::parse_root;

use crate::output::Output;
use crate::SyntaxKind::*;

pub fn parse(input: &crate::Input) -> Output {
    let _p = tracing::info_span!("grammar::parse").entered();
    let mut p = crate::parser::Parser::new(input);

    parse_root(&mut p);

    let events = p.finish();

    crate::event::process(events)
}

fn leading_comments(p: &mut crate::parser::Parser<'_>) {
    while p.eat(WHITESPACE) || p.eat(COMMENT) || p.eat(NEWLINE) {}
}

fn tailing_comment(p: &mut crate::parser::Parser<'_>) {
    while p.eat(WHITESPACE) || p.eat(COMMENT) {}
}
