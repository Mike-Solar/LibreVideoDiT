use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::config::{config, Config};
use crate::camera::CameraType;

enum FileType{
    Photo(String),
    Video(String)
}

struct MediaFile{
    pub path: PathBuf,
    pub file_type: FileType,
}

#[derive(Debug)]
struct MediaNotFoundError{
    cameras: Vec<CameraType>
}

impl MediaNotFoundError{
    fn new()-> Self{
        let config_=match config.try_read() { 
            Ok(cfg)=>cfg,
            Err(E)=>return MediaNotFoundError{
                cameras: Vec::new()
            }
        };
        return MediaNotFoundError{
            cameras: config_.cameras.clone()
        }
    }
}
impl Display for MediaNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut camera_list="".to_string();
        for camera in self.cameras.iter().as_slice(){
            camera_list+=camera.to_string().as_str();
        }
        write!(f, "Can't find video. Video roots in config file: \n {}", camera_list)
    }
}

impl Error for MediaNotFoundError{
}

// List all videos
fn list_videos(src: PathBuf)->Result<Vec<MediaFile>,Box<dyn std::error::Error>> {
    let path=src.as_path();
    let mut media_files=Vec::<MediaFile>::new();
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry=entry?;
            let path = entry.path();
            if path.is_dir() {
                media_files.append(&mut list_videos(path)?);
            }
            else if path.is_file() {
                let ext=match path.extension(){
                    Some(ext) => {ext}
                    None => continue
                };
                let ext=match ext.to_str(){
                    Some(ext) => {ext}
                    None => continue
                };
                if config.try_read()?.video_exts.contains(&ext.to_string()) {
                    media_files.push(MediaFile{
                        path: path.to_path_buf(),
                        file_type: FileType::Video(ext.to_string())
                    })
                }
            }
        }
        return Ok(media_files);
    }
    else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound,"not a directory")))
    }

}

fn find_video() -> Result<Vec<MediaFile>, Box<dyn std::error::Error>> {
    for camera in config.try_read()?.cameras.clone() {
        let pathbuf = PathBuf::from_str(camera.video_root.as_str())?;
        let path = pathbuf.as_path();

        if path.exists() {
            return list_videos(path.to_path_buf());
        }
    }
    Err(Box::new(MediaNotFoundError::new()))
}
