mod audio;
mod sub_title;
mod video;

use audio::AudioDecoder;
use sub_title::SubtitleProcessor;
use video::VideoDecoder;

/// The Facade that simplifies access to complex subsystems.
pub struct MediaPlayer {
    audio: AudioDecoder,
    video: VideoDecoder,
    subtitle: SubtitleProcessor,
}

impl Default for MediaPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaPlayer {
    pub fn new() -> Self {
        MediaPlayer {
            audio: AudioDecoder::new(),
            video: VideoDecoder::new(),
            subtitle: SubtitleProcessor::new(),
        }
    }

    /// The single, simplified API for playing media.
    pub fn play(&self) {
        println!("Starting media player...");
        self.audio.decode();
        self.video.decode();
        self.subtitle.load();
        println!("Playback complete.");
    }
}
