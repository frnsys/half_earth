<template>
<div class="sketch">
  <div class="grid">
    <div class="row spawn-row">
      <div class="cell" v-for="_, col in cols">
        <div class="tilecard" v-if="spawnRow[col] !== null">
          <img :src="`/assets/icons/pips/${spawnRow[col]}.png`" />
          <div class="tilecard--meta">75%</div>
        </div>
      </div>
    </div>
    <div class="row enemy-row" v-for="_, row in rows">
      <div class="cell" v-for="_, col in cols">
        <div class="tilecard" v-if="grid[row][col] !== null">
          <img :src="`/assets/icons/pips/${grid[row][col]}.png`" />
        </div>
      </div>
    </div>
    <div class="row player-row">
      <div class="cell" v-for="_, col in cols"
        @click="selectedCell = col"
        :class="{active: selectedCell==col}">
        <div class="tilecard" v-if="playerRow[col] !== null" @click="playerRow[col] = null">
          <img :src="`/assets/icons/pips/${playerRow[col]}.png`" />
        </div>
      </div>
    </div>
  </div>
  <div class="cards">
    <div class="tilecard" v-for="card in cards" @click="select(card)">
      <img :src="`/assets/icons/pips/${card}.png`" />
    </div>
  </div>
  <div class="actions">
    <button @click="nextTurn">Next Turn</button>
    <button @click="upgradeMonitoring">Improve Monitoring</button>
  </div>
</div>
</template>

<script>
const MAX_ROWS = 3;
const MAX_COLS = 6;
const EVENTS = ['hurricane', 'flood', 'heatwave', 'wildfires', 'drought', 'crop_failure'];

function range(n) {
  return [...Array(n).keys()];
}

function randChoice(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}


function spawnEvent() {
  return randChoice(EVENTS);
}

export default {
  data() {
    return {
      rows: 1,
      cols: 4,
      cards: ['food', 'drought', 'power'],
      selectedCell: null,
      grid: range(MAX_ROWS).map(() => range(MAX_COLS).map(() => null)),
      spawnRow: range(MAX_COLS).map(() => spawnEvent()),
      playerRow: range(MAX_COLS).map(() => null),
    }
  },
  methods: {
    upgradeMonitoring() {
      if (this.rows < MAX_ROWS) {
        this.rows += 1;
      }
    },
    select(card) {
      if (this.selectedCell === null) return;
      this.playerRow[this.selectedCell] = card;
    },
    nextTurn() {
      let newGrid = range(MAX_ROWS).map(() => range(MAX_COLS).map(() => null));
      this.spawnRow.forEach((cell, j) => newGrid[0][j] = cell);
      this.grid.forEach((row, i) => {
        if (i < this.grid.length - 1) {
          row.forEach((cell, j) => newGrid[i+1][j] = cell);
        }
      });
      this.grid = newGrid;
      this.spawnRow = range(this.cols).map(() => spawnEvent());
    }
  }
}
</script>

<style scoped>
.sketch {
  background: #fff;
  padding: 1em;
  user-select: none;
}
.grid {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.row {
  display: flex;
}
.cell {
  width: 64px;
  height: 64px;
  border-right: 1px solid #000;
  border-bottom: 1px solid #000;
  display: flex;
  justify-content: space-around;
  align-items: center;
}
.cell:first-child {
  border-left: 1px solid #000;
}
.row:first-child .cell {
  border-top: 1px solid #000;
}

.player-row {
  background: #f0f0f0;
}
.player-row .cell {
  cursor: pointer;
}
.player-row .cell:hover {
  background: #e8e8e8;
}
.player-row .cell.active {
  background: #FFFBE5;
}

.spawn-row .tilecard {
  background: #11111185;
}
.spawn-row .tilecard img {
  opacity: 0.6;
}

.tilecard {
  background: #111;
  border-radius: 0.2em;
  padding: 0.25em;
  width: 58px;
  height: 58px;
  display: flex;
  justify-content: space-around;
  align-items: center;
  cursor: pointer;
  border: 2px solid transparent;
  position: relative;
}
.tilecard:hover {
  background: #333;
  border: 2px solid #43CC70;
}
.tilecard img {
  width: 32px;
  height: 32px;
}
.tilecard--meta {
  position: absolute;
  bottom: 0;
  left: 50%;
  color: #fff;
  font-size: 0.7em;
  transform: translate(-50%, 50%);
  background: #111;
  border-radius: 0.2em;
  line-height: 1;
  padding: 0.05em 0.1em;
}

.cards {
  padding: 2em 1em 1em 1em;
  display: flex;
  justify-content: space-around;
}
</style>
