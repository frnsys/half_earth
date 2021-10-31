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
          :title="p.name"
          :flag="status(p) == 'active' && p.upgrades.length > 0 ? `Level ${p.level+1}` : null"
          :effects="effectDescs(activeEffects(p))"
          :image="imageForProject(p)">
          <template v-slot:header>
            <div>Initiative</div>
            <div>
              <img class="pip" v-for="i in p.points" src="/assets/icons/pips/initiative.png">
            </div>
            <div>{{p.points > 0 ? p.estimate : p.cost}} years</div>
          </template>
          <template v-slot:front>
            <div class="card--body project--upgrade" v-if="status(p) == 'active' && nextUpgrade(p) !== null">
              <div class="project--upgrade--title">
                <div>Upgrade</div>
                <div>{{nextUpgrade(p).cost}}<img class="pip" src="/assets/icons/pips/political_capital.png"></div>
                <button @click="upgrade(p)">Upgrade</button>
              </div>
              <ul class="effects">
                <template v-for="desc in effectDescs(nextUpgrade(p).effects)">
                  <li v-html="desc"></li>
                </template>
              </ul>
            </div>
            <div class="card--actions" v-if="status(p) == 'inactive' || status(p) == 'building'">
              <button @click="assignPoint(p)">+<img class="pip" src="/assets/icons/pips/initiative.png"></button>
              <button v-if="p.points > 0" @click="unassignPoint(p)">-<img class="pip" src="/assets/icons/pips/initiative.png"></button>
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
  border: 6px solid #FCE8A9;
}
.card.building {
  border: 6px solid #FBC011;
}
.card--tag {
  background: #945E21;
}
</style>
