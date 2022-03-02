<template>
<div class="pie-chart" ref="stage"></div>
</template>

<script>
import PieChart from 'lib/pie';

export default {
  props: ['dataset', 'colors'],
  mounted() {
    this.chart = new PieChart(this.$refs.stage);
    this.render();
  },
  watch: {
    dataset() {
      this.render();
    }
  },
  methods: {
    render() {
      this.chart.reset();
      this.chart.render(this.dataset, this.colors);
    }
  }
}
</script>

<style>
.pie-chart {
  height: 320px;
  width: 320px;
  border-radius: 50%;
  /* border: 1px inset white; */

  border-top: 1px solid rgba(0,0,0,0.6);
  border-left: 1px solid rgba(0,0,0,0.6);
  border-right: 1px solid rgba(255,255,255,0.8);
  border-bottom: 1px solid rgba(255,255,255,0.8);

}
</style>
