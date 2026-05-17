use crate::{
    calibration::*, calprofile::CalProfile, enums, enums::Instrument, psycheimage::PsycheImage,
    util,
};

use sciimg::prelude::*;

use anyhow::Result;

#[derive(Copy, Clone)]
pub struct PsycheCameraA {}
impl Calibration for PsycheCameraA {
    fn accepts_instrument(&self, instrument: Instrument) -> bool {
        matches!(instrument, Instrument::PsycheCameraA)
    }

    fn process_file(
        &self,
        input_file: &str,
        cal_context: &CalProfile,
        only_new: bool,
    ) -> Result<CompleteContext> {
        let out_file = util::append_file_name(input_file, cal_context.filename_suffix.as_str());
        if path::file_exists(&out_file) && only_new {
            warn!("Output file exists, skipping. ({})", out_file);
            cal_warn(cal_context, &out_file)
        } else {
            let mut raw = PsycheImage::open(input_file, enums::Instrument::PsycheCameraA);
            raw.desmear_ccd_image(cal_context.desmear_epsilon);

            // Doesn't actually do anything yet.
            info!("Writing to disk...");
            raw.update_history();
            match raw.save(&out_file) {
                Ok(_) => cal_ok(cal_context, &out_file),
                Err(why) => {
                    error!("Error saving file: {}", why);
                    cal_fail(cal_context, &out_file)
                }
            }
        }
        // Do stuff
    }
}
