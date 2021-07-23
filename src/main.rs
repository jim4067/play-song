extern crate rodio;
extern crate walkdir;
extern crate id3; 

mod app;
mod playback;
mod songs;

fn main() {
    // let thread_one = std::thread::spawn(move || {
    //     let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    //     let sink = rodio::Sink::try_new(&handle).unwrap();
    //     let file = File::open("/media/jim_4067/52AE2FD4AE2FAF81/squidward tenticles/Sabrina Carpenter – Singular Act 1 (2018)/Prfct.mp3").unwrap();
    //     sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    //     sink.sleep_until_end();

    //     play_fn()
    // });
    playback::play_fn();
}

//how to structure this thing
//the fn to play the songs
//the fn to list the songs
//the fn for the metadata
