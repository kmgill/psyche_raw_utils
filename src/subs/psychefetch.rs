use std::process;

use anyhow::Result;
use clap::Parser;
use sciimg::path;

use crate::psyche::fetch::PsycheFetch as PsycheFetchClient;
// use crate::prelude::*;

use crate::remotequery::FetchError;
use crate::{remotequery, util};
use crate::subs::runnable::RunnableSubcommand;

crate::pb_create!();

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Fetch raw Psyche images", long_about = None)]
pub struct PsycheFetch {
    #[arg(long, short, help = "Psyche Camera Instrument(s)", num_args = 1..)]
    camera: Vec<String>,

    #[arg(long, short = 'd', help = "Image Date")]
    date: Option<String>,

    #[arg(long, short = 'm', help = "Starting Image Date")]
    mindate: Option<String>,

    #[arg(long, short = 'M', help = "Ending Image Date")]
    maxdate: Option<String>,

    #[arg(long, short = 'l', help = "Don't download, only list results")]
    list: bool,

    #[arg(long, short = 'N', help = "Max number of results")]
    num: Option<u32>,

    #[arg(long, short = 'p', help = "Results page (starts at 1)")]
    page: Option<u8>,

    #[arg(long, short = 'f', help = "Camera filter number")]
    filter_num: Option<u8>,

    #[arg(long, short = 'F', help = "filter on image id", num_args = 1..)]
    filter: Option<Vec<String>>,

    #[arg(long, short = 'I', help = "List instruments")]
    instruments: bool,

    #[arg(long, short, help = "Output directory")]
    output: Option<std::path::PathBuf>,

    #[arg(long, short = 'n', help = "Only new images. Skipped processed images.")]
    new: bool,
}

impl RunnableSubcommand for PsycheFetch {
    async fn run(&self) -> Result<()> {
        crate::pb_set_print!();

        let client = PsycheFetchClient::new();

        let im: util::InstrumentMap = remotequery::get_instrument_map(&client).unwrap();
        if self.instruments {
            im.print_instruments();
            process::exit(0);
        }

        let num_per_page = match self.num {
            Some(n) => n as i32,
            None => 100,
        };

        let page = self.page.map(|p| p as i32);

        let search = match &self.filter {
            Some(s) => s.clone(),
            None => vec![],
        };

        let output = match &self.output {
            Some(s) => String::from(s.as_os_str().to_str().unwrap()),
            None => path::cwd(),
        };

        let camera_ids_res = im.find_remote_instrument_names_fromlist(&self.camera);
        let cameras = match camera_ids_res {
            Err(_e) => {
                error!("Invalid camera instrument(s) specified");
                process::exit(1);
            }
            Ok(v) => v,
        };

        let min_date = match &self.mindate {
            Some(s) => s.clone(),
            None => "2000-01-01".to_string()
        };

        let max_date = match &self.maxdate {
            Some(s) => s.clone(),
            None => "2100-01-01".to_string()
        };


        match remotequery::perform_fetch(
            &client,
            &remotequery::RemoteQuery {
                cameras,
                num_per_page,
                page,
                min_date: min_date,
                max_date: max_date,
                list_only: self.list,
                search,
                only_new: self.new,
                filter_num: self.filter_num,
                filter: self.filter.clone(),
                output_path: output,
            },
            |total| crate::pb_set_length!(total),
            |_| crate::pb_inc!(),
        )
        .await
        {
            Ok(_) => info!("Done"),
            Err(FetchError::SkippingFile) => info!("Not downloading images. Done"),
            Err(why) => error!("Error: {}", why),
        };

        Ok(())
    }
}
