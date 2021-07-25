use std::io;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Songs {
    pub mp3_songs: Vec<String>,
    pub m4a_songs: Vec<String>,
}

impl Songs {
    pub fn list_songs(path_str: &str) -> io::Result<Songs> {
        let mut files = Vec::new();
        for entry in WalkDir::new(path_str).min_depth(1).max_depth(3) {
            let entry = entry.unwrap();
            let path = entry.path().to_owned();
            files.push(path);
        }
        //cleaning the file names - removing path details else do not do anything
        let file_names: Vec<String>;
        file_names = files
            .iter()
            .map(|res| format!("{:?}", res.to_str().unwrap()))
            .collect();
        let mut mp3_songs = Vec::new();
        let mut m4a_songs = Vec::new();
        for song in file_names {
            //other file formats can be added here too
            if path_str.contains(".") {
                let song: String = song[3..song.len() - 1].to_string();
                if song.trim().ends_with(".mp3") {
                    mp3_songs.push(song);
                } else if song.trim().ends_with(".m4a") {
                    m4a_songs.push(song);
                }
            } else {
                let song: String =  song[1..song.len() - 1].to_string(); //here is where to change
                if song.trim().ends_with(".mp3") {
                    mp3_songs.push(song);
                } else if song.trim().ends_with(".m4a") {
                    m4a_songs.push(song);
                }
            }
        }
        Ok(Self {
            mp3_songs,
            m4a_songs,
        })
    }
}

//There are no songs in my working directory so the vector should be empty
#[test]
fn no_m4a_songs_in_working_dir() {
    let m4a_songs = Songs::list_songs(".").unwrap().m4a_songs;
    assert_eq!(m4a_songs.is_empty(), true);
}
#[test]
fn no_mp3_songs_in_working_dir() {
    let mp3_songs = Songs::list_songs(".").unwrap().mp3_songs;
    assert_eq!(mp3_songs.is_empty(), true);
}
