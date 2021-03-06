import animate from 'lib/anim';
import consts from '/src/consts';
import {updateTransform} from 'lib/util';

export default {
  data() {
    return {
      scanning: false,
      scanAnim: null,
    }
  },
  mounted() {
    this.scanTimeMultiplier = 1;

    this.$nextTick(() => {
      this.getEdges();

      // Hacky...double-check position
      // after animations have finished
      setTimeout(() => {
        this.getEdges();
      }, 500);
    });
    window.addEventListener('resize', this.getEdges);
  },
  beforeUnmount() {
    window.removeEventListener('resize', this.getEdges);
  },
  methods: {
    getEdges() {
      if (this.$refs.target) {
        let rect = this.$refs.target.getBoundingClientRect();
        this.topY = rect.y + this.revealTarget;
        this.botY = this.topY + rect.height;
      }
    },
    targetRef() {
      return this.$refs.target;
    },
    progressRef() {
      return this.$refs.progress;
    },

    // Animations
    shakeProgress() {
      let el = this.progressRef().parentElement;
      el.classList.add('scan-error');
      el.classList.add('shake');
      setTimeout(() => {
        el.classList.remove('shake');
        el.classList.remove('scan-error');
      }, 350);
    },
    pulseCard() {
      let el = document.querySelector('.draggable.active');
      if (el) {
        animate(consts.cardScale, consts.cardScale*1.05, 100, (val) => {
          updateTransform(el, {scale: val});
        }, () => {
          animate(consts.cardScale*1.05, consts.cardScale, 100, (val) => {
            updateTransform(el, {scale: val});
          });
        });
      }
    },
    shrinkPulseCard() {
      let el = document.querySelector('.draggable.active');
      if (el) {
        animate(consts.cardScale, consts.cardScale*0.95, 100, (val) => {
          updateTransform(el, {scale: val});
        }, () => {
          animate(consts.cardScale*0.95, consts.cardScale, 100, (val) => {
            updateTransform(el, {scale: val});
          });
        });
      }
    },
    shakeScreen() {
      document.body.classList.add('shake');
      window.audioManager.playOneShot('/assets/sounds/impact.mp3');
      setTimeout(() => {
        document.body.classList.remove('shake');
      }, 500);
    },

    // Movement handling
    checkDrag(dragRect) {
      if (this.shouldShow) {
        let target = this.targetRef();
        target.style.visibility = 'visible';
        target.style.transform = `translate(0, ${this.revealTarget}px)`;

        let intersects = dragRect.topY < this.botY && dragRect.botY > this.topY;
        if (intersects) {
          if (!this.scanning && this.scanAllowed()) {
            this.scanning = true;
            target.parentElement.classList.add('scan-ok');
            target.classList.add('scanning');
            this.scanCard();
          } else if (!this.scanAllowed()) {
            this.rejectScan();
          }
        } else {
          this.stopScanningCard();
        }
      }
    },
    stopDrag() {
      this.stopScanningCard();
      let ref = this.targetRef();
      if (ref) {
        ref.style.transform = `translate(0, 0)`;
      }
    },

    rejectScan() {
      let ref = this.targetRef();
      ref.parentElement.classList.add('scan-fail');
      ref.classList.add('no-scan');
      let el = document.querySelector('.draggable.active');
      if (el) el.classList.add('scan-reject');
      setTimeout(() => {
        ref.parentElement.classList.remove('scan-fail');
      }, 500);
    },
    scanCard() {
      let progressRef = this.progressRef();
      this.scanAnim = animate(0, 100, this.scanTime * 1000 * this.scanTimeMultiplier, (val) => {
        progressRef.style.width = `${val}%`;
      }, () => {
        if (this.scanning) {
          this.scanTimeMultiplier *= 4/5;
          this.scanTimeMultiplier = Math.max(0.2, this.scanTimeMultiplier);
          this.finishScan();
        }
      }, true);
    },
    stopScanningCard() {
      this.scanTimeMultiplier = 1;
      this.scanning = false;
      let ref = this.targetRef();
      ref.classList.remove('scanning');
      ref.classList.remove('no-scan');
      ref.parentElement.classList.remove('scan-ok');
      let active = document.querySelector('.draggable.active');
      if (active) active.classList.remove('scan-reject');
      if (this.scanAnim) {
        this.scanAnim.stop();
        this.scanAnim = null;
        this.progressRef().style.width = '0';
      }
    },
  }
}
