use std::fmt::{write, Display, Formatter};
use std::fs::File;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, RwLock};
use crate::camera::CameraType;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new( Config::load().unwrap_or_else(|error| {Config::new()}));
}


#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub cameras: Vec<CameraType>,
    pub video_exts: Vec<String>,
    pub photo_exts: Vec<String>,
}


impl Config {

    pub fn new()->Self{
        return Config{
            cameras: Vec::new(),
            video_exts: Vec::new(), 
            photo_exts: Vec::new()
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