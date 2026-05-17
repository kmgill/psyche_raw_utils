use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use sciimg::enums::ImageMode;
use sciimg::image::Image;

use anyhow::anyhow;

use crate::psycheimage::PsycheImage;
use crate::subs::runnable::RunnableSubcommand;
use crate::{caldata, pb_create, pb_inc, pb_set_length, pb_set_print};

pb_create!();

#[derive(Parser)]
#[command(author, version, about = "Compose RGB image from monochrome frames", long_about = None)]
pub struct Compose {
    #[arg(long, short, help = "Red band input image")]
    red: PathBuf,

    #[arg(long, short, help = "Green band input image")]
    green: PathBuf,

    #[arg(long, short, help = "Blue band input image")]
    blue: PathBuf,

    #[arg(long, short, help = "Output image")]
    output: PathBuf,
}

impl RunnableSubcommand for Compose {
    async fn run(&self) -> Result<()> {
        pb_set_print!();

        let red = if let Ok(img) = Image::open(self.red.as_os_str().to_str().unwrap()) {
            img
        } else {
            return Err(anyhow!("Failed to load red band image: {:?}", self.red));
        };

        let green = if let Ok(img) = Image::open(self.green.as_os_str().to_str().unwrap()) {
            img
        } else {
            return Err(anyhow!("Failed to load green band image: {:?}", self.green));
        };

        let blue = if let Ok(img) = Image::open(self.blue.as_os_str().to_str().unwrap()) {
            img
        } else {
            return Err(anyhow!("Failed to load blue band image: {:?}", self.blue));
        };

        let rgb = if let Ok(img) = Image::new_from_buffers_rgb(
            red.get_band(0),
            green.get_band(0),
            blue.get_band(0),
            red.get_mode(),
        ) {
            img
        } else {
            return Err(anyhow!("Failed to compose RGB image from bands"));
        };

        rgb.save(self.output.as_os_str().to_str().unwrap())
    }
}
