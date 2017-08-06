extern crate sdl2;

use self::sdl2::AudioSubsystem;
use self::sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

pub struct Tone {
    enabled: bool
}

impl AudioCallback for Tone {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            *sample = if self.enabled { 1.0 } else { 0.0 };
        }
    }
}

pub fn create_audio_device(audio: &AudioSubsystem) -> AudioDevice<Tone> {
    let spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None
    };

    audio.open_playback(None, &spec,
        |_| {
            Tone {
                enabled: true
            }
        }
    ).unwrap()
}
