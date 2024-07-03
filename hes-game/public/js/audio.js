const FADE_TIME = 1000;
const X_FADE_SECS = 15;

class AudioManager {
  constructor() {
    this._volume = 1;
    this.muted = false;
    this.soundtrack = new Playlist([]);
    this.atmosphere = new Playlist([]);
    this.oneshots = {};
  }

  get soundtrack() {
    return this._soundtrack;
  }
  set soundtrack(val) {
    if (this._soundtrack) {
      this._soundtrack.stop();
    }
    this._soundtrack = val;
  }

  get atmosphere() {
    return this._atmosphere;
  }
  set atmosphere(val) {
    if (this._atmosphere) {
      this.atmosphere.stop();
    }
    this._atmosphere = val;
  }

  startPlaylist(tracks, fade) {
    if (tracks.length == 1) {
      // For crossfading looping
      tracks = [tracks[0], tracks[0]];
    }
    let playlist = new Playlist(tracks);

    if (fade) {
      playlist.volume = 0;
      playlist.play();
      animate(0, 1, FADE_TIME, (val) => {
        playlist.volume = this.volume * val;
      });
    } else {
      playlist.volume = this.volume;
      playlist.play();
    }

    if (this.muted) {
      playlist.mute();
    }
    return playlist;
  }

  stopPlaylist(playlist, fade, cb) {
    if (fade) {
      animate(1, 0, FADE_TIME, (val) => {
        playlist.volume = this.volume * val;
      }, () => {
        playlist.stop();
        if (cb) cb();
      });
    } else {
      playlist.stop();
      if (cb) cb();
    }
  }

  startSoundtrack(track, fade) {
    if (fade) {
      this.stopSoundtrack(fade, () => {
        this.soundtrack = this.startPlaylist([track], fade);
      });
    } else {
      this.soundtrack.stop();
      this.soundtrack = this.startPlaylist([track], fade);
    }
  }

  stopSoundtrack(fade, cb) {
    this.stopPlaylist(this.soundtrack, fade, cb);
  }

  startAtmosphere(track, fade) {
    if (fade) {
      this.stopAtmosphere(fade, () => {
        this.atmosphere = this.startPlaylist([track], fade);
      });
    } else {
      this.atmosphere.stop();
      this.atmosphere = this.startPlaylist([track], fade);
    }
  }

  stopAtmosphere(fade, cb) {
    this.stopPlaylist(this.atmosphere, fade, cb);
  }

  playOneShot(src) {
    if (!(src in this.oneshots)) {
      this.oneshots[src] = new Sound(src);
    }

    let sound = this.oneshots[src];
    if (!this.muted) {
      sound.reset();
      sound.play();
    }
  }

  get volume() {
    return this._volume;
  }

  set volume(val) {
    this._volume = val;
    Object.values(this.oneshots).forEach((sound) => {
      sound.volume = this._volume;
    });
    this.soundtrack.volume = this._volume;
    this.atmosphere.volume = this._volume;
  }

  mute() {
    this.muted = true;
    this.soundtrack.mute();
    this.atmosphere.mute();
    Object.values(this.oneshots).forEach((sound) => {
      sound.audio.muted = true;
    });
  }

  unmute() {
    this.muted = false;
    this.soundtrack.unmute();
    this.atmosphere.unmute();
    Object.values(this.oneshots).forEach((sound) => {
      sound.audio.muted = false;
    });
  }
}

class Playlist {
  constructor(urls, opts) {
    this.index = 0;
    this.urls = urls;
    this._volume = 1;
    this.xFading = false;
    this.muted = false;
    this.sounds = this.urls.map((url) => {
      let sound = new Sound(url, opts);
      sound.audio.addEventListener('loadedmetadata', () => {
        let fadeOutAt = sound.audio.duration - X_FADE_SECS;
        sound.audio.addEventListener('timeupdate', () => {
          if (sound.audio.currentTime >= fadeOutAt && !this.xFading) {
            this.xFadeToNext();
          }
        });
      });
      return sound;
    });
  }

  get volume() {
    return this._volume;
  }

  set volume(val) {
    this._volume = val;
    this.sounds.forEach((sound) => {
      sound.volume = this._volume;
    });
  }

  get current() {
    return this.sounds[this.index];
  }

  get paused() {
    return this.current.audio.paused;
  }

  get nextIndex() {
    return this.index < this.urls.length - 1 ? this.index + 1 : 0;
  }

  get next() {
    return this.sounds[this.nextIndex];
  }

  play() {
    this.current.play();
  }

  pause() {
    this.current.pause();
  }

  stop() {
    this.sounds.forEach((sound) => {
      sound.pause();
    });
  }

  mute() {
    this.muted = true;
    this.sounds.forEach((sound) => {
      sound.audio.muted = true;
    });
  }

  unmute() {
    this.muted = false;
    this.sounds.forEach((sound) => {
      sound.audio.muted = false;
    });
  }

  xFadeToNext() {
    this.xFading = true;

    let cur = this.current;
    cur.fadeOut(X_FADE_SECS * 1000, () => {
      cur.pause();
      cur.reset();
      cur.volume = this._volume;
    });
    this.next.volume = 0;
    this.next.play();
    this.next.fadeIn(X_FADE_SECS * 1000, () => {
      this.index = this.nextIndex;
      this.xFading = false;
    });
  }

  playNext() {
    this.current.pause();
    this.current.reset();
    this.index = this.nextIndex;
    this.current.play();
  }
}

// Ref: <https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/Audio>
class Sound {
  constructor(url, opts) {
    opts = opts || {};
    this.url = url;
    this.audio = new Audio(url);
    Object.keys(opts).forEach((k) => this.audio[k] = opts[k]);
    this._volume = 1;
  }

  get volume() {
    return this._volume;
  }
  set volume(val) {
    val = Math.min(Math.max(val, 0), 1);
    this._volume = val
    this.audio.volume = val;
  }

  play(loop) {
    if (loop) this.audio.loop = true;
    this.audio.play().catch((_err) => {
      // Ignore
    });
  }

  pause() {
    this.audio.pause();
  }

  reset() {
    this.audio.load();
  }

  // Duration in ms
  fadeIn(duration, cb) {
    animate(0, 1, duration, (val) => {
      this.volume = val;
    }, () => {
      if (cb) cb();
    });
  }

  fadeOut(duration, cb) {
    animate(1, 0, duration, (val) => {
      this.volume = val;
    }, () => {
      if (cb) cb();
    });
  }
}

// <https://gist.github.com/andjosh/6764939>
function easeInOutQuad(t, b, c, d) {
  t /= d/2;
	if (t < 1) return c/2*t*t + b;
	t--;
	return -c/2 * (t*(t-2) - 1) + b;
};

// Duration in ms
// Can pass in arrays for start and end to animate
// multiple values at once.
function animate(start, end, duration, updateFn, cb, linear) {

  // Object to manage the animation
  let anim = {};

  if (!Array.isArray(start)) start = [start];
  if (!Array.isArray(end)) end = [end];

  // Setup and start the animation
  let changes = start.map((s, i) => end[i] - s);
  let startTime = performance.now();
  let update = (timestamp) => {
    let elapsed = timestamp - startTime;
    let vals = linear ? start.map((_, i) => elapsed/duration * changes[i])
      : start.map((s, i) => easeInOutQuad(elapsed, s, changes[i], duration));

    // If timestamp is very large it can cause
    // the value to overshoot the end target,
    // so clamp it in case.
    if (end >= start) {
      vals = vals.map((v) => Math.min(v, end));
    } else {
      vals = vals.map((v) => Math.max(v, end));
    }
    updateFn(...vals);
    if (elapsed < duration) {
      anim.id = requestAnimationFrame(update);
    } else {
      updateFn(...end);
      if (cb) cb();
    }
  };
  anim.id = requestAnimationFrame(update);

  // Call this to cancel/stop the animation
  anim.stop = () => {
    cancelAnimationFrame(anim.id);
  };

  return anim;
}

export {AudioManager};
