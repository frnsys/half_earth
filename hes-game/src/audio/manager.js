import Sound from './sound';
import animate from 'lib/anim';
import Playlist from './playlist';

const FADE_TIME = 1000;

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

export default AudioManager;
