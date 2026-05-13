use crate::enums::CalFileType;
use crate::{constants, enums};
use anyhow::anyhow;
use anyhow::Result;
use sciimg::path;
use std::env;
use std::fs::File;
use std::io::Read;

//use serde_derive::Deserialize;
use serde::Deserialize;

fn default_blank() -> String {
    "".to_string()
}

fn default_instrument_properties() -> InstrumentProperties {
    InstrumentProperties {
        flat: default_blank(),
        inpaint_mask: default_blank(),
        mask: default_blank(),
        lut: default_blank(),
    }
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub psyche: PsycheCalData,
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct InstrumentProperties {
    #[serde(default = "default_blank")]
    pub flat: String,

    #[serde(default = "default_blank")]
    pub inpaint_mask: String,

    #[serde(default = "default_blank")]
    pub mask: String,

    #[serde(default = "default_blank")]
    pub lut: String,
}

#[derive(Clone)]
pub struct CalFilePathAndType {
    pub file: String,
    pub file_type: CalFileType,
}

impl IntoIterator for InstrumentProperties {
    type Item = CalFilePathAndType;
    type IntoIter = std::array::IntoIter<CalFilePathAndType, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [
            CalFilePathAndType {
                file: self.flat,
                file_type: enums::CalFileType::FlatField,
            },
            CalFilePathAndType {
                file: self.inpaint_mask,
                file_type: enums::CalFileType::InpaintMask,
            },
            CalFilePathAndType {
                file: self.lut,
                file_type: enums::CalFileType::Lut,
            },
            CalFilePathAndType {
                file: self.mask,
                file_type: enums::CalFileType::Mask,
            },
        ]
        .into_iter()
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct PsycheCalData {
    #[serde(default = "default_instrument_properties")]
    pub cameraA: InstrumentProperties,

    #[serde(default = "default_instrument_properties")]
    pub cameraB: InstrumentProperties,
}

pub fn parse_caldata_from_string(caldata_toml_str: &str) -> Result<Config> {
    match toml::from_str(caldata_toml_str) {
        Ok(c) => Ok(c),
        Err(_) => Err(anyhow!("Failed to parse calibration manifest")),
    }
}

pub fn load_caldata_mapping_file() -> Result<Config> {
    if let Ok(caldata_toml) = locate_calibration_file(&String::from("caldata.toml")) {
        info!("Loading calibration spec from {}", caldata_toml);

        let mut file = match File::open(&caldata_toml) {
            Err(why) => panic!("couldn't open {}", why),
            Ok(file) => file,
        };

        let mut buf: Vec<u8> = Vec::default();
        file.read_to_end(&mut buf).unwrap();
        let toml = String::from_utf8(buf).unwrap();

        parse_caldata_from_string(&toml)
    } else {
        Err(anyhow!("Unable to locate calibration configuration file"))
    }
}

/// Allows the user to specify files without an extension as a shortcut. Still needs to be able
/// to guess an extension, though
pub fn locate_calibration_file_no_extention(
    file_path: &String,
    extension: &String,
) -> Result<String> {
    match locate_calibration_file(file_path) {
        Ok(fp) => Ok(fp),
        Err(_) => {
            let with_ext = format!("{}{}", file_path, extension);
            locate_calibration_file(&with_ext)
        }
    }
}

pub fn locate_calibration_file(file_path: &str) -> Result<String> {
    // If the file exists as-is, return it
    if path::file_exists(file_path) {
        return Ok(file_path.into());
    }

    // Some default locations
    let mut locations = vec![
        String::from("mars-raw-utils-data/caldata"), // Running within the repo directory (dev: cargo run --bin ...)
        String::from("/usr/share/mars_raw_utils/data/"), // Linux, installed via apt or rpm
    ];

    if let Ok(exe_path) = std::env::current_exe() {
        if cfg!(windows) {
            // I'm not even a little comfortable with this...
            // So, to figure out the installation path, we get the path to the running executable, then get the path, and then
            // append 'data' to it to get to the calibration files. We also have to get rid of those quotation marks.
            if let Some(filename) = exe_path.parent() {
                locations.insert(
                    0,
                    format!("{:?}", filename.with_file_name("data").as_os_str()).replace('\"', ""),
                );
            }
        }
    }

    // Allow for a custom data path to be defined during build.
    if let Some(v) = option_env!("PSYCHEDATAROOT") {
        locations.insert(0, String::from(v));
    }

    // Add a path based on the location of the running executable
    // Intended for Windows installations
    if let Ok(exe_path) = std::env::current_exe() {
        if cfg!(windows) {
            let bn = format!("{:?}/../data/", exe_path.file_name());
            locations.insert(0, bn);
        }
    }

    // Prepend a home directory if known
    if let Some(dir) = dirs::home_dir() {
        let homedatadir = format!("{}/.psychedata", dir.to_str().unwrap());
        locations.insert(0, homedatadir);
    }

    // Prepend a location specified by environment variable
    if let Ok(dir) = env::var("PSYCHE_RAW_DATA") {
        locations.insert(0, dir);
    }

    debug!("Calibration file search path: {:?}", locations);

    // First match wins
    for loc in locations.iter() {
        let full_file_path = format!("{}/{}", loc, file_path);
        if path::file_exists(&full_file_path) {
            return Ok(full_file_path);
        }
    }

    // Oh nos!
    Err(anyhow!("Calibration file not found: {}", file_path))
}

pub fn get_calibration_file_for_type(
    inst_props: &InstrumentProperties,
    cal_file_type: enums::CalFileType,
) -> String {
    match cal_file_type {
        enums::CalFileType::FlatField => inst_props.flat.clone(),
        enums::CalFileType::InpaintMask => inst_props.inpaint_mask.clone(),
        enums::CalFileType::Mask => inst_props.mask.clone(),
        enums::CalFileType::Lut => inst_props.lut.clone(),
    }
}

pub fn get_calibration_base_file_for_instrument(
    instrument: enums::Instrument,
    cal_file_type: enums::CalFileType,
) -> Result<String> {
    let config = load_caldata_mapping_file()?;

    match instrument {
        enums::Instrument::PsycheCameraA => Ok(get_calibration_file_for_type(
            &config.psyche.cameraA,
            cal_file_type,
        )),
        enums::Instrument::PsycheCameraB => Ok(get_calibration_file_for_type(
            &config.psyche.cameraB,
            cal_file_type,
        )),
        enums::Instrument::None => Err(anyhow!(constants::status::UNSUPPORTED_INSTRUMENT)),
    }
}

pub fn get_calibration_file_for_instrument(
    instrument: enums::Instrument,
    cal_file_type: enums::CalFileType,
) -> Result<String> {
    match get_calibration_base_file_for_instrument(instrument, cal_file_type) {
        Ok(file_name) => match file_name.len() {
            0 => Err(anyhow!(constants::status::UNSUPPORTED_INSTRUMENT)),
            _ => locate_calibration_file(&file_name),
        },
        Err(e) => Err(e),
    }
}
