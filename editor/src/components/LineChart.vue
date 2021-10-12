<template>
  <div class="chart">
    <canvas ref="canvas"></canvas>
  </div>
</template>

<script>
import Chart from 'chart.js/auto';

const colors = [
  '#4287f5',
  '#f54242',
  '#0fba76',
  '#620fba',
  '#d6b20f',
  '#0d0aa8',
  '#c242b3',
  '#ad454c',
  '#ed9b4e',
  '#8bc9b5',
  '#b4b867',
  '#4f4f4f',
  '#448391',
  '#5f630d',
  '#584475',
  '#faaa70',
  '#701246',
  '#171717',
  '#bdbdbd',
  '#72b370'
];

export default {
  props: ['title', 'xs', 'y'],
  mounted() {
    this.createChart();
  },
  methods: {
    createChart() {
      const ctx = this.$refs.canvas.getContext('2d');
      const datasets =  Object.keys(this.xs).map((label, i) => ({
        label: label,
        fill: false,
        data: this.xs[label],
        borderColor: colors[i],
      }));
      this.chart = new Chart(ctx, {
          type: 'line',
          maintainAspectRatio: false,
          responsive: true,
          data: {
            datasets,
            labels: this.y,
          },
          options: {
            animation: false,
            scales: {
              y: {
                beginAtZero: true
              }
            },
            elements: {
              point:{
                radius: 2
              }
            },
            plugins: {
              title: {
                display: true,
                text: this.title
              }
            }
          }
      });
    }
  },
  watch: {
    xs() {
      this.chart.destroy();
      this.createChart();
    }
  }
}
</script>
