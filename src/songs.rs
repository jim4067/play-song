use std::io;
use walkdir::WalkDir;

pub fn list_songs() -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(".").min_depth(1).max_depth(3) {
        let entry = entry.unwrap();
        let path = entry.path().to_owned();
        files.push(path);
    }

    //removing the ./ in files
    let file_names: Vec<String>;
    file_names = files
        .iter()
        .map(|res| format!("{:?}", res.to_str().unwrap()))
        .collect();

    let mut songs = Vec::new();
    for song in file_names {
        let song: String = song[3..song.len() - 1].to_string();
        if song.trim().ends_with(".mp3") {
            songs.push(song);
        }
        //Issue an error here if no mp3 files are found
    }
    Ok(songs)
}

//lists the songs found in the directory
