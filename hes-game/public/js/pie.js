const dpr = window.devicePixelRatio || 1;

// https://gist.github.com/nikolas/b0cce2261f1382159b507dd492e1ceef?permalink_comment_id=3947508#gistcomment-3947508
const lerpColor = function(pFrom, pTo, pRatio) {
  const ar = (pFrom & 0xFF0000) >> 16,
        ag = (pFrom & 0x00FF00) >> 8,
        ab = (pFrom & 0x0000FF),

        br = (pTo & 0xFF0000) >> 16,
        bg = (pTo & 0x00FF00) >> 8,
        bb = (pTo & 0x0000FF),

        rr = ar + pRatio * (br - ar),
        rg = ag + pRatio * (bg - ag),
        rb = ab + pRatio * (bb - ab);

  return (rr << 16) + (rg << 8) + (rb | 0);
};

const LABEL_MAX_WIDTH = 40;
const LABEL_LINE_HEIGHT = 12;

class PieChart {
  constructor(stageEl) {
    this.stage = stageEl;
    this.width = this.stage.clientWidth;
    this.height = this.stage.clientHeight;

    this.canvas = document.createElement('canvas');

    this.tooltip = document.createElement('div');
    this.tooltip.style.display = 'none';
    this.tooltip.style.position = 'absolute';
    this.tooltip.style.fontSize = '12px';
    this.tooltip.style.fontFamily = 'W95FA, sans-serif';
    // this.tooltip.style.background = '#000';
    // this.tooltip.style.color = '#fff';
    // this.tooltip.style.padding = '2px';
    this.stage.style.position = 'relative';
    this.stage.appendChild(this.tooltip);

    this.canvas.addEventListener('mousemove', (ev) => {
      if (this.radius && this.labels) {
        let bounds = ev.target.getBoundingClientRect();
        let x = ev.clientX - bounds.left;
        let y = ev.clientY - bounds.top;
        let label = this.labelAtPoint(x, y);
        if (label && !label.show) {
          this.tooltip.innerText = label.label;
          this.tooltip.style.left = `${label.x}px`;
          this.tooltip.style.top = `${label.y}px`;
          this.tooltip.style.display = 'block';
        } else {
          this.tooltip.style.display = 'none';
        }
      }
    })

    this.ctx = this.canvas.getContext('2d');
    this.setSize();
    this.stage.appendChild(this.canvas);

    this.reset();
  }

  setSize() {
    this.width = this.stage.clientWidth;
    this.height = this.stage.clientHeight;
    this.canvas.width = this.width * dpr;
    this.canvas.height = this.height * dpr;
    this.canvas.style.width = `${this.width}px`;
    this.canvas.style.height = `${this.height}px`;
    this.ctx.scale(dpr, dpr);
  }

  reset() {
    this.ctx.clearRect(0, 0, this.width, this.height);

    this.ctx.font = '12px "W95FA"';
  }

  // Data should be in the format of {label: number}
  render(data, colors) {
    let outlineWidth = 0;
    let total = Object.values(data).reduce((acc, val) => acc + val, 0);
    let center = {x: this.width/2, y: this.height/2};
    let radius = Math.min(this.width/2, this.height/2) - outlineWidth * 2;
    let lastAngle = 0;
    let c = 2 * Math.PI;
    this.labels = [];
    this.labelsSteps = [];

    // Outline
    this.ctx.beginPath();
    this.ctx.arc(center.x, center.y, radius+outlineWidth, 0, 2 * Math.PI);
    this.ctx.fillStyle = '#222222';
    // this.ctx.fill();
    this.ctx.closePath();

    // Filter to keys that have an amount that would show up
    // Then recalculate total
    let keys = Object.keys(data).filter((k) => data[k]/total >= 0.01);
    total = keys.reduce((acc, k) => acc + data[k], 0);

    keys.forEach((k, i) => {
      let color = lerpColor(colors[0], colors[1], i/keys.length);
      let val = data[k]/total;
      let endAngle = lastAngle + (val * c);
      this.ctx.beginPath();
      this.ctx.arc(center.x, center.y, radius, lastAngle, endAngle);
      this.ctx.lineTo(center.x, center.y);
      this.ctx.fillStyle = `#${color.toString(16)}`;
      this.ctx.fill();
      this.ctx.closePath();

      let size = endAngle-lastAngle;
      let angle = lastAngle + size/2;
      let x = radius/2 * Math.cos(angle);
      let y = radius/2 * Math.sin(angle);
      this.labels.push({
        show: size > Math.PI/12,
        x: center.x + x,
        y: center.y + y,
        label: k
      });
      this.labelsSteps.push(endAngle);

      lastAngle = endAngle;
    });

    // Labels have to come over so they're drawn on top
    this.labels.forEach((l) => {
      this.ctx.fillStyle = '#000000';
      if (l.show) {
        let parts = l.label.split(' ');
        let cur = '';
        let lines = [];
        let maxWidth = 0;
        while (parts.length > 0) {
          cur += parts.shift() + ' ';
          let width = this.ctx.measureText(cur).width;
          if (width > maxWidth) maxWidth = width;
          if (width >= LABEL_MAX_WIDTH) {
            lines.push(cur);
            cur = '';
          }
        }
        if (cur.length > 0) {
          lines.push(cur);
        }

        let y = l.y - ((lines.length - 1) * LABEL_LINE_HEIGHT)/2;
        lines.forEach((line) => {
          this.ctx.beginPath();
          this.ctx.fillText(line, l.x - maxWidth/2, y);
          y += LABEL_LINE_HEIGHT;
        });
      }
    });

    this.radius = radius;
  }

  labelAtPoint(x, y) {
    let center = {x: this.width/2, y: this.height/2};
    x -= center.x;
    y -= center.y;
    let dist = Math.sqrt(x**2 + y**2);
    if (dist < this.radius) {
      let ang = Math.atan2(y, x);
      ang += Math.PI * 2;
      ang %= Math.PI * 2;
      let index = this.labelsSteps.findIndex(n => n >= ang);
      let label = this.labels[index];
      return label;
    }
  }
}

export { PieChart };
