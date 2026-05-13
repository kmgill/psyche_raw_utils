use crate::constants;
use crate::enums::Instrument;
use crate::psycheimage::PsycheImage;
use crate::subs::runnable::RunnableSubcommand;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Image information", long_about = None)]
pub struct Info {
    #[arg(long, short, help = "Input images", required(true), num_args = 1..)]
    input_files: Vec<std::path::PathBuf>,
}

pub trait YesNo {
    fn yesno(&self) -> String;
}

impl YesNo for bool {
    fn yesno(&self) -> String {
        if *self {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    }
}

impl RunnableSubcommand for Info {
    async fn run(&self) -> Result<()> {
        for in_file in self.input_files.iter() {
            if in_file.exists() {
                println!("Image: {:?}", in_file);
                let img =
                    PsycheImage::open(in_file.as_os_str().to_str().unwrap(), Instrument::None);

                println!("Instrument:                  {}", img.metadata.instrument);
                println!("Camera Name:                 {}", img.metadata.camera_name);
                println!("Camera Title:                {}", img.metadata.camera_title);
                println!("Image Id:                    {}", img.metadata.imageid);

                if let Some(sclk) = img.metadata.spacecraft_clock {
                    println!("Spcacecraft Clock:           {}", sclk);
                }

                println!(
                    "Date Taken (UTC):            {}",
                    img.metadata.date_taken_utc
                );

                println!(
                    "Data Receieved (UTC):        {}",
                    img.metadata.date_received
                );

                println!("Filter #:                    {}", img.metadata.filter);

                println!("Filter Name:                 {}", img.metadata.filter_name);

                println!(
                    "Filter Wavelength:           {}",
                    img.metadata.filter_wavelength
                );

                if let Some(t) = img.metadata.target {
                    println!("Filter Name:                 {}", t);
                }

                if let Some(d) = img.metadata.distance {
                    println!("Distance:                {}", d);
                }

                if let Some(on) = img.metadata.orbit_number {
                    println!("Orbit Number:              {}", on);
                }

                println!("Image Width:                 {}", img.metadata.width);
                println!("Image Height:                {}", img.metadata.height);

                println!(
                    "Decompanded:                 {}",
                    img.metadata.decompand.yesno()
                );
                println!(
                    "Debayered:                   {}",
                    img.metadata.debayer.yesno()
                );
                println!(
                    "Flatfielded:                 {}",
                    img.metadata.flatfield.yesno()
                );
                println!(
                    "Radiometric Correction:      {}",
                    img.metadata.radiometric.yesno()
                );
                println!(
                    "Inpainted:                   {}",
                    img.metadata.inpaint.yesno()
                );
                println!(
                    "Cropped:                     {}",
                    img.metadata.cropped.yesno()
                );

                //println!("Caption:                     {}", md.caption);
                println!(
                    "Credit:                      {}",
                    constants::RAW_IMAGE_CREDIT
                );

                // Consider adding values derived from CAHVOR camera models
                println!();
                println!();
            } else {
                error!("File not found: {:?}", in_file);
            }
        }
        Ok(())
    }
}
