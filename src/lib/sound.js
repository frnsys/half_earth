import animate from './anim';

// Ref: <https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/Audio>
class Sound {
  constructor(url, opts) {
    opts = opts || {};
    this.url = url;
    this.audio = new Audio(url);
    Object.keys(opts).forEach((k) => this.audio[k] = opts[k]);
  }

  get volume() {
    return this.audio.volume;
  }
  set volume(val) {
    this.audio.volume = val;
  }

  play(loop) {
    if (loop) this.audio.loop = true;
    this.audio.play();
  }

  pause() {
    this.audio.pause();
  }

  reset() {
    this.audio.load();
  }

  // Duration in ms
  fade(start, end, duration, cb) {
    this.audio.volume = start;
    animate(start, end, duration, (val) => {
      this.audio.volume = val;
    }, () => {
      if (cb) cb();
    });
  }
}

export default Sound;
