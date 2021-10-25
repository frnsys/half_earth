<template>
  <div class="planning--page">
    <header>
      <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
      <div class="pips">
        <div class="pips--label">Research Points</div>
        {{availablePoints}} x <img class="pip" src="/assets/icons/pips/research.png">
      </div>
      <div class="pips pips--buy" @click="buyPoint">
        <div class="pips--label">Buy Research Point</div>
        {{nextPointCost}} x <img class="pip" src="/assets/icons/pips/political_capital.png"> ⮕ <img class="pip" src="/assets/icons/pips/research.png">
      </div>
    </header>
    <Cards>
      <template v-for="p in projects">
        <Card
          :class="status(p)"
          @click="assignPoint(p)"
          :title="p.name"
          :image="imageForProject(p)">
          <template v-slot:header>
            <div>Research</div>
            <div>
              <img class="pip" v-for="i in p.points" src="/assets/icons/pips/research.png">
            </div>
            <div v-if="p.points > 0" @click="(ev) => {unassignPoint(p); ev.stopImmediatePropagation();}">-Point</div>
            <div>{{p.points > 0 ? p.estimate : p.cost}} years</div>
          </template>
          <template v-slot:back>
            <div class="card--back--body">
              {{state.projects[p.id].description}}
            </div>
            <div class="image-attribution">
              Source image: {{state.projects[p.id].image.attribution}}
            </div>
          </template>

          <template v-slot:extras>
            <div class="card--tag" v-if="status(p) !== 'inactive'">
              {{status(p) == 'building' ? 'Researching' : 'Finished'}}
            </div>
          </template>
        </Card>
      </template>
    </Cards>
  </div>
</template>

<script>
import ProjectMixin from './ProjectMixin';

export default {
  mixins: [ProjectMixin('Research')],
}
</script>

<style scoped>
.card {
  background: #DDCFE2;
}
.card.building {
  background: #c66fc6;
}
.card--tag {
	background: #4A3169;
}
</style>
