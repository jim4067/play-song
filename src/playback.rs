use crate::songs::Songs;
use id3::Tag;
use redlux::Decoder;
use rodio;
use std::io;
use std::{env, fs::File, io::BufReader, path::Path, process};

// find out if there is a way to pause using rodio - there is
//find out if there is a way to flush the std::output after printing to it
// and if there is a way to see duration using it
// how to read from std::in in rust when a program is running eg like creating a TUI

//you could use notify.rs to notify user which track is playing
//visualizers

//add a readme and instruction for use
//maybe play in another thread -> think more about this

//how to handle the actual playback
//add an option to as the user whether to replay the song -> here use loop and break once user says play-next
//if next is pressed then you can run a continue/skip action, when stop is pressed you can do a
//break and previous is just going behind by one index. to prevent out of bounds errors check where you are
//i.e if the index is not below the len and above it also

//the dir we are going to read for the music files
fn current_dir() -> io::Result<String> {
    let dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    Ok(dir)
}

fn mp3_songs_info(dir: &String, song: &String) {
    let tag = Tag::read_from_path(format!("{}/{}", dir, song)).unwrap();
    if let Some(title) = tag.title() {
        println!("title :  {}", title);
    };
    if let Some(artist) = tag.artist() {
        println!("artist:  {}", artist);
    }
    if let Some(album) = tag.album() {
        println!("album :  {}\n", album);
    }
    //duration does not work; FIX IT
    //can rodio be used for song duration
}

pub fn show_songs() {
    let m4a_songs = Songs::list_songs().unwrap().m4a_songs;
    let mp3_songs = Songs::list_songs().unwrap().mp3_songs;
    let dir = current_dir().unwrap();

    println!("{} m4a songs in directory -> {}\n", m4a_songs.len(), dir);
    for (index, song) in m4a_songs.iter().enumerate() {
        println!("{}: {}\n", index, song);
    }
    println!("{} mp3 songs in directory -> {}\n", mp3_songs.len(), dir);
    for (index, song) in mp3_songs.iter().enumerate() {
        println!("{}: {}\n", index, song);
    }
}

//sink has pause
//paying with these controls
pub fn play_fn() {
    let m4a_songs = Songs::list_songs().unwrap().m4a_songs;
    let mp3_songs = Songs::list_songs().unwrap().mp3_songs;
    let dir = current_dir().unwrap();

    //check whether there are any songs in
    if m4a_songs.is_empty() && mp3_songs.is_empty() {
        println!("No songs found in current directory, exiting...");
        process::exit(1);
    }

    //playing the mp3 songs first
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    for song in mp3_songs {
        mp3_songs_info(&dir, &song); //fn to print song metadata

        let file = File::open(format!("{}/{}", dir, song)).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        // sink.pause();//they work
        // std::thread::sleep(std::time::Duration::from_secs(30));
        // sink.play();//they work

        sink.sleep_until_end();
    }

    //playing m4a songs //very cpu intensize
    for song in m4a_songs{
        //info about songs here
        // songs_info(&dir, &song); //fn to print song metadata

        let file = File::open(format!("{}/{}", dir, song)).unwrap();
        let size = file.metadata().expect("error reading file metadata").len();
        let buf = BufReader::new(file);

        let decoder = redlux::Decoder::new_mpeg4(buf, size).expect("error creating m4a decoder");
        let output_stream = rodio::OutputStream::try_default();
        let (_stream, handle) = output_stream.expect("error creating output stream");
        let sink = rodio::Sink::try_new(&handle).expect("error creating sink");
        sink.append(decoder);
        sink.sleep_until_end()
    }
}

// //play from a certain directory
// pub fn play_from(path: &str) {
//     let path_str = Path::new(path);
//     todo!()
// }
