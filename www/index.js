import * as wasm from "rust-wasm";
import { memory } from "rust-wasm/rust_wasm_bg.wasm";
var svgpath = require("svgpath");
const getBounds = require("svg-path-bounds");
const msPerS = 1000;

// Components
let svgPathTextArea = document.getElementById("svgpath");
let canvas = document.getElementById("render-canvas");
let renderBtn = document.getElementById("render-button");
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

function getNewPath(e) {
    // Read SVG path from textarea
    let path = svgPathTextArea.value;

    // Transform SVG path
    let [l, u, r, d] = getBounds(path);
    let pathCenterX = (r - l) / 2;
    let pathCenterY = (d - u) / 2;
    let transformed = svgpath(path)
        .translate(originX - pathCenterX, originY - pathCenterY)
        .toString();

    return transformed;
}

function drawWithStyle(color, width, func) {
    let old_style = ctx.strokeStyle
    let old_width = ctx.lineWidth;
    try {
        ctx.strokeStyle = color
        ctx.lineWidth = width
        func()
    }
    finally {
        ctx.strokeStyle = old_style
        ctx.lineWidth = old_width
    }
}

function drawPath(points, color='#fff', width=1) {
    drawWithStyle(color, width, () => {
        ctx.beginPath()
        points.forEach(({ x, y }, i) => {
            ctx[i == 0 ? 'moveTo' : 'lineTo'](x, y)
        })
        ctx.stroke()
    })
}

function drawCircles(circles, color='#fff', width=1) {
    drawWithStyle(color, width, () => {
        circles.forEach(({ x, y, r }) => {
            if (r > 0) {
                ctx.beginPath()
                ctx.arc(x, y, r, 0, 2 * Math.PI)
                ctx.stroke()
            }
        })
    })
}

let randomRange = (a, b) => {
    let t = Math.random()
    return a*(1 - t) + b*t
}
let randomChoice = (elems) => {
    let t = Math.floor(Math.random()*elems.length)
    return elems[t]
}

// Display SVG
// let path = getNewPath({})
// ctx.stroke(new Path2D(path))

let rotateSpeed = 1

let r_w = 300
let r_h = 200

// Phasor animation
let pAni = wasm.PhasorAnim.rectangle(r_w, r_h)

// Arm
let arm = {
    draw() {
        let points = pAni.get_arm_state(originX, originY)
        drawPath(points, '#555')
        drawCircles(points, '#333')
    }
}

// Trail Points
let trail = { 
    max: 100, 
    points: [],
    push(point) {
        this.points.push(point)
        while (this.points.length >= this.max) {
            this.points.shift()
        }
    },
    draw() { 
        drawPath(this.points, '#fff', 3) 
    }
}

animate(({ dt }) => {
    ctx.clearRect(0, 0, width, height)
    
    pAni.update(rotateSpeed*dt, originX, originY);
    let point = pAni.get_last_point(originX, originY);
    trail.push(point)

    // Draw SVG
    drawWithStyle('#0af',1,() => {
        ctx.beginPath()
        ctx.rect(originX - r_w/2, originY - r_h/2, r_w, r_h)
        ctx.stroke()
    })

    // Draw Arm
    arm.draw()
    trail.draw()
});
