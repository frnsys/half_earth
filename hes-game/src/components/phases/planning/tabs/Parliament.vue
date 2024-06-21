<template>
<div class="planning--page parliament">
  <div class="parliament-suspended" v-if="suspended">{{t('Parliament Suspended')}}</div>
  <div class="parliament-seats" :class="{'parliament-suspended-fade': suspended}">
    <div v-for="col in seats">
      <template v-for="seat in col">
        <div v-if="seat !== null" :class="{coalitionSeat: seat.isAlly}">
          <img
            :src="`/assets/characters/${seat.name}.png`"
            onerror="this.src='/assets/placeholders/character.png';" />
        </div>
        <div v-else></div>
      </template>
    </div>
  </div>
  <div class="coalition-seats" :class="{'parliament-suspended-fade': suspended}" v-tip="{
    icon: 'political_capital',
    text: t('How many seats your coalition has. Draw parties to your coalition by implementing projects they support.')
  }">
    {{t('Your coalition has {coalitionSeats}/{totalSeats} seats.', {coalitionSeats, totalSeats})}}
  </div>

  <div class="minicard-grid">
    <div class="minicard-grid-item" v-for="npc in npcs" :key="npc.id">
      <MiniNPC :npc="npc" />
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import consts from '/src/consts';
import {rngForYear} from '/src/lib/util';
import NPCS from 'content/npcs.json';
import MiniNPC from 'components/cards/mini/MiniNPC.vue';

const totalSeats = consts.parliamentSeats.reduce((acc,s) => acc + s, 0);

export default {
  components: {
    MiniNPC
  },
  data() {
    return {
      state,
      totalSeats,
      suspended: state.gameState.flags.includes('ParliamentSuspended'),
    }
  },
  computed: {
    npcs() {
      return state.gameState.npcs.filter((npc) => !npc.locked);
    },
    coalitionSeats() {
      return this.seats.flat().filter((s) => s.isAlly).length;
    },
    seats() {
      let usedSeats = 0;
      let npcs = state.gameState.npcs.filter((npc) => !npc.locked);
      let seats = npcs.map((npc) => {
        let seats = Math.floor(npc.seats * totalSeats);
        usedSeats += seats;
        return {
          name: npc.name,
          color: NPCS[npc.id].color,
          isAlly: npc.relationship >= 5,
          seats: seats,
        }
      });

      // Assign extra seats randomly
      // We generate the assignment based on the current year
      // so that it's consistent
      let extraSeats = totalSeats - usedSeats;
      let rng = rngForYear(state.gameState.world.year);
      state.extraSeats = npcs.reduce((acc, npc) => {
        acc[npc.name] = 0;
        return acc;
      }, {});
      while (extraSeats > 0) {
        let s = seats[Math.floor(rng() * seats.length)];
        s.seats++;
        state.extraSeats[s.name]++;
        extraSeats--;
      }

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
  background: url('/assets/backgrounds/parliament.png');
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center center;
  image-rendering: pixelated;
}

.parliament-seats {
  display: flex;
  justify-content: space-evenly;
  width: 380px;
  margin: 1em auto;
  background: #965FA9;
  border-radius: 0.75em 0.75em 12em 12em;
  padding: 0.5em 0.5em 1em 0.5em;
  max-width: 100%;
  border-top: 1px solid #333;
  border-left: 1px solid #333;
  border-right: 1px solid #b49abd;
  border-bottom: 1px solid #b49abd;
  image-rendering: auto;
}
.parliament-seats > div {
  display: flex;
  flex-direction: column;
}
.parliament-seats > div > div {
  width: 26px;
  height: 26px;
  margin: 0.5em;
}
.parliament-seats .coalitionSeat {
  box-shadow: 0 0 7px yellow, 0 0 12px 5px #ff66ff;
  border-radius: 0.2em;
  background: #fdf7e2;
}

.coalition-seats {
  text-align: center;
  font-size: 1.8em;
  text-shadow: 1px 1px 2px black;
  color: #fff;
  max-width: 180px;
  margin: 0.75em auto 0;
}

.parliament .minicard-grid {
  max-width: 560px;
  margin: 2em auto 0;
}
.parliament .minicard-grid-item {
  background: #965FA9;
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
  padding-bottom: 1em;
  height: 100%;
}

.parliament-suspended {
  position: absolute;
  font-size: 3em;
  left: 50%;
  transform: translate(-50%, 0) rotate(-12deg);
  color: #fff;
  top: 4.5em;
  max-width: 320px;
  text-align: center;
  z-index: 1;
  background: #FF0404;
  border-radius: 0.2em;
  box-shadow: 1px 1px 2px rgb(0 0 0 / 50%);
}

.parliament-suspended-fade {
  opacity: 0.5;
}
</style>
