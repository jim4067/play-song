use crate::playback;
use clap::{App, Arg};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = App::new("play-song")
        .version("0.1.0")
        .about("Play all mp3 file in the directory which you are in")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list-mp3")
                .help("list all the available mp3 files in the directory you are in"), // .takes_value(true),
        )
        .get_matches();

    if cli.is_present("list") {
        playback::show_songs();
    } else {
        playback::play_fn();
    }
    Ok(())
}
