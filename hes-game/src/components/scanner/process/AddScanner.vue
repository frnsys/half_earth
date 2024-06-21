<template>
  <div class="scanbar-wrapper" ref="target">
    <div class="mini-scanbar">
        <div class="scanbar-base">
          <div class="scan-progress-bar" ref="progress"></div>
        </div>
        <div class="scanbar-led scanbar-led-ok"></div>
        <div class="scanbar-led scanbar-led-bad"></div>
        <div class="card-scan-target"></div>
    </div>
  </div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import tutorial from '/src/tutorial';
import ScannerMixin from '../ScannerMixin';

export default {
  props: ['points', 'process', 'addPoint'],
  mixins: [ScannerMixin],
  computed: {
    shouldShow() {
      return this.addable;
    },
    revealTarget() {
      return 65;
    },
    scanTime() {
      return consts.processCardScanTime;
    },
    addable() {
      let change = state.processMixChanges[this.process.output][this.process.id] || 0;
      return this.points !== 0 && change + 1 <= game.processMaxShare(this.process);
    },
  },
  methods: {
    scanAllowed() {
      return this.addable;
    },
    finishScan() {
      if(this.addable) {
        if (state.tutorial == tutorial.PROCESSES) {
          state.tutorial++;
        }

        this.addPoint(this.process);
        this.pulseCard();
        this.scanCard();
      } else {
        this.rejectScan();
        this.shakeProgress();
        this.stopScanningCard();
      }
    }
  }
}
</script>
