use std::path::{Path,PathBuf};
use std::str::FromStr;
use crate::config::config;

enum FileType{
    Photo(String),
    Video(String)
}

struct MediaFile{
    pub path: PathBuf,
    pub file_type: FileType,
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
                if config.try_lock()?.video_exts.contains(&ext.to_string()) {
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

fn find_video() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    for camera in config.try_lock()?.cameras {
        let pathbuf = PathBuf::from_str(camera.video_root.as_str())?;
        let path = pathbuf.as_path();

        if path.exists() {
            break;
        }
    }

    Ok(())
}

fn copyfile(src: String, dst: String, file_type: FileType) -> Result<(), io::Error>{

}