import api from '../../api';
import util from '../../util';
import Tip from '../Tip.vue';
import Flags from '../Flags.vue';
import Notes from '../Notes.vue';
import Image from '../Image.vue';
import Effects from '../subs/Effects.vue';
import Outputs from '../subs/Outputs.vue';
import Resources from '../subs/Resources.vue';
import Byproducts from '../subs/Byproducts.vue';
import Conditions from '../subs/Conditions.vue';
import Outcomes from '../subs/Outcomes.vue';
import OutcomesSummary from '../subs/OutcomesSummary.vue';
import Probabilities from '../subs/Probabilities.vue';
import ProbabilitiesSummary from '../subs/ProbabilitiesSummary.vue';
import Dialogue from '../subs/Dialogue.vue';
import DialogueSummary from '../subs/DialogueSummary.vue';
import Upgrades from '../subs/Upgrades.vue';
import UpgradesSummary from '../subs/UpgradesSummary.vue';
import EffectsSummary from '../subs/EffectsSummary.vue';
import ChoicesSummary from '../subs/ChoicesSummary.vue';
import ResourcesSummary from '../subs/ResourcesSummary.vue';
import ByproductsSummary from '../subs/ByproductsSummary.vue';
import OutputsSummary from '../subs/OutputsSummary.vue';
import validate from '../../validate';
import Autolinker from 'autolinker';

function resizeTextArea(ev) {
  util.resizeTextArea(ev.target);
}

export default {
  props: ['item'],
  data() {
    return {
      editing: false,
      localData: Object.assign({}, this.item)
    };
  },
  components: {
    Tip, Flags, Notes, Image, Effects,
    Outputs, Resources, Byproducts,
    Conditions, Probabilities,
    ProbabilitiesSummary, EffectsSummary,
    ChoicesSummary, ResourcesSummary,
    Dialogue, DialogueSummary,
    ByproductsSummary, OutputsSummary,
    Outcomes, OutcomesSummary,
    Upgrades, UpgradesSummary,
  },
  created() {
    if (this.localData._validation === undefined) {
      this.validate();
      this.save();
    }
  },
  mounted() {
    this.$refs.root.querySelectorAll('textarea').forEach((el) => {
      util.resizeTextArea(el);
      el.addEventListener('input', resizeTextArea);
      el.addEventListener('focus', resizeTextArea);
    });
  },
  computed: {
    notesHtml() {
      return Autolinker.link(this.localData.notes.replaceAll('\n', '<br />'));
    },
    flavorHtml() {
      return this.localData.flavor.replaceAll('\n', '<br />');
    },
    validator() {
      let type = this.localData._type;
      return validate[type];
    },
  },
  watch: {
    editing(val) {
      if (val) {
        this.$nextTick(() => {
          this.$refs.root.querySelectorAll('textarea').forEach((el) => {
            util.resizeTextArea(el);
            el.addEventListener('input', resizeTextArea);
            el.addEventListener('focus', resizeTextArea);
          });
        });
      } else {
        this.$refs.root.querySelectorAll('textarea').forEach((el) => {
          el.removeEventListener('input', resizeTextArea);
          el.removeEventListener('focus', resizeTextArea);
        });
      }
    },
    item(newItem) {
      this.localData = Object.assign({}, newItem);
      this.$el.querySelectorAll('textarea').forEach((el) => {
        util.resizeTextArea(el);
      });
    }
  },
  methods: {
    save() {
      api.update(this.localData);
    },
    saveNotes(notes) {
      this.localData.notes = notes;
      // this.save();
    },
    saveData(key, data) {
      this.localData[key] = data;
      // this.save();
    },
    delete() {
      if (confirm('Are you sure you want to delete this?')) {
        this.localData.deleted = true;
        this.save();
      }
    },
    validate() {
      this.localData._validation = {
        invalid: this.invalid(),
        questions: this.questions(),
      }
    },
    invalid() {
      return this.validator.validate.filter((k) => {
        return !this.validateKey(k);
      });
    },
    questions() {
      return this.validator.questions.filter((k) => {
        let val = this.localData[k];
        return val && val.includes('? ');
      });
    },
    flags(key) {
      return {
        invalid: this.localData._validation.invalid.includes(key),
        question: this.localData._validation.questions.includes(key)
      }
    },
    validateKey(key) {
      return this.validator.validateKey(this.localData, key);
    },
    defined(key) {
      return this.localData[key] !== undefined && this.localData[key] !== '';
    },
    definedWithValues(key) {
      return this.localData[key] !== undefined && this.localData[key] !== '' && this.localData[key].length > 0;
    },
    toggleEditing() {
      this.editing = !this.editing;

      // Save when you leave editing
      if (!this.editing) {
        this.validate();
        this.save();
      }
    }
  }
};
