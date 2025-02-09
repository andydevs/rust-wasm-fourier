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

// Phasor animation
let rect = { 
    width: 300, 
    height: 200,

    getPath() {
        let path = wasm.Path.new()
        let x = this.width/2
        let y = this.height/2
        path.move_to( x, -y)
        path.line_to( x,  y)
        path.line_to(-x,  y)
        path.line_to(-x, -y)
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

let obj = rect
let rotateSpeed = 1
let numPhasors = 10
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
    obj.draw(ctx, originX, originY)

    // Draw Arm and trail
    arm.draw();
    trail.draw();
});
