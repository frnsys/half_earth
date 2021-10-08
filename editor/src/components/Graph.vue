<template>
<div class="graph"
  @mousedown="dragStart"
  @mouseup="dragEnd"
  @mousemove="pan"
  :class="expanded ? '' : 'graph-collapsed'" @click="expand">
  <div class="graph-controls" v-if="expanded">
    <input type="text" placeholder="Search" @keyup="search" />
    <div class="graph-collapse" @click="collapse">X</div>
  </div>
  <div v-if="!expanded" class="graph-expand-icon"><img src="/static/graph.png" /></div>
  <div class="graph-body" :style="offsets">
    <svg :style="size">
      <template v-for="edge in edges">
        <path class="graph-edge" :d="edge.path" />
      </template>
    </svg>
    <template v-for="node in nodes">
      <div class="graph-node" :style="node.style" :class="node.type">
        <div class="graph-tags">
          <div class="graph-type">{{node.type}}</div>
          <div class="graph-subtype" :class="{invalid: !node.subtype}">{{node.subtype ? node.subtype : '[Missing]'}}</div>
        </div>
        <div class="graph-invalid" v-if="!node.valid" title="Item has invalid or missing data">!</div>
        <div class="graph-node-body">
          <a @click="collapse" :href="`/?type=${node.type}#${node.id}`">{{node.text}}</a>
        </div>
        <div class="graph-node-icons">
          <template v-if="node.type === 'Event'">
            <div>
              <img class="graph-icon" v-for="factor in eventFactors(node.data)" :src="icon(factor)" />
            </div>
            <div>
              <img class="graph-icon" v-for="effect in displayEffects(node.data)" :src="icon(effect)" />
            </div>
          </template>

          <template v-if="node.type === 'Project'">
            <div class="graph-pill">
              <div>Years</div>
              <div>{{node.data.years || 'X'}}</div>
            </div>
            <div>
              <img class="graph-icon" v-for="effect in displayEffects(node.data)" :src="icon(effect)" />
            </div>
          </template>

          <template v-if="node.type === 'Process'">
            <div class="graph-pill">
              <div>{{node.data.feedstock}}</div>
              <div>{{node.data.feedstock_amount || 'X'}} {{FEEDSTOCKS[node.data.feedstock]}}</div>
            </div>
            <div class="graph-pill">
              <div>Mix</div>
              <div>{{node.data.mix_share || 'X'}}%</div>
            </div>
          </template>
        </div>
        <div class="graph-node-details">
          <template v-if="node.type === 'Event'">
            <div class="graph-node-details-meta">
              <div class="meta-pill invalid" v-if="!node.valid">Invalid/Missing Data</div>
              <div class="meta-pill">{{node.data.local ? 'Local': 'Global'}}</div>
              <div class="meta-pill" v-if="node.data.repeats">⭯ Repeats</div>
              <div class="meta-pill" v-if="node.data.locked">Locked</div>
            </div>
            <p>{{node.data.description}}</p>
            <ProbabilitiesSummary :probabilities="node.data.probabilities" />
            <EffectsSummary :effects="node.data.effects" />
          </template>

          <template v-if="node.type === 'Project'">
            <div class="graph-node-details-meta">
              <div class="meta-pill invalid" v-if="!node.valid">Invalid/Missing Data</div>
              <div class="meta-pill split-pill">
                <div>Years</div>
                <div>{{node.data.years || 'MISSING'}}</div>
              </div>
              <div class="meta-pill" v-if="node.data.uncertain">Uncertain</div>
              <div class="meta-pill" v-if="node.data.locked">Locked</div>
            </div>
            <p>{{node.data.description}}</p>
            <div>
              <h5>Implementation (per year)</h5>
              <ResourcesSummary :resources="node.data.construction" />
              <ByproductsSummary :byproducts="node.data.construction_byproducts" />
            </div>
            <div v-if="node.data.ongoing">
              <h5>Maintenance (per year)</h5>
              <ResourcesSummary :resources="node.data.maintenance" />
              <ByproductsSummary :byproducts="node.data.maintenance_byproducts" />
            </div>
            <EffectsSummary :effects="node.data.effects" />
            <OutcomesSummary :outcomes="node.data.outcomes" />
          </template>

          <template v-if="node.type === 'Process'">
            <div class="graph-node-details-meta">
              <div class="meta-pill invalid" v-if="!node.valid">Invalid/Missing Data</div>
              <div class="meta-pill split-pill">
                <div>Mix Share</div>
                <div>{{ node.data.mix_share }}%</div>
              </div>
              <div class="meta-pill" v-if="node.data.locked">Locked</div>
            </div>
            <p>{{node.data.description}}</p>
            <h5>Per {{OUTPUTS[node.data.output]}}:</h5>
            <ResourcesSummary :resources="node.data.reqs" />
            <ByproductsSummary :byproducts="node.data.byproducts" />
            <div>
              <template v-for="k in Object.keys(PROCESS_FEATURES)" v-if="node.data.features">
                <div class="summary-pill feature-pill" v-if="node.data.features[k]"><div>{{k}}</div></div>
              </template>
            </div>
          </template>
          <img class="image-preview" v-if="node.data.image" :src="`/image/${node.data.image}`"/>
        </div>
      </div>
    </template>
  </div>
</div>
</template>

<script>
import { Bezier } from "bezier-js";
import ResourcesSummary from './subs/ResourcesSummary.vue';
import ByproductsSummary from './subs/ByproductsSummary.vue';
import OutcomesSummary from './subs/OutcomesSummary.vue';
import EffectsSummary from './subs/EffectsSummary.vue';
import ProbabilitiesSummary from './subs/ProbabilitiesSummary.vue';
import validate from '../validate';
import consts from '../consts';

const icons = {
  Global: 'globe.png',
  Outlook: 'crystal_ball.png',
  Biodiversity: 'whale.png',
  Locked: 'locked.png',
  Contentedness: 'face.png',
  Local: 'local.png',
  Other: 'other_2.png',
  Temperature: 'temperature.png',
  Repeats: 'repeats.png',
  SeaLevelRise: 'wave.png',
  Habitability: 'tent_2.png',
  Health: 'heart.png',
  Population: 'people.png',
  Flag: 'flag.png',
  RunsPlayed: 'replays.png',
  PoliticalCapital: 'ballot_box.png',
  Year: 'calendar.png',
  Emissions: 'cloud.png',
  Precipitation: 'rain.png',
  ResourceIntensity: 'pickaxe.png',
  Output: 'factory.png',
};

function childrenForEffect(effect) {
  switch (effect.type) {
    case 'AddEvent':
      return [effect.entity];
    case 'TriggerEvent':
      return [effect.entity];
    case 'UnlocksProject':
      return [effect.entity];
    case 'UnlocksProcess':
      return [effect.entity];
    default:
      return [];
  }
}

function subtypeForItem(item) {
  switch (item._type) {
    case 'Event':
      return item.arc || '(no arc)';
    case 'Project':
      return item.type;
    case 'Process':
      return item.output;
  }
}

const displayFactors = [
  'LocalVariable',
  'WorldVariable',

  'Demand',
  'Output',
  'OutputDemandGap',
  'Resource',
  'ResourceDemandGap',
  'Flag',
  'RunsPlayed'
];
const displayEffects = [
  'LocalVariable',
  'WorldVariable',
  'PlayerVariable',

  'Demand',
  'Output',
  'OutputForFeature',
  'Resource',
  'SetFlag',
];

export default {
  props: ['items'],
  components: {
    ProbabilitiesSummary,
    EffectsSummary,
    OutcomesSummary,
    ResourcesSummary,
    ByproductsSummary
  },
  data() {
    return {
      nodes: {},
      edges: [],

      tree: [],
      children: {},

      query: null,
      matchIdx: 0,
      matches: [],

      expanded: false,

      height: 0,
      width: 0,
      offsetX: 0,
      offsetY: 0,
      mousedown: false,
      mouseprev: null
    };
  },
  watch: {
    items() {
      if (this.expanded) {
        this.updateElements();
      }
    }
  },
  computed: {
    size() {
      return {
        width: `${this.width}px`,
        height: `${this.height}px`
      }
    },
    offsets() {
      return {
        top: `${this.offsetY}px`,
        left: `${this.offsetX}px`
      }
    }
  },
  methods: {
    icon(label) {
      return `/static/icons/${icons[label] || icons['Other']}`;
    },
    eventFactors(event) {
      let factors = new Set();
      (event.probabilities || []).forEach((prob) => {
        prob.conditions.forEach((cond) => {
          if (displayFactors.includes(cond.type)) {
            if (cond.type == 'Flag') {
              factors.add('Flag');
            } else if (cond.type == 'RunsPlayed') {
              factors.add('RunsPlayed');
            } else if (cond.type == 'Output') {
              factors.add('Output');
            } else {
              factors.add(cond.subtype);
            }
          }
        });
      });
      return factors;
    },
    displayEffects(item) {
      let effects = new Set();
      (item.effects || []).forEach((effect) => {
        if (displayEffects.includes(effect.type)) {
          if (effect.type == 'SetFlag') {
            effects.add('Flag');
          } else if (effect.type == 'Output') {
            effects.add('Output');
          } else {
            effects.add(effect.subtype);
          }
        }
      });
      return effects;
    },
    search(ev) {
      if (ev.key === 'Enter') {
        let query = ev.target.value;
        if (this.query !== query) {
          let regex = new RegExp(query, 'gi');
          this.matches = Object.values(this.nodes).filter((node) => {
            return node.text.match(regex) !== null;
          });
          this.matchIdx = 0;
          this.query = query;
        } else {
          this.matchIdx++;
          if (this.matchIdx > this.matches.length - 1) {
            this.matchIdx = 0;
          }
        }
        if (this.matches.length > 0) {
          let node = this.matches[this.matchIdx];
          this.offsetY = -node.y;
        }
      }
    },
    itemsOfType(type) {
      return Object.values(this.items)
        .filter((i) => i._type == type);
    },
    validateItem(item) {
      let spec = validate[item._type];
      let valid = spec.validate.every((k) => {
        return spec.validateKey(item, k);
      });
      return valid;
    },
    collapse(ev) {
      this.expanded = false;
      document.body.style.overflow = 'auto';
      ev.stopPropagation();
    },
    expand() {
      if (!this.expanded) this.expanded = true;
      this.updateElements();
      document.body.style.overflow = 'hidden';
    },
    dragStart(ev) {
      if (!this.expanded) return;
      if (ev.which !== 1) return; // Left mouse only
      this.mousedown = true;
      this.mouseprev = {
        x: ev.clientX,
        y: ev.clientY,
      };
    },
    dragEnd() {
      this.mousedown = false;
    },
    pan(ev) {
      if (this.mousedown) {
        let x = ev.clientX;
        let y = ev.clientY;
        let deltaX = x - this.mouseprev.x;
        let deltaY = y - this.mouseprev.y;
        this.offsetX += deltaX;
        this.offsetY += deltaY;
        this.mouseprev = {x, y};
      }
    },
    getDescendants(node) {
      let children = this.children[node];
      return children.concat(children.flatMap((ch) => this.getDescendants(ch)));
    },
    getMaxDepths(nodes, depths, depth) {
      depths = depths || {};
      depth = depth || 0;
      nodes.forEach((node) => {
        depths[node] = depth;
        this.getMaxDepths(this.children[node], depths, depth+1);
      });
      return depths;
    },
    getRoots() {
      let parents = Object.keys(this.tree).reduce((acc, k) => {
        acc[k] = [];
        return acc;
      }, {});
      Object.keys(this.tree).forEach((k) => {
        this.children[k].forEach((ch) => {
          parents[ch].push(k);
        });
      });
      let roots = Object.keys(parents)
        .filter((k) => parents[k].length === 0)
        .filter((k) => ['Event', 'Project'].includes(this.items[k]._type));

      let descendants = {};
      roots.forEach((root) => {
        descendants[root] = this.getDescendants(root);
      });

      let maxDepths = this.getMaxDepths(roots);
      return {roots, descendants, maxDepths};
    },
    dataForNode(node) {
      let data = this.items[node];
      return {
        id: node,
        text: data.name || '[MISSING NAME]',
        type: data._type,
        subtype: subtypeForItem(data),
        valid: this.validateItem(data),
        data
      };
    },
    updateTree() {
      this.tree = Object.values(this.items).filter((item) => {
        return ['Event', 'Project', 'Process'].includes(item._type);
      }).reduce((acc, item) => {
        acc[item.id] = item;
        return acc;
      }, {});
    },
    updateChildren() {
      let children = Object.keys(this.tree).reduce((acc, id) => {
        acc[id] = [];
        return acc;
      }, {});

      let processesByFeature = Object.values(this.tree).filter((item) => {
        return item._type == 'Process';
      }).reduce((acc, process) => {
        Object.keys(consts.PROCESS_FEATURES).forEach((feat) => {
          if (process.features[feat]) {
            acc[feat].push(process.id);
          }
        });
        return acc;
      }, Object.keys(consts.PROCESS_FEATURES).reduce((acc, feat) => {
        acc[feat] = [];
        return acc;
      }, {}));

      Object.values(this.tree).forEach((item) => {
        switch (item._type) {
          case 'Event': {
            let chs = (item.effects || []).flatMap((effect) => childrenForEffect(effect));
            children[item.id].push(...chs);

            // Figure out if this event is influenced by any projects or processes
            (item.probabilities || []).forEach((prob) => {
              prob.conditions.forEach((cond) => {
                if (cond.type.startsWith('Project') || cond.type == 'ProcessMixShare') {
                  children[cond.entity].push(item.id);
                } else if (cond.type == 'ProcessMixShareFeature') {
                  processesByFeature[cond.subtype].forEach((id) => {
                    children[id].push(item.id);
                  });
                }
              });
            });
            break;
          }

          case 'Project': {
            let chs = (item.effects || []).flatMap((effect) => childrenForEffect(effect));
            children[item.id].push(...chs);
            break;
          }
          default:
            break;
        }
      });
      this.children = children;
    },
    updateElements() {
      this.updateTree();
      this.updateChildren();

      let {roots, descendants, maxDepths} = this.getRoots(this.tree);
      let nodes = {};
      let edges = [];

      const nodeHeight = 70;
      const nodeWidth = 140;
      const nodeSpacing = 20;
      const edgeHeight = 4;
      let y = 0;

      const nodesByPositions = {};
      const recurseChildren = (parent) => {
        let paNode = nodes[parent];
        let y = paNode.y;
        this.children[parent].forEach((ch, i) => {
          let existing = nodes[ch];
          if (!existing) {
            if (i > 0) {
              y += nodeHeight + nodeSpacing;
            }
            let x = maxDepths[ch] * (nodeWidth + nodeSpacing) + nodeSpacing;
            let adj = ((nodesByPositions[x] || {})[y] ? nodeHeight + nodeSpacing : 0);
            let yAdj = y + adj;
            nodes[ch] = {
              x, y: yAdj,
              style: {
                top: `${yAdj}px`,
                left: `${x}px`,
                height: `${nodeHeight}px`,
                width: `${nodeWidth}px`,
              },
              ...this.dataForNode(ch)
            };
            if (!(x in nodesByPositions)) {
              nodesByPositions[x] = {};
            }
            nodesByPositions[x][yAdj] = true;
          }
          let node = nodes[ch];
          let start = {
            x: paNode.x + nodeWidth,
            y: paNode.y + nodeHeight/2
          };
          let end = {
            x: node.x,
            y: node.y + nodeHeight/2,
          };
          let p1 = {
            x: end.x,
            y: start.y,
          };
          let p2 = {
            x: start.x,
            y: end.y,
          };
          let bez = new Bezier(start, p1, p2, end);
          edges.push({
            path: bez.toSVG()
          });
          recurseChildren(ch);
        });
        return y;
      }

      let nextIdx = 0;
      while (roots.length > 0) {
        let root = roots.splice(nextIdx, 1);

        y += nodeSpacing;
        let x = nodeSpacing;
        nodes[root] = {
          x, y,
          style: {
            top: `${y}px`,
            left: `${x}px`,
            height: `${nodeHeight}px`,
            width: `${nodeWidth}px`,
          },
          ...this.dataForNode(root)
        };
        y = recurseChildren(root) + nodeHeight;

        if (roots.length === 0) break;

        // Try to find the next root that shares the most descendants with this one
        let nextRoot = roots.reduce((acc, otherRoot) => {
          let setA = new Set(descendants[root]);
          let setB = new Set(descendants[otherRoot]);
          let commonDescendants = new Set([...setA].filter(x => setB.has(x)));
          let n = commonDescendants.size;
          if (acc === null || n > acc.count) {
            return {cand: otherRoot, count: n};
          } else {
            return acc;
          }
        }, null);
        nextIdx = roots.indexOf(nextRoot.cand);
      }

      this.nodes = nodes;
      this.edges = edges;
      this.height = Object.values(nodes).reduce((acc, node) => {
        return Math.max(node.y + nodeHeight, acc);
      }, 0);
      this.width = Object.values(nodes).reduce((acc, node) => {
        return Math.max(node.x + nodeWidth, acc);
      }, 0);
    }
  },
}
</script>

<style>
.graph {
  width: 100%;
  height: 100vh;
  left: 0;
  top: 0;
  position: fixed;
  background: #222;
  overflow: hidden;
  z-index: 10;
}
.graph-body {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  cursor: move;
}
.graph-node {
  position: absolute;
  padding: 0.25em 0.5em;
  background: #eee;
  border: 2px solid #FFC049;
  border-radius: 0.2em;
  display: inline-block;
  font-size: 0.8em;
	cursor: help;
}
.graph-edge {
	stroke: #FFC049;
	fill: none;
	stroke-width: 3px;
}
.graph svg {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
}
.graph-tags {
  position: absolute;
  top: 0;
  left: 0;
  transform: translate(0, -75%);
  font-size: 0.75em;
  display: flex;
}
.graph-tags > div {
  border: 1px solid #000;
  border-radius: 0.2em;
  padding: 0 0.2em;
  margin-right: 0.5em;
  background: #83C8F2;
}
.graph-tags > div.invalid {
  background: #ff4949
}

.graph-node.Process {
  border-color: #29B66D;
}
.graph-node.Process .graph-type {
  background: #29B66D;
}
.graph-node.Project {
  border-color: #7E6CFF;
}
.graph-node.Project .graph-type {
  background: #7E6CFF;
}
.graph-node.Event {
  border-color: #FF736C;
}
.graph-node.Event .graph-type {
  background: #FF736C;
}
.graph-subtype {
  right: 0;
  left: auto;
}
.graph-node-body {
  display: flex;
  align-items: center;
  height: 100%;
  line-height: 1.1;
}
.graph-node-body a {
  color: #000;
  text-decoration: none;
}
.graph-node-body a:hover {
  text-decoration: underline;
}

.graph-collapsed {
	width: 120px;
	height: 60px;
	box-shadow: 2px 2px 12px rgb(0, 0, 0.8);
	opacity: 0.75;
	right: 1em;
	left: auto;
	bottom: 1em;
	top: auto;
  user-select: none;
  border-radius: 0.2em;
}
.graph-collapsed:hover {
  cursor: pointer;
  opacity: 1;
}
.graph-collapsed .graph-body {
  transform: scale(0.5);
  cursor: pointer;
}

.graph-controls {
	position: absolute;
	right: 1em;
	top: 0.5em;
  z-index: 20;
  display: flex;
}
.graph-controls input {
	font-size: 1.4em;
	width: 10em;
	margin-right: 0.5em;
  background: #aaa;
}
.graph-collapse {
	cursor: pointer;
	opacity: 0.7;
  user-select: none;
	color: #aaa;
	font-size: 2em;
}
.graph-collapse:hover {
  opacity: 1;
}

.graph-node:hover .graph-node-details {
  display: block;
}
.graph-node-details {
	padding: 0.5em;
  background: #fff7ee;
	position: absolute;
  border: 1px solid #201f1f;
	display: none;
	box-shadow: 1px 1px 6px rgba(0,0,0,0.9);
  z-index: 2;
  left: 5%;
  border-radius: 0.2em;
  min-width: 240px;
  pointer-events: none;
}
.graph-node-details-meta {
  display: flex;
  width: 100%;
  font-size: 1.1em;
}
.graph-node-details .probability-type,
.graph-node-details .summary-pill {
  font-size: 0.75em;
}
.graph-node-details .probability-type {
  padding: 0 0.25em;
}

.graph-invalid {
	position: absolute;
	right: 0;
	top: 0;
	transform: translate(0, -75%);
	background: #FF4949;
	color: #000;
	padding: 0.1em 0.38em;
	line-height: 1;
	border-radius: 20em;
	font-size: 0.8em;
	border: 1px solid #9f2727;
	text-shadow: 0 0 2px rgba(0,0,0,0.5);
	font-weight: bold;
}

.graph-node-icons {
	position: absolute;
	bottom: 0;
	transform: translate(0, 50%);
	left: 0;
  right: 0;
  display: flex;
  justify-content: space-between;
}
.graph-node-icons > div {
	display: flex;
	background: rgba(0,0,0,0.8);
	padding: 2px;
	border-radius: 0.2em;
	line-height: 1;
}
.graph-icon {
  width: 14px;
}

.graph-expand-icon {
	align-items: center;
	display: flex;
	height: 100%;
}
.graph-expand-icon img {
	height: 32px;
	margin: 0 auto;
	display: block;
}

.graph-node-icons .graph-pill {
	border: 1px solid;
	font-size: 0.75em;
	padding: 0;
  background: #eee;
}
.graph-pill > div {
  padding: 1px 2px;
}
.graph-pill > div:first-child {
	background: #FFC049;
  border-right: 1px solid #000;
}

.graph-node-details .image-preview {
  margin-bottom: 0.5em;
  border-radius: 0.2em;
}
</style>
