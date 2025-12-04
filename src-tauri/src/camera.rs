use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CameraType{
    pub camera_type: String,
    pub video_root: String,
}

impl CameraType{
    pub fn new()->Self{
        return CameraType{
            camera_type: String::new(),
            video_root: String::new()
        }
    }
}

impl Clone for CameraType{
    fn clone(&self) -> Self {
        return CameraType{
            camera_type: self.camera_type.clone(),
            video_root: self.video_root.clone()
        }
    }
}
impl Display for CameraType{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CameraType:{:10}, Video Root: {}", self.camera_type, self.video_root)
    }
}
pub struct Camera{
    camera_type: CameraType,
    camera_name: String,
}

impl Camera{
    pub fn new() -> Camera{
        Camera{
            camera_type:CameraType::new(),
            camera_name:String::new()
        }
    }
}
