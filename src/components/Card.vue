<template>
    <div class="card"
        @mousedown="onDragStart"
        @mousemove="onDrag"
        @mouseup="onDragStop"
        @mouseleave="onLeave"
        >card {{card.id}}</div>
</template>

<script>
export default {
    props: {
        card: Object,
        draggable: Boolean,
    },
    data() {
        return {
            isDraggable: this.draggable,
            dragging: false,
            snapTimeout: null
        }
    },
    methods: {
        // We could use the HTML drag-and-drop API
        // but it doesn't seem quite suited to the simple
        // use-case here and seems like it might be complicated
        // to adapt it for this case
        // This is buggy but fine for now (e.g. if you drag outside the window)
        onDragStart() {
            if (!this.isDraggable) return;
            this.dragging = true;
            /* this.$el.style.transform = 'rotate(-2deg)'; */
            this.$el.style.cursor = 'grab';
            if (this.snapTimeout) clearTimeout(this.snapTimeout);
            this.$emit('onDragStart', this);
        },
        onDrag(ev) {
            if (!this.isDraggable) return;
            if (this.dragging) {
                const $card = this.$el;
                let top = $card.style.top || 0;
                let newTop = `${parseInt(top) + ev.movementY}px`;
                let left = $card.style.left || 0;
                let newLeft = `${parseInt(left) + ev.movementX}px`;
                $card.style.top = newTop;
                $card.style.left = newLeft;
                this.$emit('onDrag', this);
            }
        },
        onDragStop() {
            if (!this.isDraggable) return;
            this.dragging = false;
            this.$emit('onDragStop', this);
        },
        resetDrag() {
            // Snap card back to position
            this.$el.style.transform = '';
            this.$el.style.cursor = 'default';
            this.$el.style.transition = 'all 0.2s';
            this.$el.style.top = `0px`;
            this.$el.style.left = `0px`;
            this.snapTimeout = setTimeout(() => {
                this.$el.style.transition = '';
            }, 200);
        },
        stopDrag() {
            this.resetDrag();
        },
        onLeave() {
            this.onDragStop();
        }
    }
}
</script>

<style scoped>
.card {
    position: relative;
    border-radius: 1em;
    width: 320px;
    height: 400px;
    background: #202020;
    color: #fff;
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: space-around;
    z-index: 1;
}
</style>
