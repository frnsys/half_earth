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
      <MiniNPC :npc="npc" />
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import consts from '/src/consts';
import NPCS from '/assets/content/npcs.json';
import MiniNPC from 'components/cards/mini/MiniNPC.vue';

const totalSeats = consts.parliamentSeats.reduce((acc,s) => acc + s, 0);

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
.parliament {
  background: url('/assets/backgrounds/parliament.jpg');
  background-size: cover;
  background-repeat: no-repeat;
}

.parliament-seats {
    display: flex;
    justify-content: space-evenly;
    width: 380px;
    margin: 1em auto;
    background: #724681;
    border-radius: 0.75em 0.75em 12em 12em;
    padding: 0.5em 0.5em 1em 0.5em;
    max-width: 100%;
    border-top: 1px solid #333;
    border-left: 1px solid #333;
    border-right: 1px solid #b49abd;
    border-bottom: 1px solid #b49abd;
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

.coalition-seats {
  text-align: center;
  font-size: 1.8em;
  margin-top: 0.75em;
  text-shadow: 1px 1px 2px black;
  color: #fff;
}

.parliament .minicard-grid {
  margin-top: 2em;
}
.parliament .minicard-grid-item {
  background: #724681;
  border-right: 1px solid #333;
  border-bottom: 1px solid #333;
  border-top: 1px solid #b49abd;
  border-left: 1px solid #b49abd;
  border-radius: 0.5em;
  color: #fff;
  position: relative;
  width: 120px;
  margin: 1em 0.25em;
}
.parliament .minicard {
  width: 120px;
  overflow: visible;
  padding-top: 2.5em;
  height: auto;
}
</style>
