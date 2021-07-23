use crate::songs;
use id3::{Tag};
use rodio;
use std::io;
use std::{env, fs::File, io::BufReader};

//the dir we are going to read for the music files
fn current_dir() -> io::Result<String> {
    let dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    Ok(dir)
}

fn songs_info(dir: &String, song: &String) {
    let tag = Tag::read_from_path(format!("{}/{}", dir, song)).unwrap();
    if let Some(title) = tag.title() {
        println!("title :  {}", title);
    };
    if let Some(artist) = tag.artist() {
        println!("artist:  {}", artist);
    }
    if let Some(album) = tag.album() {
        println!("album :  {}", album);
    }
    //duration does not work; FIX IT
    //can rodio be used for song duration
    //stats for nerds:[time / dur of song]
    println!();
}

pub fn play_fn() {
    //maybe for this function for listing i could spawn a new thread and run it there to learn more about async
    let songs = songs::list_songs().unwrap();
    println!(
        "Playing {} songs in this dir {}\n",
        songs.len(),
        current_dir().unwrap()
    ); //this was added just for fun

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let dir = current_dir().unwrap();

    for song in songs {
        songs_info(&dir, &song); //fn to print song metadata

        let file = File::open(format!("{}/{}", dir, song)).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        sink.sleep_until_end();
    }
}
