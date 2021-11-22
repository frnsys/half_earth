import Sound from './sound';

class Playlist {
  constructor(urls, opts) {
    this.index = 0;
    this.urls = urls;
    this.sounds = this.urls.map((url) => {
      let sound = new Sound(url, opts);
      sound.audio.addEventListener('ended', () => {
        this.playNext();
      });
      return sound;
    });
  }

  get current() {
    return this.sounds[this.index];
  }

  play() {
    this.current.play();
  }

  pause() {
    this.current.pause();
  }

  playNext() {
    this.current.pause();
    this.current.load();
    this.index = this.index < urls.length - 1 ? this.index + 1 : 0;
    this.current.play();
  }
}

export default Playlist;
