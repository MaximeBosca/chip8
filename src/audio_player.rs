use sdl3::Sdl;
use sdl3::audio::{AudioCallback, AudioFormat, AudioSpec, AudioStream, AudioStreamWithCallback};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

pub struct AudioPlayer {
    audio_device: AudioStreamWithCallback<SquareWave>,
}

impl AudioCallback<f32> for SquareWave {
    fn callback(&mut self, stream: &mut AudioStream, requested: i32) {
        let mut out = Vec::<f32>::with_capacity(requested as usize);
        // Generate a square wave
        for _ in 0..requested {
            out.push(if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            });
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
        stream.put_data_f32(&out).expect("Can't play audio !");
    }
}

impl AudioPlayer {
    pub fn new(sdl_context: &Sdl) -> AudioPlayer {
        let audio_subsystem = sdl_context.audio().unwrap();

        let source_freq = 44100;
        let source_spec = AudioSpec {
            freq: Some(source_freq),
            channels: Some(1),                    // mono
            format: Some(AudioFormat::f32_sys()), // floating 32 bit samples
        };
        let device = audio_subsystem
            .open_playback_stream(
                &source_spec,
                SquareWave {
                    phase_inc: 440.0 / source_freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                },
            )
            .unwrap();
        Self {
            audio_device: device,
        }
    }

    pub fn play_sound(&self) {
        self.audio_device.resume().expect("Can't resume audio !");
    }

    pub fn pause_sound(&self) {
        self.audio_device.pause().expect("Can't resume audio !");
    }
}
