use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

mod cmd;

const VERSION: &str = concat!("v", crate_version!());

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .map_err(|e| anyhow::anyhow!(e))?;

    let matches = App::new(crate_name!())
        .version(VERSION)
        .author(crate_authors!())
        .about(crate_description!())
        // .arg("-c, --config=[FILE] 'Sets a custom config file'")
        .subcommand(
            App::new("build").about("build sqlite db indexes from markdown files"), // .arg("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            App::new("server")
                .about("run a search server for web")
                .arg(Arg::from("-p, --port=[PORT] 'Server port'").default_value("3000")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("build", args)) => cmd::build::execute(args),
        Some(("server", args)) => {
            use tokio::runtime::Builder;
            let rt = Builder::new_multi_thread().enable_all().build()?;
            rt.block_on(cmd::server::execute(args))
        }
        _ => Ok(()),
    }
}
