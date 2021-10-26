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
          @click="payPoints(p)"
          :title="p.name"
          :image="imageForProject(p)">
          <template v-slot:header>
            <div>Policy</div>
            <div>
              {{p.cost}}<img class="pip" src="/assets/icons/pips/political_capital.png">
            </div>
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
  background: #CFB99A;
}
.card.active {
  background: #BEAC97;
}
.card--tag {
	background: #5B534D;
}
</style>
