use std::fs::File;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
lazy_static!(
    pub static ref config: Mutex<Config> = Mutex::new( match Config::load(){
        Ok(cfg)=>cfg,
        Err(error)=>{Config::new()}
    });
);


#[derive(Deserialize, Serialize)]
pub struct Config {
    pub cameras: Vec<Camera>,
    pub video_exts: Vec<String>,
    pub photo_exts: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Camera{
    pub camera_type: String,
    pub video_root: String,
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