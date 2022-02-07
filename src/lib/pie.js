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

class PieChart {
  constructor(stageEl) {
    this.stage = stageEl;
    this.width = this.stage.clientWidth;
    this.height = this.stage.clientHeight;

    this.canvas = document.createElement('canvas');
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
  }

  // Data should be in the format of {label: number}
  render(data, colors) {
    let outlineWidth = 1;
    let total = Object.values(data).reduce((acc, val) => acc + val, 0);
    let center = {x: this.width/2, y: this.height/2};
    let radius = Math.min(this.width/2, this.height/2) - outlineWidth * 2;
    let lastAngle = 0;
    let c = 2 * Math.PI;
    let labels = [];

    // Outline
    this.ctx.beginPath();
    this.ctx.arc(center.x, center.y, radius+outlineWidth, 0, 2 * Math.PI);
    this.ctx.fillStyle = '#222222';
    this.ctx.fill();
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
      if (size > Math.PI/12) {
        let angle = lastAngle + size/2;
        let x = radius/2 * Math.cos(angle);
        let y = radius/2 * Math.sin(angle);
        let {width} = this.ctx.measureText(k);
        labels.push({
          x: center.x + x - width/2,
          y: center.y + y,
          label: k
        });
    }

      lastAngle = endAngle;
    });

    // Labels have to come over so they're drawn on top
    this.ctx.fillStyle = '#000000';
    labels.forEach(({x, y, label}) => {
      this.ctx.fillText(label, x, y);
    });
  }
}

export default PieChart;
