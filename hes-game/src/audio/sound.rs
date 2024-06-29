use leptos::*;
use leptos_animation::*;

pub struct Sound {
    audio: web_sys::HtmlAudioElement,
    get_volume: ReadSignal<f64>,
    set_volume: WriteSignal<f64>,
    volume_anim: AnimatedSignal<f64, f64>,
}
impl Sound {
    pub fn new(url: &str) -> Self {
        // Update these to start animating to a new thing
        let (get_volume_target, set_volume_target) = create_signal(1.);
        let (get_duration, set_duration) = create_signal(1.);
        // AnimationMode::ReplaceOrStart // DO this

        let (get_volume, set_volume) = create_signal(1.);
        let volume_anim = create_animated_signal(move || get_volume.get().into(), tween_default);
        let audio = web_sys::HtmlAudioElement::new_with_src(url).unwrap();
        let aud = audio.clone();
        create_effect(move |_| {
            aud.set_volume(volume_anim.get());

            if volume_anim.get() == get_volume_target.get() {
                // TODO execute callback?
            }
        });

        Sound {
            audio,
            get_volume,
            set_volume,
            volume_anim,
        }
    }

    pub fn play(&self, should_loop: bool) {
        self.audio.set_loop(should_loop);
        self.audio.play();
    }

    pub fn set_volume(&self, val: f32) {
        let val = val.max(0.).min(1.) as f64;
        self.set_volume.set(val);
    }

    pub fn pause(&self) {
        self.audio.pause();
    }

    pub fn reset(&self) {
        self.audio.load();
    }

    pub fn fade_in(&self, duration_ms: usize) {
        // TODO something like this?
        self.set_volume.set(0.);
        // self.set_duration.set(duration_ms);
        // self.set_target_volume.set(1.);

        // self.set_callback.set();
    }
}
