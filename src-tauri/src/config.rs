use std::fmt::{write, Display, Formatter};
use std::fs::File;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, RwLock};
lazy_static!(
    pub static ref config: RwLock<Config> = RwLock::new( match Config::load(){
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

impl Clone for Config{
    fn clone(&self) -> Self {
        return Config{
            cameras: Vec::from(self.cameras.iter().as_slice()),
            video_exts:  Vec::from(self.video_exts.iter().as_slice()),
            photo_exts:  Vec::from(self.photo_exts.iter().as_slice()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Camera{
    pub camera_type: String,
    pub video_root: String,
}

impl Clone for Camera{
    fn clone(&self) -> Self {
        return Camera{
            camera_type: self.camera_type.clone(),
            video_root: self.video_root.clone()
        }
    }
}
impl Display for Camera{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Camera:{:10}, Video Root: {}", self.camera_type, self.video_root)
    }
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