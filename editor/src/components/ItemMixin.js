import api from '../api';
import util from '../util';
import Tip from './Tip.vue';
import Flags from './Flags.vue';
import Notes from './Notes.vue';
import Effects from './Effects.vue';
import Outputs from './Outputs.vue';
import Resources from './Resources.vue';
import Byproducts from './Byproducts.vue';

export default {
  props: ['item'],
  data() {
    return {
      localData: Object.assign({}, this.item)
    };
  },
  components: {
    Tip, Flags, Notes, Effects,
    Outputs, Resources, Byproducts
  },
  mounted() {
    this.$refs.root.querySelectorAll('textarea').forEach((el) => {
      util.resizeTextArea(el);
      el.addEventListener('input', () => {
        util.resizeTextArea(el);
      });
    });
  },
  computed: {
    invalid() {
      return this.validateKeys.filter((k) => {
        let val = this.localData[k];
        return !(val && val !== '');
      });
    },
    questions() {
      return this.questionKeys.filter((k) => {
        let val = this.localData[k];
        return val && val.includes('?');
      });
    }
  },
  watch: {
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
      this.save();
    },
    saveData(key, data) {
      this.localData[key] = data;
      this.save();
    },
    flags(key) {
      return {
        invalid: this.invalid.includes(key),
        question: this.questions.includes(key)
      }
    }
  }
};
