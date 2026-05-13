#![allow(unused)]

mod caldata;
mod calibfile;
mod calibration;
mod calprofile;
mod constants;
mod decompanding;
mod enums;
mod flatfield;
mod httpfetch;
mod inpaintmask;
mod jsonfetch;
mod memcache;
mod metadata;
mod psyche;
mod psycheimage;
mod remotequery;
mod serializers;
mod subs;
mod util;

use anyhow::Result;
use colored::Colorize;
use subs::runnable::RunnableSubcommand;

#[macro_use]
extern crate stump;

extern crate wild;
use clap::{Parser, Subcommand};

#[macro_use]
extern crate lazy_static;

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
    PsycheFetch(subs::psychefetch::PsycheFetch),
    Info(subs::info::Info),
    Calibrate(subs::calibrate::Calibrate),
    HpcFilter(subs::hpcfilter::HpcFilter),
    Profile(subs::profile::Profile),
    UpdateCalData(subs::caldata::UpdateCalData),
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
        Pru::Info(args) => args.run().await,
        Pru::Calibrate(args) => args.run().await,
        Pru::HpcFilter(args) => args.run().await,
        Pru::Profile(args) => args.run().await,
        Pru::UpdateCalData(args) => args.run().await,
    } {
        error!("{}", "Unhandled program error:".red());
        error!("{}", why);
    };
    info!("Runtime: {}s", t1.elapsed().as_secs_f64());
    Ok(())
}
