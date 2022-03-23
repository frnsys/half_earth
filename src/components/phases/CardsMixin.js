import Cards from 'components/cards/Cards.vue';
import CardFocusArea from 'components/cards/CardFocusArea.vue';
import {detectCenterElement, isTouchDevice} from 'lib/util';

export default {
  components: {
    Cards,
    CardFocusArea
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
    updateFocused(cb) {
      // Figure out what the focused card is
      this.$nextTick(() => {
        let scroller = document.querySelector('.cards');
        let els = [...document.querySelectorAll('.draggable')];
        let idx = detectCenterElement(scroller, els);
        this.onFocused(idx);
        cb();
      });
    },
    onFocused(idx) {
      this.focusedIdx = idx;
      this.focused = this.items(idx);
    },
    onDrag(rect) {
      this.allowScroll = false;

      // TODO temporarily disabling this to focus on drag handling
      // This triggers the scanner functionalities
      // this.checkDrag(rect);
    },
    onDragStop() {
      // TODO temporarily disabling this to focus on drag handling
      // This stops/cancels the scanner functionalities
      // this.stopDrag();
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
