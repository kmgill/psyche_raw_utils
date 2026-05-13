use crate::{calibfile, enums, memcache::load_image, psycheimage::PsycheImage};

use anyhow::Result;

pub fn load_flat(instrument: enums::Instrument) -> Result<PsycheImage> {
    match calibfile::get_calibration_file_for_instrument(instrument, enums::CalFileType::FlatField)
    {
        Ok(cal_file) => {
            info!("Loading calibration file from {}", cal_file);
            Ok(PsycheImage::from_image(
                &load_image(&cal_file).unwrap(),
                instrument,
            ))
        }
        Err(e) => Err(e),
    }
}
