use crate::{constants, metadata::*};
use sciimg::prelude::*;

use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageRecord {
    pub id: u32,
    pub imageid: String,
    pub url: String,
    pub date_taken_utc: String,
    pub date_received: String,
    pub width: u32,
    pub height: u32,
    pub instrument: String,
    pub camera_name: String,
    pub camera_title: String,
    pub filter: u32,
    pub filter_name: String,
    pub filter_wavelength: String,
    pub target: Option<String>,
    pub distance: Option<u32>,
    pub orbit_number: Option<u32>,
    pub spacecraft_clock: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct PsycheApiResults {
    pub items: Vec<ImageRecord>,
    pub per_page: String,
    pub total: u32,
    pub page: u32,
}

impl ImageMetadata for ImageRecord {
    fn get_date_received(&self) -> String {
        self.date_received.clone()
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_imageid(&self) -> String {
        self.imageid.clone()
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_date_taken_utc(&self) -> String {
        self.date_taken_utc.clone()
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn get_instrument(&self) -> String {
        self.instrument.clone()
    }

    fn get_camera_name(&self) -> String {
        self.camera_name.clone()
    }

    fn get_camera_title(&self) -> String {
        self.camera_title.clone()
    }

    fn get_filter(&self) -> u32 {
        self.filter
    }

    fn get_filter_name(&self) -> String {
        self.filter_name.clone()
    }

    fn get_filter_wavelength(&self) -> String {
        self.filter_wavelength.clone()
    }

    fn get_target(&self) -> Option<String> {
        self.target.clone()
    }

    fn get_distance(&self) -> Option<u32> {
        self.distance.clone()
    }

    fn get_orbit_number(&self) -> Option<u32> {
        self.orbit_number.clone()
    }

    fn get_spacecraft_clock(&self) -> Option<f64> {
        self.spacecraft_clock.clone()
    }

    fn get_created_at(&self) -> String {
        self.created_at.clone()
    }

    fn get_updated_at(&self) -> String {
        self.updated_at.clone()
    }
}

pub fn load_metadata_file(file_path: String) -> Result<Metadata> {
    vprintln!("Loading metadata file from {}", file_path);

    if !path::file_exists(file_path.as_str()) {
        return Err(anyhow!(constants::status::FILE_NOT_FOUND));
    }

    let mut file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };

    let mut buf: Vec<u8> = Vec::default();
    file.read_to_end(&mut buf).unwrap();
    let s = String::from_utf8(buf).unwrap();

    let res: ImageRecord = serde_json::from_str(s.as_str()).unwrap();

    Ok(convert_to_std_metadata(&res))
}
