import animate from 'lib/anim';

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
      this.audio.volume = val * this._volume;
    }, () => {
      if (cb) cb();
    });
  }

  fadeOut(duration, cb) {
    animate(1, 0, duration, (val) => {
      this.audio.volume = val * this._volume;
    }, () => {
      if (cb) cb();
    });
  }
}

export default Sound;
