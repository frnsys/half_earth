<template>
  <div class="planning--page">
    <header>
      <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
      <div class="pips">
        <div class="pips--label">Political Capital</div>
        {{availablePoints}} x <img class="pip" src="/assets/icons/pips/political_capital.png">
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
            <div>Policy</div>
            <div>
              {{p.cost}}<img class="pip" src="/assets/icons/pips/political_capital.png">
            </div>
          </template>
          <template v-slot:front>
            <div class="card--body project--upgrade" v-if="status(p) == 'active' && nextUpgrade(p) !== null">
              <div class="project--upgrade--title">
                <div>Next Level</div>
                <div>{{nextUpgrade(p).cost}}<img class="pip" src="/assets/icons/pips/political_capital.png"></div>
                <button @click="upgrade(p)">Upgrade</button>
              </div>
              <ul class="effects">
                <template v-for="desc in effectDescs(nextUpgrade(p).effects)">
                  <li v-html="desc"></li>
                </template>
              </ul>
            </div>
            <div class="card--actions" v-if="status(p) == 'inactive'">
              <button @click="payPoints(p)">Implement</button>
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
  mixins: [ProjectMixin('Policy')],
}
</script>

<style scoped>
.card {
  border: 6px solid #CFB99A;
}
.card.active {
  border: 6px solid #BEAC97;
}
.card--tag {
	background: #5B534D;
}
</style>
