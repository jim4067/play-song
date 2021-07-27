extern crate id3;
extern crate rodio;
extern crate mp4ameta;
extern crate redlux;
extern crate walkdir;

mod app;
mod playback;
mod songs;
mod ui;

fn main() {
    // let thread_one = std::thread::spawn(move || {
    //     let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    //     let sink = rodio::Sink::try_new(&handle).unwrap();
    //     let file = std::fs::File::open("/media/jim_4067/52AE2FD4AE2FAF81/squidward tenticles/Echosmith - over my head/01 Over My Head.m4a").unwrap();
    //     sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).unwrap());
    //     sink.sleep_until_end();
    // });
    // thread_one.join().unwrap();

    // let file = std::fs::File::open("/media/jim_4067/52AE2FD4AE2FAF81/squidward tenticles/Echosmith - over my head/01 Over My Head.m4a").unwrap();
    // let metadata = file.metadata().expect("Error getting file metadata");
    // let buf = std::io::BufReader::new(file);
    // let size = metadata.len();

    // let decoder = redlux::Decoder::new_mpeg4(buf, size).expect("error creating m4a decoder");
    // let output_stream = rodio::OutputStream::try_default();
    // let (_stream, handle) = output_stream.expect("error creating output stream");
    // let sink = rodio::Sink::try_new(&handle).expect("error crating sink");
    // sink.append(decoder);
    // println!("duration {:?}", mp4.duration());
    // sink.sleep_until_end();

    // playback::play_fn();
    let now = std::time::Instant::now();
    // //HERE WHEN RUNNING BENCHES
    // app::run().unwrap();
    ui::init();
    // playback::play_from("/media/jim_4067/52AE2FD4AE2FAF81/squidward tenticles/Echosmith - over my head");
    let elapsed = now.elapsed();
    println!("done after {:?}", elapsed);
}
