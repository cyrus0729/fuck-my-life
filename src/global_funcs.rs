pub fn play_sound(_audio: &'static str) {
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, source::Source};
    use std::thread;

    // new thread for audio playback
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = File::open("assets/sfxOw.mp3").unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        let _ = stream_handle.play_raw(source.convert_samples());
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
}