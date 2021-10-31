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
          :title="p.name"
          :effects="effectDescs(activeEffects(p))"
          :image="imageForProject(p)">
          <template v-slot:header>
            <div>Research</div>
            <div>
              <img class="pip" v-for="i in p.points" src="/assets/icons/pips/research.png">
            </div>
            <div>{{p.points > 0 ? p.estimate : p.cost}} years</div>
          </template>
          <template v-slot:front>
            <div class="card--actions" v-if="status(p) == 'inactive' || status(p) == 'building'">
              <button @click="assignPoint(p)">+<img class="pip" src="/assets/icons/pips/research.png"></button>
              <button v-if="p.points > 0" @click="unassignPoint(p)">-<img class="pip" src="/assets/icons/pips/research.png"></button>
            </div>
          </template>
          <template v-slot:back>
            <div class="card--back--body">
              <div class="card--body">
                {{state.projects[p.id].description}}
              </div>
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
  border: 6px solid #DDCFE2;
}
.card.building {
  border: 6px solid #c66fc6;
}
.card--tag {
	background: #4A3169;
}
</style>
