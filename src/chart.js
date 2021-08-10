const dpr = window.devicePixelRatio || 1;
const tickSize = 10;

class Chart {
  constructor(stageEl, conf) {
    this.stage = stageEl;
    this.config = conf || {};
    this.width = this.stage.clientWidth;
    this.height = this.stage.clientHeight;

    this.canvas = document.createElement('canvas');
    this.ctx = this.canvas.getContext('2d');
    this.setSize();
    this.stage.appendChild(this.canvas);

    this.ranges = conf.ranges;
    this._reset();
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

  _reset() {
    this.scale = {
      x: this.width/(this.ranges.x[1] - this.ranges.x[0]),
      y: this.height/(this.ranges.y[1] - this.ranges.y[0])
    }
    this.origin = {
      x: -this.ranges.x[0] * this.scale.x,
      y: this.ranges.y[1] * this.scale.y
    };
  }

  reset() {
    this._reset();
    this.ctx.clearRect(0, 0, this.width, this.height);
  }

  drawLine(points, color) {
    this.ctx.beginPath();
    this.ctx.lineWidth = 1;
    this.ctx.strokeStyle = color;

    let pixels = points.map((pt) => this.pointToPixel(pt));
    let start = pixels[0];
    this.ctx.moveTo(start.x, start.y);
    pixels.slice(1).forEach((px) => {
      this.ctx.lineTo(px.x, px.y);
    });
    this.ctx.stroke();
    this.ctx.closePath();
  }

  drawHLine(y, color, label) {
    y = this.yToPixel(y);
    this.ctx.beginPath();
    this.ctx.lineWidth = 1;
    this.ctx.strokeStyle = color;
    this.ctx.moveTo(0, y);
    this.ctx.lineTo(this.width, y);
    this.ctx.stroke();
    this.ctx.closePath();
    if (label) {
      // this.ctx.fillStyle = label.background;
      let textWidth = this.ctx.measureText(label.text).width;
      // this.ctx.rect(label.offset, y-12, textWidth, 12);
      // this.ctx.fill();
      this.ctx.fillStyle = label.color;
      this.ctx.fillText(label.text, this.xToPixel(this.ranges.x[1]) - textWidth, y);
    }
  }

  drawVLine(x, color, label) {
    x = this.xToPixel(x);
    this.ctx.beginPath();
    this.ctx.lineWidth = 1;
    this.ctx.strokeStyle = color;
    this.ctx.moveTo(x, 0);
    this.ctx.lineTo(x, this.height);
    this.ctx.stroke();
    this.ctx.closePath();
    if (label) {
      this.ctx.fillStyle = label.color;
      this.ctx.fillText(label.text, x, this.yToPixel(this.ranges.y[0]));
    }
  }

  drawPoint(pt, color, radius) {
    color = color || '#f7120e';
    let rangeSize = this.ranges.x[1] - this.ranges.x[0];
    radius = radius || (rangeSize < 100 ? 3 : 2) * this.width/800;

    let px = this.pointToPixel(pt);
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.arc(px.x, px.y, radius, 0, Math.PI * 2);
    this.ctx.closePath();
    this.ctx.fill();
  }

  xToPixel(x) {
    let pt = this.pointToPixel({x: x, y: 0});
    return pt.x;
  }

  yToPixel(y) {
    let pt = this.pointToPixel({x: 0, y: y});
    return pt.y;
  }

  drawAxes() {
    let origin = this.origin;
    this.ctx.beginPath();
    this.ctx.lineWidth = 1;
    this.ctx.strokeStyle = this.config.axesColor || '#222222';
    this.ctx.moveTo(0, origin.y);
    this.ctx.lineTo(this.width, origin.y);
    this.ctx.moveTo(origin.x, 0);
    this.ctx.lineTo(origin.x, this.height);
    this.ctx.stroke();
    this.ctx.closePath();
  }

  drawTick(from, to, labelPos, label) {
    let fontSize = 6;
    this.ctx.font = `${fontSize}px sans-serif`;

    this.ctx.strokeStyle = '#888888';
    this.ctx.beginPath();
    this.ctx.moveTo(from[0], from[1]);
    this.ctx.lineTo(to[0], to[1]);
    this.ctx.stroke();
    this.ctx.closePath();

    if (label) {
      this.ctx.fillStyle = '#000000';
      let textSize = this.ctx.measureText(label);
      this.ctx.fillText(label, labelPos[0], labelPos[1] - fontSize/2);
    }
  }

  drawYTick(y, size, label) {
    this.drawTick(
        [this.origin.x - size/2, y],
        [this.origin.x + size/2, y],
        [this.origin.x + size,   y + size],
        label
      );
  }

  drawXTick(x, size, label) {
    let px_x = this.xToPixel(x);
    this.drawTick(
        [px_x, this.height-size/2],
        [px_x, this.height+size/2],
        [px_x, this.height-size/2],
        label
      );
  }

  pointToPixel(pt) {
    return {
      x: (pt.x*this.scale.x) + this.origin.x,
      y: -((pt.y*this.scale.y) - this.origin.y)
    };
  }
}

export default Chart;
