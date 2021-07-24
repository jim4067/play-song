use crate::songs;
use id3::Tag;
use rodio;
use std::io;
use std::{env, fs::File, io::BufReader, process};

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

fn songs_info(dir: &String, song: &String) {
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

//function to print songs in the directory
pub fn show_songs() {
    let songs = songs::list_songs().unwrap();
    let dir = current_dir().unwrap();

    for (index, song) in songs.iter().enumerate() {
        println!("{}: {}\n", index, song);
    }
    println!("{} mp3 songs in directory -> {}\n", songs.len(), dir);
}

//sink has pause
//paying with these controls
pub fn play_fn() {
    let songs = songs::list_songs().unwrap();

    if songs.is_empty() {
        println!(" no mp3 songs found");
        process::exit(1);
    } //write a test for this

    println!(
        "Playing {} songs in this dir {}\n",
        songs.len(),
        current_dir().unwrap()
    );
    //add playback instruction above here
    //to repeat use r, to quite or stop use q, prev -> p, next -> n

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let dir = current_dir().unwrap();

    for song in songs {
        songs_info(&dir, &song); //fn to print song metadata

        let file = File::open(format!("{}/{}", dir, song)).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        // sink.pause();//they work
        // std::thread::sleep(std::time::Duration::from_secs(30));
        // sink.play();//they work

        sink.sleep_until_end();
    }
}
