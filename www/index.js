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

function getNewPath(e) {
    console.groupCollapsed('getNewPath(', e, ')')
    
    // Read SVG path from textarea
    let path = svgPathTextArea.value;

    // Transform SVG path
    let [l, u, r, d] = getBounds(path);
    let pathCenterX = (r + l) / 2;
    let pathCenterY = (d + u) / 2;
    console.log(l, u, r, d, pathCenterX, pathCenterY);
    let p = svgpath(path).translate(
        originX + pathCenterX,
        originY + pathCenterY,
    );
    p.iterate(console.log);
    let transformed = p.toString();
    console.log(transformed);

    console.groupEnd()
    return transformed;
}

function drawWithStyle(color, width, func) {
    let old_style = ctx.strokeStyle;
    let old_width = ctx.lineWidth;
    try {
        ctx.strokeStyle = color;
        ctx.lineWidth = width;
        func();
    } finally {
        ctx.strokeStyle = old_style;
        ctx.lineWidth = old_width;
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

let rotateSpeed = 1;

let numPhasors = 20
let doRect = true;
let doIntegral = false;

// Phasor animation
let rect = { width: 300, height: 200 }
let line = { z0: { x: -100, y: 30 }, z1: { x: 200, y: 90 } }

let pAni = doRect
    ? wasm.PhasorAnim.rectangle(numPhasors, rect.width, rect.height, doIntegral)
    : wasm.PhasorAnim.line(numPhasors, line.z0.x, line.z0.y, line.z1.x, line.z1.y, doIntegral)

// Arm
let arm = {
    draw() {
        let points = pAni.get_arm_state(originX, originY);
        drawPath(points, "#555");
        drawCircles(points, "#333");
    },
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
        let point = pAni.get_last_point(originX, originY)
        this.push(point)
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
    if (doRect) {
        drawWithStyle("#0af", 1, () => {
            ctx.beginPath();
            ctx.rect(originX - rect.width / 2, 
                     originY - rect.height / 2, 
                     rect.width, rect.height);
            ctx.stroke();
        })
    }
    else {
        drawWithStyle('#0af',1,() => {
            ctx.beginPath()
            ctx.moveTo(originX + line.z0.x, originY + line.z0.y)
            ctx.lineTo(originX + line.z1.x, originY + line.z1.y)
            ctx.stroke()
        })
    }
    

    // Draw Arm
    arm.draw();
    trail.draw();
});
