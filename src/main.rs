use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let sink_handle = DeviceSinkBuilder::open_default_sink()
        .expect("Default audio device not found.");

    let source = SineWave::new(20.0).amplify(0.0005);

    sink_handle.mixer().add(source);

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}