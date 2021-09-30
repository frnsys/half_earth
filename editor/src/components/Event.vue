<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <div>
    <label>
      Name
      <Tip>The name of the event</Tip>
    </label>
    <textarea class="title" placeholder="Name" v-model="localData.name" @blur="save" :class="flags('name')" />
  </div>
  <fieldset>
    <div>
      <label>
        Story Arc (optional)
        <Tip>If the event is part of or triggers an arc, note the arc name here</Tip>
      </label>
      <input type="text" list="arcs" v-model="localData.arc" @blur="save" />
    </div>
    <div class="checkbox">
      <label :for="`${item.id}_repeats`">
        Repeats
        <Tip>Can this event occur more than once?</Tip>
      </label>
      <input type="checkbox" :id="`${item.id}_repeats`" v-model="localData.repeats" @change="save">
    </div>
    <div class="checkbox">
      <label :for="`${item.id}_decision`">
        Decision
        <Tip>Is this an informative event or does the player need to make a decision?</Tip>
      </label>
      <input type="checkbox" :id="`${item.id}_decision`" v-model="localData.decision" @change="save">
    </div>
  </fieldset>

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
    <input type="text" placeholder="(world.temperature - 1.5)/2." v-model="localData.probability" @blur="save" :class="flags('probability')" />
  </div>

  <div class="field-group">
    <h3>Event Effects</h3>
    <Effects :effects="localData.effects" @update="saveData('effects', $event)" />
  </div>

  <div class="choices" v-if="localData.decision">
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

      <div class="radio">
        <label>Type:</label>
        <div>
          <label :for="`${item.id}-${i}-none`">None</label>
          <input :id="`${item.id}-${i}-none`" type="radio" v-model="choice.type" value="none" @change="save">
        </div>
        <div>
          <label :for="`${item.id}-${i}-malthusian`">Malthusian</label>
          <input :id="`${item.id}-${i}-malthusian`" type="radio" v-model="choice.type" value="malthusian" @change="save">
        </div>
        <div>
          <label :for="`${item.id}-${i}-falc`">FALC</label>
          <input :id="`${item.id}-${i}-falc`" type="radio" v-model="choice.type" value="falc" @change="save">
        </div>
        <div>
          <label :for="`${item.id}-${i}-hes`">HES</label>
          <input :id="`${item.id}-${i}-hes`" type="radio" v-model="choice.type" value="hes" @change="save">
        </div>
      </div>
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
      return ['name', 'description', 'probability'];
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
