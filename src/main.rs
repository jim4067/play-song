extern crate id3;
extern crate mp4ameta;
extern crate redlux;
extern crate rodio;
extern crate walkdir;

mod app;
mod playback;
mod songs;

fn main() {
    app::run().unwrap();
}
