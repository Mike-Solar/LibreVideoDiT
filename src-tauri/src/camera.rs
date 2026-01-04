use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CameraProfile {
    pub name: String,
    pub signature_paths: Vec<String>,
    pub media_roots: Vec<String>,
}

impl CameraProfile {
    pub fn new() -> Self {
        CameraProfile {
            name: String::new(),
            signature_paths: Vec::new(),
            media_roots: Vec::new(),
        }
    }
}
