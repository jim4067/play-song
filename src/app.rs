use crate::playback;
use clap::{App, Arg};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = App::new("play-song")
        .version("0.0.1")
        .about("Play the songs in a certain directory")
        .arg(
            Arg::with_name("play")
                .short("p")
                .long("play-all")
                .help("default, plays all the mp3 files in directory user is in"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list-songs")
                .help("Lists all songs in the directory you are in"), // .takes_value(true),
        )
        .arg(
            Arg::with_name("play-from")
                // .short("l")
                .long("play-from")
                .takes_value(true)
                .help("Play the songs in a specified directory")
        )
        .get_matches();

    //getting the value for play from

    if cli.is_present("list") {
        playback::show_songs();
    } else if cli.is_present("play-from") {
    let input = cli.value_of("play-from").unwrap();
        playback::play_from(input);
    } else {
        playback::play_fn();
    }
    Ok(())
}
