import * as wasm from "rust-wasm";
import { memory } from "rust-wasm/rust_wasm_bg.wasm";
var svgpath = require("svgpath");
const getBounds = require("svg-path-bounds");
const msPerS = 1000;

// Components
let canvas = document.getElementById("render-canvas");
let ctx = canvas.getContext("2d");
ctx.strokeStyle = "#ffffff";
let width = canvas.width;
let height = canvas.height;
let originX = width / 2;
let originY = height / 2;

function animate(onFrame) {
    let s = Date.now();
    function loop() {
        let dt = (Date.now() - s) / msPerS;
        onFrame({ dt });
        s = Date.now();
        requestAnimationFrame(loop);
    }
    requestAnimationFrame(loop);
}

// Transform SVG path
function transformPath(path, originX, originY) {
    let [l, u, r, d] = getBounds(path);
    return svgpath(path)
        .translate(originX, originY)
        .toString();
}

function drawWithStyle(color, width, func) {
    let old_style = ctx.strokeStyle;
    let old_width = ctx.lineWidth;
    let old_cap = ctx.lineCap
    let old_join = ctx.lineJoin
    try {
        ctx.strokeStyle = color;
        ctx.lineWidth = width;
        ctx.lineCap = 'round'
        ctx.lineJoin = 'round'
        func();
    } finally {
        ctx.strokeStyle = old_style;
        ctx.lineWidth = old_width;
        ctx.lineCap = old_cap
        ctx.lineJoin = old_join
    }
}

function drawPath(points, color = "#fff", width = 1) {
    drawWithStyle(color, width, () => {
        ctx.beginPath();
        points.forEach(({ x, y }, i) => {
            ctx[i == 0 ? "moveTo" : "lineTo"](x, y);
        });
        ctx.stroke();
    });
}

function drawCircles(circles, color = "#fff", width = 1) {
    drawWithStyle(color, width, () => {
        circles.forEach(({ x, y, r }) => {
            if (r > 0) {
                ctx.beginPath();
                ctx.arc(x, y, r, 0, 2 * Math.PI);
                ctx.stroke();
            }
        });
    });
}

let linesOnly = 'M 50 150 L 300 -200 L -50 -40 L -100 10 L -120 -100 L -300 -30 Z'
let heart = 'M140 20C73 20 20 74 20 140c0 135 136 170 228 303 88-132 229-173 229-303 0-66-54-120-120-120-48 0-90 28-109 69-19-41-60-69-108-69z'

let svg = {
    pathstr: linesOnly,

    getPath() {
        let path = wasm.Path.new()
        svgpath(this.pathstr).abs().iterate((e) => {
            switch (e[0]) {
                case 'M':
                    path.move_to(e[1], e[2]);
                case 'L':
                    path.line_to(e[1], e[2]);
                case 'Z':
                    break;
                default:
                    console.error('Unsupported path element type! ' + e[0])
            }
        })
        path.close()
        return path
    },

    draw(ctx, originX, originY) {
        let transformed = transformPath(this.pathstr, originX, originY)
        drawWithStyle('#0af', 1, () => {
            ctx.stroke(new Path2D(transformed))
        })
    }
}

// Phasor animation
let rect = { 
    x0: 50,
    y0: 20,
    width: 300, 
    height: 200,

    getPath() {
        let path = wasm.Path.new()
        let dx = this.width/2
        let dy = this.height/2
        path.move_to(this.x0 + dx, this.y0 - dy)
        path.line_to(this.x0 + dx, this.y0 + dy)
        path.line_to(this.x0 - dx, this.y0 + dy)
        path.line_to(this.x0 - dx, this.y0 - dy)
        path.close()
        return path
    },

    draw(ctx, originX, originY) {
        drawWithStyle("#0af", 1, () => {
            ctx.beginPath();
            ctx.rect(originX - this.width / 2, 
                     originY - this.height / 2, 
                     this.width, this.height);
            ctx.stroke();
        })
    }
}
let line = { 
    z0: { x: -100, y: 30 }, 
    z1: { x: 200, y: 90 },

    getPath() {
        let path = wasm.Path.new()
        path.move_to(this.z0.x, this.z0.y)
        path.line_to(this.z1.x, this.z1.y)
        path.close()
        return path
    },

    draw(ctx, originX, originY) {
        drawWithStyle('#0af',1,() => {
            ctx.beginPath()
            ctx.moveTo(originX + this.z0.x, originY + this.z0.y)
            ctx.lineTo(originX + this.z1.x, originY + this.z1.y)
            ctx.stroke()
        })
    }
}

let obj = svg
let rotateSpeed = 1.25
let numPhasors = 50
let pAni = wasm.PhasorAnim.from_path(numPhasors, obj.getPath())

// Arm
let arm = {
    draw() {
        let points = pAni.get_arm_state(originX, originY);
        drawPath(points, "#555");
        drawCircles(points, "#333");
    }
};

// Trail Points
let trail = {
    max: 100,
    points: [],
    push(point) {
        this.points.push(point);
        while (this.points.length >= this.max) {
            this.points.shift();
        }
    },
    update() {
        this.push(pAni.get_last_point(originX, originY))
    },
    draw() {
        drawPath(this.points, "#fff", 3);
    },
};

animate(({ dt }) => {
    ctx.clearRect(0, 0, width, height);

    pAni.update(rotateSpeed * dt, originX, originY);
    trail.update()

    // Draw SVG
    obj.draw(ctx, originX, originY)

    // Draw Arm and trail
    arm.draw();
    trail.draw();
});
