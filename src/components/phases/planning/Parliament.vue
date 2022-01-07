<template>
<div class="planning--page parliament">
  <div class="parliament-seats">
    <div v-for="col in seats">
      <template v-for="seat in col">
        <div v-if="seat !== null"
          :class="{coalitionSeat: seat.isAlly}"
          :style="{background: seat.color}"></div>
        <div v-else></div>
      </template>
    </div>
  </div>
  <div class="coalition-seats">Your coalition has {{coalitionSeats}}/{{totalSeats}} seats.</div>

  <div class="minicard-grid">
    <div class="minicard-grid-item" v-for="npc in npcs">
      <div class="npc-seats" :style="{borderColor: factionColor(npc)}">{{factionSeats(npc)}} seats</div>
      <MiniNPC :npc="npc" />
      <div class="minicard-grid-item-label">{{npc.name}}</div>
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import consts from '/src/consts';
import NPCS from '/assets/content/npcs.json';
import MiniNPC from 'components/cards/MiniNPC.vue';

let totalSeats = consts.parliamentSeats.reduce((acc,s) => acc + s, 0);

export default {
  components: {
    MiniNPC
  },
  data() {
    return {
      totalSeats,
      npcs: state.gameState.npcs.filter((npc) => !npc.locked),
    }
  },
  methods: {
    factionSeats(npc) {
      return Math.floor(npc.seats * totalSeats);
    },
    factionColor(npc) {
      return NPCS[npc.id].color;
    }
  },
  computed: {
    coalitionSeats() {
      return state.gameState.npcs.reduce((acc, npc) => {
        if (npc.relationship >= 5) {
          return acc + this.factionSeats(npc);
        } else {
          return acc;
        }
      }, 0);
    },
    seats() {
      let usedSeats = 0;
      let seats = state.gameState.npcs.map((npc) => {
        let seats = this.factionSeats(npc);
        usedSeats += seats;
        return {
          name: npc.name,
          color: NPCS[npc.id].color,
          isAlly: npc.relationship >= 5,
          seats: seats,
        }
      });

      // TODO what to do with extra seats?
      let extraSeats = totalSeats - usedSeats;

      return consts.parliamentSeats.map((nSeats) => {
        return [...Array(nSeats).keys()].map(() => {
          while (seats.length > 0 && seats[0].seats == 0) {
            seats.shift();
          }
          if (seats.length == 0) {
            return null;
          } else {
            seats[0].seats--;
            return {
              name: seats[0].name,
              color: seats[0].color,
              isAlly: seats[0].isAlly,
            }
          }
        })
      });
    }
  },
}
</script>

<style>
.parliament-seats {
  display: flex;
  justify-content: space-evenly;
  width: 360px;
  margin: 1em auto;
}
.parliament-seats > div {
  display: flex;
  flex-direction: column;
}
.parliament-seats > div > div {
  width: 18px;
  height: 18px;
  margin: 0.5em;
  background: #aaa;
}
.parliament-seats .coalitionSeat {
  border: 2px solid #000;
}

.npc-seats {
  border-bottom: 3px solid;
  text-align: center;
  font-size: 0.8em;
}

.coalition-seats {
  text-align: center;
  font-size: 0.9em;
}

.parliament .minicard-grid {
  margin-top: 2em;
}
</style>
