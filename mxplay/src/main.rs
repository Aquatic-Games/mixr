use std::collections::VecDeque;

use clap::Parser;
use crossterm::*;
use mixr::{AudioSystem, BufferDescription, PlayProperties, PlayState, stream::{Wav, AudioStream}};
use sdl2::audio::{AudioSpecDesired, AudioCallback};
use std::io::{stdout, Write};

#[derive(Parser)]
struct CliArgs {
    #[arg(required = true)]
    paths: Vec<String>,

    #[arg(short, long, default_value_t = 1.0)]
    volume: f64,

    #[arg(short, long, default_value_t = 1.0)]
    speed: f64,

    #[arg(long, default_value_t = false)]
    looping: bool
}

fn main() {
    println!("mxplay 0.1.0\npiegfx 2023");

    let args = CliArgs::parse();
    let speed = args.speed;
    let volume = args.volume;
    let looping = args.looping;

    let mut files = VecDeque::from(args.paths);

    let first_file = files.pop_front().unwrap();
    let mut wav = if let Ok(wav) = Wav::from_file(&first_file) {
        wav
    } else {
        println!("Could not find file with path \"{}\"!", first_file);
        return;
    };

    execute!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();

    terminal::enable_raw_mode().unwrap();

    let pcm = wav.get_pcm().unwrap();

    let mut system = AudioSystem::new(48000, 1);

    let buffer = system.create_buffer(BufferDescription {
        format: wav.format()
    }, Some(&pcm)).unwrap();

    let properties = PlayProperties {
        speed,
        volume,
        looping,
        ..Default::default()
    };

    system.play_buffer(buffer, 0, properties).unwrap();

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(2),
        samples: Some(8192),
    };

    let mut device = audio.open_playback(None, &spec, |_| {
        Audio {
            system
        }
    }).unwrap();

    device.resume();

    ctrlc::set_handler(|| {
        execute!(
            stdout(),
            cursor::Show
        ).unwrap();
        println!(); // print a newline
        std::process::exit(0);
    }).unwrap();

    loop {
        let event = if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            Some(event::read().unwrap())
        } else {
            None
        };

        let mut guard = device.lock();
        let system = &mut guard.system;

        let curr_secs = system.get_position(0).unwrap() as usize;
        let state = system.get_voice_state(0).unwrap();

        if let Some(event) = event {
            match event {
                event::Event::Key(k) => {
                    match k.code {
                        event::KeyCode::Char('p') => {
                            system.set_voice_state(0, if state == PlayState::Playing { PlayState::Paused } else { PlayState::Playing }).unwrap();
                        },

                        event::KeyCode::Char('q') => { break; },

                        _ => {}
                    }
                },

                _ => {}
            }
        }

        queue!(
            stdout(),
            cursor::MoveTo(0, 0),
            style::Print(format!("{state:?} {:0>2}:{:0>2}:{:0>2}", curr_secs / 60 / 60, (curr_secs / 60) % 60, curr_secs % 60))
        ).unwrap();
        
        stdout().flush().unwrap();

        if system.get_voice_state(0).unwrap() == PlayState::Stopped {
            let path = if let Some(f) = files.pop_front() {
                f
            } else {
                break;
            };

            let mut wav = if let Ok(wav) = Wav::from_file(&path) {
                wav
            } else {
                println!("Could not find file with path \"{}\"!", path);
                break;
            };

            system.update_buffer(buffer, wav.format(), Some(&wav.get_pcm().unwrap())).unwrap();
            system.play_buffer(buffer, 0, properties).unwrap();
        }
    }

    terminal::disable_raw_mode().unwrap();
    println!();
    execute!(
        stdout(),
        cursor::Show,
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();
}

struct Audio {
    system: AudioSystem
}

impl AudioCallback for Audio {
    type Channel = f32;

    fn callback(&mut self, buf: &mut [Self::Channel]) {
        self.system.read_buffer_stereo_f32(buf);
    }
}
