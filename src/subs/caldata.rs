use anyhow::Result;
use clap::Parser;

use crate::subs::runnable::RunnableSubcommand;
use crate::{caldata, pb_create, pb_inc, pb_set_length, pb_set_print};

pb_create!();

#[derive(Parser)]
#[command(author, version, about = "Updated calibration data from remote repository", long_about = None)]
pub struct UpdateCalData {
    #[arg(long, short, help = "Do not replace existing files")]
    noreplace: bool,

    #[arg(long, short, help = "Override default storage path")]
    local_store: Option<String>,
}

impl RunnableSubcommand for UpdateCalData {
    async fn run(&self) -> Result<()> {
        pb_set_print!();

        match caldata::update_calibration_data(
            !self.noreplace,
            &self.local_store,
            |total| pb_set_length!(total),
            || pb_inc!(),
        )
        .await
        {
            Ok(_) => info!("Done."),
            Err(why) => error!("Error: {}", why),
        };

        Ok(())
    }
}
