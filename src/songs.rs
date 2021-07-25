use std::io;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Songs {
    mp3_songs: Vec<String>,
    m4a_songs: Vec<String>,
}

impl Songs {
    pub fn list_songs() -> io::Result<Songs> {
        let mut files = Vec::new();
        for entry in WalkDir::new(".").min_depth(1).max_depth(3) {
            let entry = entry.unwrap();
            let path = entry.path().to_owned();
            files.push(path);
        }
        //cleaning the file names - removing path details
        let file_names: Vec<String>;
        file_names = files
            .iter()
            .map(|res| format!("{:?}", res.to_str().unwrap()))
            .collect();
        let mut mp3_songs = Vec::new();
        let mut m4a_songs = Vec::new();
        for song in file_names {
            //other file formats can be added here too
            let song: String = song[3..song.len() - 1].to_string();
            if song.trim().ends_with(".mp3") {
                mp3_songs.push(song);
            } else if song.trim().ends_with(".m4a") {
                m4a_songs.push(song);
            }
            //Issue an error here if no mp3 files are found
        }
        Ok(Self {
            mp3_songs,
            m4a_songs,
        })
    }
}

