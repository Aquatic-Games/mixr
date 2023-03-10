# mixr
Mixr, an audio mixer and sound engine.

### mixr
A standalone PCM audio mixer. Needs to be plugged into some sort of audio callback, such as SDL's. Supports PCM audio only, with a few additions such as native IMA ADPCM support (coming soon!).

### mxload
Various audio file loaders, such as Wav, Vorbis (coming soon), MP3 (coming soon), and FLAC (coming soon).

Provides both streaming buffers and conversion to PCM.

### mxengine (coming soon)
A sound engine using mixr as the base. Features automatic streaming using `mxload`, and provides a ready-to-use audio device using SDL. Also provides a clean, no hassle sound system, so you can just get on and play sounds!

### mxplay
A standalone CLI audio player.