<template>
<li class="item" :id="item.id" ref="root">
  <div>
    <label>
      Name
      <Tip>The name of the event</Tip>
    </label>
    <input class="title" type="text" placeholder="Name" v-model="localData.name" @blur="save" :class="flags('name')" />
  </div>
  <div>
    <label>
      Story Arc (optional)
      <Tip>If the event is part of or triggers an arc, note the arc name here</Tip>
    </label>
    <input type="text" list="arcs" v-model="localData.arc" @blur="save" />
  </div>

  <div>
    <label>
      Description
      <Tip>A more detailed narrative description of the event.</Tip>
    </label>
    <input type="text" placeholder="Description" v-model="localData.description" @blur="save" :class="flags('description')" />
  </div>

  <div>
    <label>
      Probability Function
      <Tip>Write out the probability of this event occurring as an equation. The result must be a float that's greater than 0, ideally in the range [0-1]; values higher than 1 are just treated as 1 (i.e. guaranteed to happen). Use whatever variables you like.</Tip>
    </label>
    <input type="text" placeholder="(world.temperature - 1.5)/2." v-model="localData.probability" @blur="save" :class="flags('description')" />
  </div>

  <div class="field-group">
    <h3>Event Effects</h3>
    <Effects :effects="localData.effects" @update="saveData('effects', $event)" />
  </div>

  <div class="choices">
    <div class="field-group" v-for="(choice, i) in localData.choices">
      <h3>Choice {{i+1}}</h3>
      <div>
        <label>
          Choice Text
          <Tip>The text representing this choice, presented to the player.</Tip>
        </label>
        <input type="text" placeholder="Choice text" v-model="choice.text" @blur="save" :class="choiceFlag(i, 'text')" />
      </div>
      <div>
        <label>
          Conditions (optional)
          <Tip>A player can't select this choice if these conditions are false.</Tip>
        </label>
      </div>
      <input type="text" placeholder="Condition(s)" v-model="choice.condition" @blur="save"/>
      <Effects :effects="choice.effects" @update="saveChoiceEffects(i, $event)" />
    </div>
  </div>

  <div>
    <label>
      Flavor Text/Dialogue
      <Tip>Advisor dialogue introducing the event.</Tip>
    </label>
    <textarea v-model="localData.flavor" placeholder="Flavor text and dialogue" @blur="save" />
  </div>

  <Notes :notes="localData.notes" @blur="saveNotes" />
</li>
</template>

<script>
import ItemMixin from './ItemMixin';

export default {
  mounted() {
    if (!this.localData.choices) {
      this.localData.choices = [...Array(4)].map(() => ({
        text: '',
        condition: '',
        effects: []
      }));
      this.save();
    }
  },
  methods: {
    saveChoiceEffects(i, effects) {
      this.localData.choices[i].effects = effects;
      this.save();
    },
    choiceFlag(i, key) {
      let val = this.localData.choices[i][key];
      return {invalid: !(val && val.length > 0)};
    }
  },
  computed: {
    validateKeys() {
      return ['name', 'description'];
    },
    questionKeys() {
      return ['name', 'description'];
    },
  },
  mixins: [ItemMixin]
};
</script>

<style>
.choices {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-around;
}
.choices .field-group {
  width: 49%;
}
</style>
