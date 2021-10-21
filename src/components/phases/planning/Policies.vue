<template>
  <div class="planning--page">
    <header>
      <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
      <div class="pips">
        <div class="pips--label">Political Capital</div>
        <template v-for="i in state.points['Policy'].available">
          <img class="pip" src="/assets/placeholders/pip3.png">
        </template>
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
              <img v-for="i in p.cost" class="pip" src="/assets/placeholders/pip3.png">
            </div>
          </template>
          <template v-slot:back>
            {{state.projects[p.id].description}}
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
