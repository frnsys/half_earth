const dpr = window.devicePixelRatio || 1;

class Chart {
  constructor(stageEl, ranges) {
    this.stage = stageEl;
    this.width = this.stage.clientWidth;
    this.height = this.stage.clientHeight;

    this.canvas = document.createElement('canvas');
    this.ctx = this.canvas.getContext('2d');
    this.setSize();
    this.stage.appendChild(this.canvas);

    this.ranges = ranges;
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

  _drawLine(pixels, color) {
    this.ctx.beginPath();
    this.ctx.lineWidth = 1;
    this.ctx.strokeStyle = color;
    let start = pixels[0];
    this.ctx.moveTo(start.x, start.y);
    pixels.slice(1).forEach((px) => {
      this.ctx.lineTo(px.x, px.y);
    });
    this.ctx.stroke();
    this.ctx.closePath();
  }

  drawLine(points, color) {
    let pixels = points.map((pt) => this.pointToPixel(pt));
    this._drawLine(pixels, color);
  }

  drawHLine(y, color) {
    y = this.yToPixel(y);
    this._drawLine([{
      x: 0, y
    }, {
      x: this.width, y
    }], color);
  }

  drawVLine(x, color) {
    x = this.xToPixel(x);
    this._drawLine([{
      x, y: 0
    }, {
      x, y: this.height
    }], color);
  }

  drawLabel(text, point, {background, color, anchor}={}) {
    let px = this.pointToPixel(point);
    let {width} = this.ctx.measureText(text);
    let height = 8;
    let padding = 2;

    if (anchor === 'RIGHT') {
      px.x -= width;
    } else if (anchor === 'CENTER') {
      px.x -= width/2;
    }

    if (background) {
      this.ctx.fillStyle = background;
      this.ctx.fillRect(
        px.x - padding,
        px.y - height - padding,
        width + 2*padding,
        height + 2*padding);
    }
    this.ctx.fillStyle = color || '#000000';
    this.ctx.fillText(text, px.x, px.y);
  }

  drawPoint(pt, {color, radius}) {
    color = color || '#f7120e';
    radius = radius || 2;

    let px = this.pointToPixel(pt);
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.arc(px.x, px.y, radius, 0, Math.PI * 2);
    this.ctx.closePath();
    this.ctx.fill();
  }

  xToPixel(x) {
    return this.pointToPixel({x, y: 0}).x;
  }

  yToPixel(y) {
    return this.pointToPixel({x: 0, y}).y;
  }

  drawAxes() {
    let origin = this.origin;
    this._drawLine([{
      x: 0, y: origin.y
    }, {
      x: this.width,
      y: origin.y
    }], '#222222');
    this._drawLine([{
      x: origin.x, y: 0
    }, {
      x: origin.x,
      y: this.height
    }], '#222222');
  }

  pointToPixel(pt) {
    return {
      x: (pt.x*this.scale.x) + this.origin.x,
      y: -((pt.y*this.scale.y) - this.origin.y)
    };
  }
}

export default Chart;
