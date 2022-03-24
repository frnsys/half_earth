<template>
  <div class="card-withdraw-target" ref="target">
    Remove points
    <div class="withdraw-bar" ref="progress"></div>
  </div>
</template>

<script>
import state from '/src/state';
import consts from '/src/consts';
import ScannerMixin from '../ScannerMixin';

export default {
  props: ['process', 'removePoint'],
  mixins: [ScannerMixin],
  computed: {
    shouldShow() {
      return this.subtractable;
    },
    revealTarget() {
      return -60;
    },
    scanTime() {
      return consts.processCardWithdrawTime;
    },
    changedMixShare() {
      let p = this.process;
      let change = state.processMixChanges[p.output][p.id] || 0;
      return p.mix_share + change;
    },
    subtractable() {
      return this.changedMixShare !== 0;
    },
  },
  methods: {
    scanAllowed() {
      return this.subtractable;
    },
    finishScan() {
      this.removePoint(this.process);
      if (this.subtractable) {
        this.scanCard();
      } else {
        this.stopScanningCard();
      }
    }
  }
}
</script>
