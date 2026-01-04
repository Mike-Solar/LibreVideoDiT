use std::fs::File;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use crate::camera::CameraProfile;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new( Config::load().unwrap_or_else(|error| {Config::new()}));
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SdCardMapping {
    pub root: String,
    pub target_subdir: String,
    pub camera_override: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub destination_root: String,
    pub cameras: Vec<CameraProfile>,
    pub video_exts: Vec<String>,
    pub photo_exts: Vec<String>,
    pub sd_cards: Vec<SdCardMapping>,
}


impl Config {

    pub fn new()->Self{
        return Config{
            destination_root: String::new(),
            cameras: Vec::new(),
            video_exts: Vec::new(), 
            photo_exts: Vec::new(),
            sd_cards: Vec::new(),
        };
    }
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let config_file = match File::open("config.json"){
            Ok(file) => file,
            Err(error) => return Err(Box::new(error))
        };
        let config_:Config=match serde_json::from_reader(config_file){
            Ok(config_) => config_,
            Err(error) => return Err(Box::new(error))
        };
        Ok(config_)
    }
}
