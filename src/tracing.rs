use colored::Colorize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn setup() -> anyhow::Result<()> {
    colored::control::set_override(true); // always apply color

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()) // .without_time()
        .with(
            // let user override RUST_LOG in local run if they want to
            EnvFilter::try_from_default_env()
                // otherwise use our default
                .or_else(|_| tracing_subscriber::EnvFilter::try_new("info,shuttle=trace"))?,
        )
        .init();

    println!("{}", "Custom tracing subscriber is initialized!".yellow());
    Ok(())
}
