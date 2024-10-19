use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tracing_subscriber::prelude::*;

/// TOML: TOML linter and code formatter.
#[derive(clap::Parser)]
#[command(name = "toml", version = crate::version())]
pub struct Args {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

impl<I, T> From<I> for Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    #[inline]
    fn from(value: I) -> Self {
        Self::parse_from(value)
    }
}

pub fn run(args: impl Into<Args>) -> Result<(), anyhow::Error> {
    let args: Args = args.into();

    println!("version: {}", crate::version());

    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(
            args.verbose.log_level_filter().to_string(),
        ))
        .with(tracing_subscriber::fmt::layer().pretty().without_time())
        .init();

    crate::server::run()
}
