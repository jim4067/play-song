extern crate id3;
extern crate rodio;
extern crate walkdir;

mod app;
mod playback;
mod songs;

fn main() {
    playback::play_fn();
}
