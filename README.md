<!-- ![play-song](Screenshot/1.png) -->
# PLAY-SONG

## Description

play-song is a cli app that plays mp3 and m4a songs from the current directory or a specified directory. For uninterrupted music playback on the terminal.

## Tech stack 💻

This is a [rust](https://www.rust-lang.org/) project and I am using the following crates.

-   [rodeo](https://github.com/RustAudio/rodio) for decoding and playing the mp3 files.
-   [redlux](https://github.com/probablykasper/redlux/) for decoding and playing the m4a files.
-   [id3](https://github.com/polyfloyd/rust-id3) for reading the mp3 files' metadata.
-   [mp4ameta](https://github.com/Saecki/rust-mp4ameta) for reading the m4a files' metadata.
-   [walkdir](https://github.com/BurntSushi/walkdir) for traversing the file system and finding mp3 and m4a files.
-   [clap](https://clap.rs/) for parsing command line arguments.

## Source data 📝

I used local files on my computer

-   [local files]()

## What I learned?

-  Traversing the Linux file system.
-  what Id3 metadata for music files is and how to read it.

## Going forward?
- I'll take a deeper dive into the world of Linux and audio and get to better understand how ALSA, pulseaudio and the clients _sound producers_ all work together.
- I'll learn how to write audio decoders for media file and learn how different audio file formats store media data.
- I'll learn how to add playback controls to the player, use async rust features to prevent the blocking behaviour encountered when only working with one thread.  

## Future Improvements
- Instead of reading file extensions to determine the type of file, I could use the file metadata.
- Use the rust native path type instead of a String for file paths.
## Project setup

Download the compiled binary package from the [releases page](https://github.com/jim4067/play-song/releases/download/0.0.1/play-song). 

Jimmy ©2021
