<template>
  <div class="planning--page">
    <header>
      <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
      <div class="pips">
        <div class="pips--label">Initiative Points</div>
        {{availablePoints}} x <img class="pip" src="/assets/icons/pips/initiative.png">
      </div>
      <div class="pips pips--buy" @click="buyPoint">
        <div class="pips--label">Buy Initiative Point</div>
        {{nextPointCost}} x <img class="pip" src="/assets/icons/pips/political_capital.png"> ⮕ <img class="pip" src="/assets/icons/pips/initiative.png">
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
            <div>Initiative</div>
            <div>
              <img class="pip" v-for="i in p.points" src="/assets/icons/pips/initiative.png">
            </div>
            <div v-if="p.points > 0" @click="(ev) => {unassignPoint(p); ev.stopImmediatePropagation();}">-Point</div>
            <div>{{p.points > 0 ? p.estimate : p.cost}} years</div>
          </template>
          <template v-slot:back>
            <div class="card--back--body">
              {{state.projects[p.id].description}}
              <ul class="effects">
                <template v-for="desc in effectDescs(state.projects[p.id])">
                  <li v-html="desc"></li>
                </template>
              </ul>
            </div>
            <div class="image-attribution">
              Source image: {{state.projects[p.id].image.attribution}}
            </div>
          </template>
          <template v-slot:extras>
            <div class="card--tag" v-if="status(p) !== 'inactive'">
              {{status(p).toUpperCase()}}
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
  mixins: [ProjectMixin('Initiative')],
}
</script>

<style scoped>
.card {
  background: #FCE8A9;
}
.card.building {
  background: #FBC011;
}
.card--tag {
  background: #945E21;
}
</style>
