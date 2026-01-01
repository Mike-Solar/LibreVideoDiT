use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CameraType{
    pub camera_type: String,
    pub video_root: String,
}

impl CameraType{
    pub fn new()->Self{
        CameraType{
            camera_type: String::new(),
            video_root: String::new()
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
