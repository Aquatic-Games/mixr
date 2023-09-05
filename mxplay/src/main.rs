use clap::Parser;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use mixr::{AudioBuffer, AudioSystem, BufferDescription, PlayProperties, PlayState};
use mixr::stream::{AudioStream, Stream};

#[derive(Parser)]
struct CliArgs {
    #[arg(required = true)]
    paths: Vec<String>,

    #[arg(short, long, default_value_t = 1.0)]
    volume: f64,

    #[arg(short, long, default_value_t = 1.0)]
    speed: f64,

    //#[arg(long, default_value_t = false)]
    //repeat: bool
}

fn main() {
    /*let paths = vec![
        "/home/skye/Music/house2.ogg".to_string(),
        "/home/skye/Music/SCD/1-20 Quartz Quadrant 'G' Mix JP.wav".to_string()
    ];*/

    let args = CliArgs::parse();

    let desired = AudioSpecDesired {
        freq: Some(SAMPLE_RATE as i32),
        channels: Some(2),
        samples: Some(8192),
    };

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let device = audio.open_playback(None, &desired, |_| {
        Audio::new(args)
    }).unwrap();

    device.resume();

    ctrlc::set_handler(|| {
        println!();
        std::process::exit(0);
    }).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

const SAMPLE_RATE: u32 = 48000;
const NUM_BUFFERS: usize = 2;
const BUFFER_SIZE: usize = 48000;
const VOICE: u16 = 0;

struct Audio {
    system: AudioSystem,
    paths: Vec<String>,

    play_properties: PlayProperties,

    buffer: AudioBuffer,

    current_sound: usize
}

impl Audio {
    pub fn new(args: CliArgs) -> Self {
        let paths = args.paths;

        if paths.is_empty() {
            panic!("Empty path provided.");
        }

        let play_properties = PlayProperties {
            volume: args.volume,
            speed: args.speed,
            ..Default::default()
        };

        let mut system = AudioSystem::new(SAMPLE_RATE, 1);

        let mut stream = Stream::from_file(paths[0].as_str()).unwrap();
        let format = stream.format();
        let pcm = stream.get_pcm().unwrap();

        let buffer = system.create_buffer(BufferDescription { format }, Some(&pcm)).unwrap();
        system.play_buffer(buffer, VOICE, play_properties).unwrap();

        Self {
            system,
            paths,

            play_properties,

            buffer,

            current_sound: 0
        }
    }
}

impl AudioCallback for Audio {
    type Channel = f32;

    fn callback(&mut self, buf: &mut [Self::Channel]) {
        self.system.read_buffer_stereo_f32(buf);

        // !! TERRIBLE !!
        // TODO: Proper audio streaming
        // TODO: Threading
        if self.system.get_voice_state(VOICE).unwrap() == PlayState::Stopped {
            self.current_sound += 1;

            // TODO: Don't do this utter garbage!!!
            if self.current_sound >= self.paths.len() {
                std::process::exit(0);
            }

            let path = &self.paths[self.current_sound];
            let mut stream = Stream::from_file(path.as_str()).unwrap();
            let format = stream.format();
            let pcm = stream.get_pcm().unwrap();

            self.system.update_buffer(self.buffer, format, Some(&pcm)).unwrap();
            self.system.play_buffer(self.buffer, VOICE, self.play_properties).unwrap();
        }
    }
}

unsafe impl Send for Audio { }