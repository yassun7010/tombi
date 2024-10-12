use crate::{codegen::syntax_kind::generate_syntax_kind, utils::project_root};
use anyhow::Context;
use ungrammar::Grammar;

/// Codegen Grammer.
#[derive(clap::Args)]
pub struct Args {}

pub fn run(_args: Args) -> Result<(), anyhow::Error> {
    let grammar = std::fs::read_to_string(project_root().join("xtask/toml.ungram"))
        .unwrap()
        .parse::<Grammar>()
        .unwrap();

    write_file(
        &generate_syntax_kind(&grammar)
            .context(format!("Failed to generate syntax kind from grammar."))?,
        &project_root().join("crates/syntax/src/generated/syntax_kind.rs"),
    );
    Ok(())
}

#[cfg(not(test))]
fn write_file(content: &str, target: &std::path::Path) {
    if !target.exists() {
        std::fs::create_dir_all(target.parent().unwrap())
            .unwrap_or_else(|err| panic!("Failed to create directory: {}", err));
    }
    std::fs::write(target, content)
        .unwrap_or_else(|err| panic!("Failed to write file {}: {}", target.display(), err));
}

#[cfg(test)]
fn write_file(_content: &str, _target: &std::path::Path) {}
