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
    let cx = (r - l) / 2
    let cy = (d - u) / 2
    return svgpath(path)
        .translate(originX - cx, originY - cy)
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

let heart = 'M140 20C73 20 20 74 20 140c0 135 136 170 228 303 88-132 229-173 229-303 0-66-54-120-120-120-48 0-90 28-109 69-19-41-60-69-108-69z'

class SVGPath {
    constructor(pathstr) {
        this.pathstr = pathstr
    }

    getPath() {
        let path = wasm.Path.new()
        let [l, u, r, d] = getBounds(this.pathstr)
        let cx = (r - l) / 2
        let cy = (d - u) / 2
        svgpath(this.pathstr)
            .abs()
            .translate(-cx, -cy)
            .iterate((e) => {
                switch (e[0]) {
                    case 'M':
                        path.move_to(e[1], e[2]);
                        break;
                    case 'L':
                        path.line_to(e[1], e[2]);
                        break;
                    case 'C':
                        path.curve_to(e[1], e[2], e[3], e[4], e[5], e[6])
                        break;
                    case 'Z':
                        path.close()
                        break;
                    default:
                        console.error('Unsupported path element type!', e)
                }
            })
        return path
    }

    draw(ctx, originX, originY) {
        let transformed = transformPath(this.pathstr, originX, originY)
        drawWithStyle('#0af', 1, () => {
            ctx.stroke(new Path2D(transformed))
        })
    }
}


let obj = new SVGPath(heart)
let rotateSpeed = 1.25
let numPhasors = 100
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
        drawPath(this.points, "#ff0", 5);
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
