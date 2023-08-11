use std::collections::VecDeque;

use clap::Parser;
use crossterm::{*, style::Stylize};
use mixr::{AudioSystem, BufferDescription, PlayProperties, PlayState, stream::{Wav, AudioStream}, AudioBuffer};
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
    println!("mxplay 0.1.0\npiegfx 2023\n\n\n\n");

    /*let mut args = CliArgs::parse();
    let speed = args.speed;
    let volume = args.volume;
    let looping = args.looping;*/

    //let files = args.paths;
    let files = vec!["/home/skye/Music/S3D/03 Richard Jacques - Green Grove, Act One.wav".to_string()];

    let mut current_track = 0;

    /*let first_file = files.get(current_track).unwrap();
    let mut wav = if let Ok(wav) = Wav::from_file(&first_file) {
        wav
    } else {
        println!("Could not find file with path \"{}\"!", first_file);
        return;
    };*/

    /*execute!(
        stdout(),
        cursor::Hide
    ).unwrap();*/

    //terminal::enable_raw_mode().unwrap();

    //let pcm = wav.get_pcm().unwrap();

    let mut system = AudioSystem::new(48000, 1);

    /*let buffer = system.create_buffer(BufferDescription {
        format: wav.format()
    }, Some(&pcm)).unwrap();

    let properties = PlayProperties {
        speed,
        volume,
        looping,
        ..Default::default()
    };*/

    const VOICE: u16 = 0;

    //system.play_buffer(buffer, VOICE, properties).unwrap();

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(2),
        samples: Some(8192),
    };

    let mut device = audio.open_playback(None, &spec, |_| {
        Audio::new(system, files)
    }).unwrap();

    device.resume();

    loop {
        /*let event = if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            Some(event::read().unwrap())
        } else {
            None
        };*/

        let mut guard = device.lock();
        let system = &mut guard.system;

        /*let curr_secs = system.get_position(VOICE).unwrap() as usize;
        let state = system.get_voice_state(VOICE).unwrap();

        let mut go_back = false;

        if let Some(event) = event {
            match event {
                event::Event::Key(k) => {
                    match k.code {
                        event::KeyCode::Char('p') => {
                            let state = if state == PlayState::Playing {
                                PlayState::Paused
                            } else {
                                PlayState::Playing
                            };
                            
                            system.set_voice_state(VOICE, state).unwrap();
                        },

                        event::KeyCode::Char('q') => { break; },

                        event::KeyCode::Char('c') if k.modifiers == event::KeyModifiers::CONTROL => {
                            break;
                        },

                        event::KeyCode::Char('n') => {
                            system.set_voice_state(VOICE, PlayState::Stopped).unwrap();
                        },

                        event::KeyCode::Char('v') => {
                            if curr_secs >= 1 {
                                system.set_position_samples(VOICE, 0).unwrap();
                            } else {
                                go_back = true;
                                system.set_voice_state(VOICE, PlayState::Stopped).unwrap();
                            }
                        },

                        event::KeyCode::Char('f') => {
                            let position = system.get_position(VOICE).unwrap();
                            system.set_position(VOICE, position + 0.5).unwrap();
                        },

                        event::KeyCode::Char('d') => {
                            let position = system.get_position(VOICE).unwrap();
                            system.set_position(VOICE, f64::max(position - 0.5, 0.0)).unwrap();
                        },

                        event::KeyCode::Char('u') => {
                            let mut props = system.get_play_properties(VOICE).unwrap();
                            props.speed += 0.01;

                            system.set_play_properties(VOICE, props).unwrap();
                        },

                        event::KeyCode::Char('y') => {
                            let mut props = system.get_play_properties(VOICE).unwrap();
                            props.speed -= 0.01;

                            system.set_play_properties(VOICE, props).unwrap();
                        },

                        _ => {}
                    }
                },

                _ => {}
            }
        }

        let (cols, rows) = terminal::size().unwrap();

        queue!(
            stdout(),

            cursor::MoveTo(0, rows - 7),
            terminal::Clear(terminal::ClearType::FromCursorDown),

            style::Print("mxplay 0.1.0"),
            cursor::MoveTo(0, rows - 6),
            style::Print("piegfx 2023"),

            cursor::MoveTo(0, rows - 4),
            style::Print(format!("{state:?}    ")),
            cursor::MoveTo(0, rows - 3),
            style::Print(format!("{:0>2}:{:0>2}:{:0>2}", curr_secs / 60 / 60, (curr_secs / 60) % 60, curr_secs % 60)),

            cursor::MoveTo(0, rows - 1),

            style::PrintStyledContent(style::style("Q").underlined()),
            style::Print("uit "),

            style::PrintStyledContent(style::style("P").underlined()),
            style::Print("ause "),

            style::PrintStyledContent(style::style("N").underlined()),
            style::Print("ext "),

            style::Print("Pre"),
            style::PrintStyledContent(style::style("v").underlined()),
            style::Print("ious "),

            style::PrintStyledContent(style::style("F").underlined()),
            style::Print("Forward "),

            style::Print("Rewin"),
            style::PrintStyledContent(style::style("d").underlined()),
            style::Print(" "),
        ).unwrap();
        
        stdout().flush().unwrap();

        if system.get_voice_state(VOICE).unwrap() == PlayState::Stopped {
            if go_back {
                if current_track != 0 {
                    current_track -= 1;
                }
            } else {
                current_track += 1;
            }

            /*let path = if let Some(f) = files.get(current_track) {
                f
            } else {
                break;
            };

            let mut wav = if let Ok(wav) = Wav::from_file(&path) {
                wav
            } else {
                //println!("Could not load file with path \"{}\"!", path);
                continue;
            };

            system.update_buffer(buffer, wav.format(), Some(&wav.get_pcm().unwrap())).unwrap();
            system.play_buffer(buffer, VOICE, properties).unwrap();*/
        }*/
    }

    /*terminal::disable_raw_mode().unwrap();
    println!();
    execute!(
        stdout(),
        cursor::Show,
    ).unwrap();*/
}

const NUM_BUFFERS: usize = 100;
const BUFFER_SIZE: usize = 24000;
const VOICE: u16 = 0;

struct Audio {
    system: AudioSystem,

    files: Vec<String>,

    buffers: Vec<AudioBuffer>,
    curr_buffer: usize,
    buffer_data: Vec<u8>,

    current_stream: Box<dyn AudioStream>
}

impl Audio {
    pub fn new(mut system: AudioSystem, files: Vec<String>) -> Self {
        let mut buffers = Vec::with_capacity(NUM_BUFFERS);

        let mut buffer_data = Vec::with_capacity(BUFFER_SIZE);
        // force the vec to have the correct length as we will be overwriting the data immediately.
        unsafe { buffer_data.set_len(BUFFER_SIZE) };

        let mut current_stream = Wav::from_file(files[0].as_str()).unwrap();

        for _ in 0..NUM_BUFFERS {
            current_stream.get_buffer(&mut buffer_data).unwrap();

            let buffer = system.create_buffer(BufferDescription { format: current_stream.format() }, Some(&buffer_data)).unwrap();
            buffers.push(buffer);
        }

        system.play_buffer(buffers[0], VOICE, PlayProperties::default()).unwrap();
        for i in 1..NUM_BUFFERS {
            system.queue_buffer(buffers[i], VOICE).unwrap();
        }

        Self {
            system,
            files,
            buffers,
            buffer_data,
            curr_buffer: 0,
            current_stream: Box::new(current_stream)
        }
    }
}

impl AudioCallback for Audio {
    type Channel = f32;

    fn callback(&mut self, buf: &mut [Self::Channel]) {
        self.system.read_buffer_stereo_f32(buf);
    }
}
