mod constants;
mod enums;
mod httpfetch;
mod jsonfetch;
mod metadata;
mod psyche;
mod remotequery;
mod serializers;
mod subs;
mod util;

use anyhow::Result;
use colored::Colorize;
use subs::runnable::RunnableSubcommand;
use subs::*;


#[macro_use]
extern crate stump;

extern crate wild;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "pru")]
#[clap(about = "Psyche Raw Utils", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Pru,

    #[clap(long, short, help = "Verbose output")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Pru {
    #[clap(name = "fetch")]
    PsycheFetch(psychefetch::PsycheFetch)
    // MslFetch(msl::mslfetch::MslFetch),
    // MslDate(msl::msldate::MslDate),

    // #[clap(name = "diffgif")]
    // DiffGif(diffgif::DiffGif),
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let t1 = std::time::Instant::now();

    stump::set_min_log_level(stump::LogEntryLevel::WARN);
    info!("Initialized logging"); // INFO, which means that this won't be seen
                                  // unless the user overrides via environment
                                  // variable.

    let args = Cli::parse_from(wild::args());

    if args.verbose {
        stump::set_verbose(true);
    }

    if let Err(why) = match args.command {
        Pru::PsycheFetch(args) => args.run().await,
    } {
        error!("{}", "Unhandled program error:".red());
        error!("{}", why);
    };
    info!("Runtime: {}s", t1.elapsed().as_secs_f64());
    Ok(())
}
