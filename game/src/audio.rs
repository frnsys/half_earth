//! Two different audio "engines" here; on web we still use JS as it uses
//! the browser's optimized decoding so audio files load much faster
//! and it can load the audio files asynchronously so the WASM size is much smaller.
//!
//! [`AudioSystem`] provides a unified interface over both implementations.

use std::sync::{OnceLock, mpsc};

enum AudioRequest {
    Mute,
    Unmute,
    Ping,
    Loop(Track),
}

static AUDIO_TX: OnceLock<mpsc::Sender<AudioRequest>> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
mod engine {
    use super::{AudioRequest, Track};
    use kira::{
        AudioManager, AudioManagerSettings, Decibels, DefaultBackend, PlaySoundError, Tween,
        sound::{
            SoundData,
            static_sound::{StaticSoundData, StaticSoundHandle},
        },
    };

    macro_rules! audio {
        ($path:literal) => {
            kira::sound::static_sound::StaticSoundData::from_cursor(std::io::Cursor::new(
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/sounds/",
                    $path
                )),
            ))
            .unwrap()
        };
    }

    pub struct Engine {
        muted: bool,
        manager: Option<AudioManager<DefaultBackend>>,
        handle: Option<StaticSoundHandle>,
    }
    impl Engine {
        pub fn new(muted: bool) -> Self {
            Self {
                muted,
                manager: None,
                handle: None,
            }
        }

        fn load_track(&self, track: Track) -> StaticSoundData {
            match track {
                Track::Intro => {
                    audio!("music/intro.ogg")
                }
                Track::Planning => {
                    audio!("music/planning.ogg")
                }
                Track::Interstitial => {
                    audio!("music/city_noise.ogg")
                }
                Track::ReportBad => {
                    audio!("music/report_bad.ogg")
                }
                Track::ReportGood => {
                    audio!("music/report_good.ogg")
                }
            }
        }

        fn play_or_queue_track(
            &mut self,
            track: Track,
        ) -> Result<(), PlaySoundError<<StaticSoundData as SoundData>::Error>> {
            if !self.muted {
                let sound = self.load_track(track);
                if let Some(handle) = &mut self.handle {
                    handle.stop(Tween::default());
                }
                if let Some(manager) = &mut self.manager {
                    let handle = manager.play(sound.loop_region(0.0..))?;
                    self.handle = Some(handle);
                }
            }
            Ok(())
        }

        fn mute(&mut self) {
            if let Some(manager) = &mut self.manager {
                manager
                    .main_track()
                    .set_volume(Decibels::SILENCE, Tween::default());
                self.muted = true;
            }
        }

        fn unmute(&mut self) {
            if let Some(manager) = &mut self.manager {
                manager.main_track().set_volume(0., Tween::default());
                self.muted = false;
            }
        }

        fn ping(&mut self) -> Result<(), PlaySoundError<<StaticSoundData as SoundData>::Error>> {
            if !self.muted
                && let Some(manager) = &mut self.manager
            {
                let sound = audio!("notification.ogg");
                let handle = manager.play(sound)?;
                self.handle = Some(handle);
            }
            Ok(())
        }

        pub fn handle_request(
            &mut self,
            request: AudioRequest,
        ) -> Result<(), PlaySoundError<<StaticSoundData as SoundData>::Error>> {
            if self.manager.is_none() {
                let manager =
                    AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
                self.manager = Some(manager);
                if self.muted {
                    self.mute();
                }
            }
            match request {
                AudioRequest::Mute => {
                    self.mute();
                }
                AudioRequest::Unmute => {
                    self.unmute();
                }
                AudioRequest::Ping => {
                    self.ping()?;
                }
                AudioRequest::Loop(track) => {
                    self.play_or_queue_track(track)?;
                }
            }
            Ok(())
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod engine {
    use super::{AudioRequest, Track};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/assets/js/audio.js")]
    extern "C" {
        type AudioManager;

        fn get_audio_manager() -> AudioManager;

        #[wasm_bindgen(method, js_name = startSoundtrack)]
        fn start_soundtrack(this: &AudioManager, file: &str, fade: bool);

        #[wasm_bindgen(method, js_name = stopSoundtrack)]
        fn stop_soundtrack(this: &AudioManager, fade: bool);

        #[wasm_bindgen(method, js_name = playOneShot)]
        fn play_one_shot(this: &AudioManager, file: &str);

        #[wasm_bindgen(method)]
        fn mute(this: &AudioManager);

        #[wasm_bindgen(method)]
        fn unmute(this: &AudioManager);
    }

    pub struct Engine {
        muted: bool,
    }
    impl Engine {
        pub fn new(muted: bool) -> Self {
            Self { muted }
        }

        pub fn handle_request(&mut self, request: AudioRequest) -> Result<(), String> {
            if self.muted {
                get_audio_manager().mute();
            }

            match request {
                AudioRequest::Mute => {
                    self.muted = true;
                    get_audio_manager().mute();
                }
                AudioRequest::Unmute => {
                    self.muted = false;
                    get_audio_manager().unmute();
                }
                AudioRequest::Ping => {
                    get_audio_manager().play_one_shot("/sounds/notification.ogg");
                }
                AudioRequest::Loop(track) => {
                    let url = match track {
                        Track::Intro => "/sounds/music/intro.ogg",
                        Track::Planning => "/sounds/music/planning.ogg",
                        Track::Interstitial => "/sounds/music/city_noise.ogg",
                        Track::ReportBad => "/sounds/music/report_bad.ogg",
                        Track::ReportGood => "/sounds/music/report_good.ogg",
                    };
                    let fade = false;
                    let manager = get_audio_manager();
                    manager.stop_soundtrack(fade);
                    manager.start_soundtrack(url, fade);
                }
            }
            Ok(())
        }
    }
}

pub struct AudioSystem {
    request_rx: mpsc::Receiver<AudioRequest>,
    engine: engine::Engine,
}
impl AudioSystem {
    pub fn new(muted: bool) -> Self {
        let (tx, rx) = mpsc::channel::<AudioRequest>();

        AUDIO_TX.get_or_init(|| tx);

        Self {
            request_rx: rx,
            engine: engine::Engine::new(muted),
        }
    }

    pub fn update(&mut self) -> Result<(), String> {
        while let Ok(req) = self.request_rx.try_recv() {
            self.engine
                .handle_request(req)
                .map_err(|err| err.to_string())?;
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

#[derive(Clone, Copy)]
pub enum Track {
    Intro,
    Planning,
    Interstitial,
    ReportBad,
    ReportGood,
}
