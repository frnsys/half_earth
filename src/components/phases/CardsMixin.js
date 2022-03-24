import Cards from 'components/cards/Cards.vue';
import CardFocusArea from 'components/cards/CardFocusArea.vue';
import Draggable from 'components/cards/Draggable.vue';
import {detectCenterElement} from 'lib/util';

export default {
  components: {
    Cards,
    CardFocusArea,
    Draggable,
  },
  mounted() {
    this.updateFocused();
  },
  data() {
    return {
      allowScroll: true,
      allowSwipe: true,
      focused: 0,
      focusedIdx: 0,
    }
  },
  methods: {
    updateFocused() {
      // Figure out what the focused card is
      this.$nextTick(() => {
        let scroller = document.querySelector('.cards');
        let els = [...document.querySelectorAll('.draggable')];
        let idx = detectCenterElement(scroller, els);
        this.onFocused(idx);
      });
    },
    onFocused(idx) {
      this.focusedIdx = idx;
      this.focused = this.items(idx);
    },
    onDrag(rect) {
      // This triggers the scanner functionalities
      this.$refs.addScanner.checkDrag(rect);
      this.$refs.removeScanner.checkDrag(rect);
      this.allowScroll = false;
    },
    onDragStop() {
      // This stops/cancels the scanner functionalities
      this.$refs.addScanner.stopDrag();
      this.$refs.removeScanner.stopDrag();
      this.allowScroll = true;
    },
    onScrollStart() {
      this.allowSwipe = false;
    },
    onScrollEnd() {
      this.allowSwipe = true;
    }
  }
}
