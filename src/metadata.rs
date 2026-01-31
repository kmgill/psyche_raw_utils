use crate::serializers;
use serde::{Deserialize, Serialize};

use anyhow::Result;
use std::fs::File;
use std::io::Read;

pub trait ImageMetadata {
    fn get_id(&self) -> u32;
    fn get_imageid(&self) -> String;
    fn get_url(&self) -> String;
    fn get_date_taken_utc(&self) -> String;
    fn get_date_received(&self) -> String;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_instrument(&self) -> String;
    fn get_camera_name(&self) -> String;
    fn get_camera_title(&self) -> String;
    fn get_filter(&self) -> u32;
    fn get_filter_name(&self) -> String;
    fn get_filter_wavelength(&self) -> String;
    fn get_target(&self) -> Option<String>;
    fn get_distance(&self) -> Option<u32>;
    fn get_orbit_number(&self) -> Option<u32>;
    fn get_spacecraft_clock(&self) -> Option<f64>;
    fn get_created_at(&self) -> String;
    fn get_updated_at(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Metadata {
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

pub fn convert_to_std_metadata<T: ImageMetadata>(im: &T) -> Metadata {
    Metadata {
        id: im.get_id(),
        imageid: im.get_imageid(),
        url: im.get_url(),
        date_taken_utc: im.get_date_taken_utc(),
        date_received: im.get_date_received(),
        width: im.get_width(),
        height: im.get_height(),
        instrument: im.get_instrument(),
        camera_name: im.get_camera_name(),
        camera_title: im.get_camera_title(),
        filter: im.get_filter(),
        filter_name: im.get_filter_name(),
        filter_wavelength: im.get_filter_wavelength(),
        target: im.get_target(),
        distance: im.get_distance(),
        orbit_number: im.get_orbit_number(),
        spacecraft_clock: im.get_spacecraft_clock(),
        created_at: im.get_created_at(),
        updated_at: im.get_updated_at(),
    }
}

pub fn load_image_metadata(json_path: &String) -> Result<Metadata> {
    let mut file = match File::open(json_path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };

    let mut buf: Vec<u8> = Vec::default();
    file.read_to_end(&mut buf).unwrap();
    let json = String::from_utf8(buf).unwrap();

    let metadata = serde_json::from_str(&json).unwrap();

    Ok(metadata)
}
