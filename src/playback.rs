use crate::songs::Songs;
use id3::Tag as mp3_tag;
use mp4ameta::Tag as m4a_tag;
use redlux::Decoder;
use rodio;
use std::io;
use std::{env, fs::File, io::BufReader, process};

// find out if there is a way to pause using rodio - there is
//find out if there is a way to flush the std::output after printing to it
// and if there is a way to see duration using it
// how to read from std::in in rust when a program is running eg like creating a TUI

//add an option to as the user whether to replay the song -> here use loop and break once user says play-next
//if next is pressed then you can run a continue/skip action, when stop is pressed you can do a
//break and previous is just going behind by one index. to prevent out of bounds errors check where you are
//i.e if the index is not below the len and above it also

//the dir we are going to read for the music files
fn current_dir() -> io::Result<String> {
    let dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    Ok(dir)
}

pub fn show_songs() {
    let m4a_songs = Songs::list_songs(".").unwrap().m4a_songs;
    let mp3_songs = Songs::list_songs(".").unwrap().mp3_songs;
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

fn mp3_songs_info(dir: &String, song: &String) {
    let tag = mp3_tag::read_from_path(format!("{}/{}", dir, song)).unwrap();
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

fn m4a_songs_info(dir: &String, song: &String) {
    let tag = m4a_tag::read_from_path(format!("{}/{}", dir, song)).unwrap();

    if let Some(title) = tag.title() {
        println!("title :  {}", title);
    };
    if let Some(artist) = tag.artist() {
        println!("artist:  {}", artist);
    }
    if let Some(album) = tag.album() {
        println!("album :  {}\n", album);
    }
}

// fn play_mp3(mp3_songs: Vec<String>, dir: String) {
//     // playing the mp3 songs
//     let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
//     let sink = rodio::Sink::try_new(&handle).unwrap();
//     for song in mp3_songs {
//         mp3_songs_info(&dir, &song); //fn to print song metadata

//         let file = File::open(format!("{}/{}", dir, song)).unwrap();
//         sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

//         // sink.pause();//they work
//         // std::thread::sleep(std::time::Duration::from_secs(30));
//         // sink.play();//they work

//         sink.sleep_until_end();
//     }
// }

fn play_mp3(mp3_songs: Vec<String>, dir: String) {
    // playing the mp3 songs
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    for song in mp3_songs {
        mp3_songs_info(&dir, &song); //fn to print song metadata

        //check whether your are playing from current directory or user specified directory
        let file;

        if dir.contains(".") {
            file = File::open(format!("{}/{}", dir, song)).unwrap();
        } else {
            file = File::open(song).expect("error opening file");
        }
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        // sink.pause();//they work
        // std::thread::sleep(std::time::Duration::from_secs(30));
        // sink.play();//they work

        sink.sleep_until_end();
    }
}

fn play_m4a(m4a_songs: Vec<String>, dir: String) {
    //playing m4a songs
    //info about songs here

    let output_stream = rodio::OutputStream::try_default();
    let (_stream, handle) = output_stream.expect("error creating output stream");
    let sink = rodio::Sink::try_new(&handle).expect("error creating sink");

    for song in m4a_songs {
        m4a_songs_info(&dir, &song);

        let file = File::open(format!("{}/{}", dir, song)).unwrap();
        let size = file.metadata().expect("error reading file metadata").len();
        let buf = BufReader::new(file);
        let decoder = Decoder::new_mpeg4(buf, size).expect("error creating m4a decoder");
        sink.append(decoder);
        sink.sleep_until_end();
    }
}

//main play fn
pub fn play_fn() {
    let m4a_songs = Songs::list_songs(".").unwrap().m4a_songs;
    let mp3_songs = Songs::list_songs(".").unwrap().mp3_songs;
    let dir = current_dir().unwrap();

    //check whether there are any songs in
    if m4a_songs.is_empty() && mp3_songs.is_empty() {
        println!("No songs found in current directory, exiting...");
        process::exit(1);
    } else if !mp3_songs.is_empty() && m4a_songs.is_empty() {
        //if mp3 found and m4a not found
        play_mp3(mp3_songs, dir);
    } else if !m4a_songs.is_empty() && mp3_songs.is_empty() {
        play_m4a(m4a_songs, dir);
    } else {
        //very cpu intensive
        play_m4a(m4a_songs, dir.clone());
        play_mp3(mp3_songs, dir);
    }
}

//play from a certain directory
pub fn play_from(path_str: &str) {
    let mp3_songs = Songs::list_songs(path_str).unwrap().mp3_songs;
    let m4a_songs = Songs::list_songs(path_str).unwrap().m4a_songs;
    // let path = path_str.to_string();
    let dir = "".to_string();

    if m4a_songs.is_empty() && mp3_songs.is_empty() {
        println!("No songs found in current directory, exiting...");
        process::exit(1);
    } else if !mp3_songs.is_empty() && m4a_songs.is_empty() {
        //if mp3 found and m4a not found
        play_mp3(mp3_songs, dir);
    } else if !m4a_songs.is_empty() && mp3_songs.is_empty() {
        m4a_playing_from(m4a_songs);
    } //else {
      //     //very cpu intensive
      //     play_m4a(m4a_songs, dir.clone());
      //     play_mp3(mp3_songs, dir);
      // }
}

fn m4a_playing_from(songs: Vec<String>) {
    let dir = "".to_string(); //

    for song in songs {
        m4a_songs_info(&dir, &song);
        let file = File::open(song).expect("Error opening the file");
        let size = file.metadata().expect("error reading metadata").len();
        let buf = BufReader::new(file);

        let decoder = redlux::Decoder::new_mpeg4(buf, size).expect("Error creating m4a decoder");
        let output_stream = rodio::OutputStream::try_default();
        let (_stream, handle) = output_stream.expect("error creating output stream");
        let sink = rodio::Sink::try_new(&handle).expect("error creating sink");

        sink.append(decoder);
        sink.sleep_until_end();
    }
}

fn mp3_playing_from(songs: Vec<String>) {
    let dir = "".to_string(); //
}
