import Sound from './sound';

const xFadeSecs = 15;

class Playlist {
  constructor(urls, opts) {
    this.index = 0;
    this.urls = urls;
    this.volume = 1;
    this.xFading = false;
    this.muted = false;
    this.sounds = this.urls.map((url) => {
      let sound = new Sound(url, opts);
      sound.audio.addEventListener('loadedmetadata', () => {
        let fadeOutAt = sound.audio.duration - xFadeSecs;
        sound.audio.addEventListener('timeupdate', () => {
          if (sound.audio.currentTime >= fadeOutAt && !this.xFading) {
            this.xFadeToNext();
          }
        });
      });
      return sound;
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
    cur.fade(cur.volume, 0, xFadeSecs * 1000, () => {
      cur.pause();
      cur.reset();
      cur.volume = this.volume;
    });
    this.next.volume = 0;
    this.next.play();
    this.next.fade(0, this.volume, xFadeSecs * 1000, () => {
      this.index = this.nextIndex;
    });
  }

  playNext() {
    this.current.pause();
    this.current.reset();
    this.index = this.nextIndex;
    this.current.play();
  }
}

export default Playlist;
