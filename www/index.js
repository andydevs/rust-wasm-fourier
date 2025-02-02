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

// Display SVG
// let path = getNewPath({})
// ctx.stroke(new Path2D(path))

// Display Rect
// let rwidth = 200
// let rheight = 200
// ctx.beginPath()
// ctx.rect(originX - rwidth/2, originY - rheight/2, rwidth, rheight)
// ctx.stroke()

// Draw phasor
let pAni = wasm.PhasorAnimation.simple();


animate(({ dt }) => {
    pAni.update(dt);
    let arm_state = pAni.get_arm_state();
    let trail = pAni.get_trail_state();

    // Construct arm from state
    let arm = arm_state.map((s) => ({
        x: originX + s.x,
        y: originY + s.y,
        r: s.r
    }))

    ctx.clearRect(0, 0, width, height)

    ctx.strokeStyle = "#555";
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(originX, originY)
    arm.forEach(({ x, y }) => {
        ctx.lineTo(x, y)
    })
    ctx.stroke()

    ctx.strokeStyle = "#333"
    ctx.lineWidth = 1;
    arm.forEach(({ x, y, r }) => {
        ctx.beginPath()
        ctx.arc(x, y, r, 0, 2 * Math.PI)
        ctx.stroke()
    })

    ctx.strokeStyle = "#fff";
    ctx.lineWidth = 3
    ctx.beginPath()
    trail.forEach(({ x, y }, i) => {
        ctx[i == 0 ? 'moveTo' : 'lineTo'](originX + x, originY + y)
    })
    ctx.stroke()
});
