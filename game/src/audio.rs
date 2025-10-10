use std::sync::{OnceLock, mpsc};

use kira::{
    AudioManager,
    AudioManagerSettings,
    Decibels,
    DefaultBackend,
    PlaySoundError,
    Tween,
    sound::{
        SoundData,
        static_sound::{StaticSoundData, StaticSoundHandle},
    },
};

enum AudioRequest {
    Mute,
    Unmute,
    Ping,
    Loop(Track),
}

static AUDIO_TX: OnceLock<mpsc::Sender<AudioRequest>> =
    OnceLock::new();

macro_rules! audio {
    ($path:literal) => {
        kira::sound::static_sound::StaticSoundData::from_cursor(
            std::io::Cursor::new(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/sounds/",
                $path
            ))),
        )
        .unwrap()
    };
}

pub struct AudioSystem {
    request_rx: mpsc::Receiver<AudioRequest>,
    manager: AudioManager<DefaultBackend>,
    handle: Option<StaticSoundHandle>,
    muted: bool,
}
impl AudioSystem {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<AudioRequest>();

        AUDIO_TX.get_or_init(|| tx);

        Self {
            manager: AudioManager::<DefaultBackend>::new(
                AudioManagerSettings::default(),
            )
            .unwrap(),
            request_rx: rx,
            muted: false,
            handle: None,
        }
    }

    pub fn update(
        &mut self,
    ) -> Result<
        (),
        PlaySoundError<<StaticSoundData as SoundData>::Error>,
    > {
        while let Ok(req) = self.request_rx.try_recv() {
            match req {
                AudioRequest::Mute => {
                    self.manager.main_track().set_volume(
                        Decibels::SILENCE,
                        Tween::default(),
                    );
                    self.muted = true;
                }
                AudioRequest::Unmute => {
                    self.manager
                        .main_track()
                        .set_volume(0., Tween::default());
                    self.muted = false;
                }
                AudioRequest::Ping => {
                    if !self.muted {
                        let sound = audio!("notification.ogg");
                        let handle =
                            self.manager.play(sound)?;
                        self.handle = Some(handle);
                    }
                }
                AudioRequest::Loop(track) => {
                    if !self.muted {
                        let sound = match track {
                            Track::Intro => {
                                audio!("music/intro.ogg")
                            }
                            Track::Planning => {
                                audio!("music/planning.ogg")
                            }
                            Track::Interstitial => {
                                audio!("city_noise.ogg")
                            }
                            Track::ReportBad => {
                                audio!("music/report_bad.ogg")
                            }
                            Track::ReportGood => {
                                audio!("music/report_good.ogg")
                            }
                        };
                        if let Some(handle) = &mut self.handle {
                            handle.stop(Tween::default());
                        }
                        let handle = self
                            .manager
                            .play(sound.loop_region(0.0..))?;
                        self.handle = Some(handle);
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn mute() {
    AUDIO_TX.get().unwrap().send(AudioRequest::Mute).unwrap();
}

pub fn unmute() {
    AUDIO_TX.get().unwrap().send(AudioRequest::Unmute).unwrap();
}

pub fn ping() {
    AUDIO_TX.get().unwrap().send(AudioRequest::Ping).unwrap();
}

pub fn soundtrack(track: Track) {
    AUDIO_TX
        .get()
        .unwrap()
        .send(AudioRequest::Loop(track))
        .unwrap();
}

pub enum Track {
    Intro,
    Planning,
    Interstitial,
    ReportBad,
    ReportGood,
}
