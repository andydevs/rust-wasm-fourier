import * as wasm from "rust-wasm";
var svgpath = require('svgpath')
const getBounds = require('svg-path-bounds')

// Components
let svgPathTextArea = document.getElementById('svgpath')
let canvas = document.getElementById('render-canvas')
let renderBtn = document.getElementById('render-button')
let ctx = canvas.getContext('2d')
ctx.strokeStyle = '#ffffff'
let width = canvas.width
let height = canvas.height
let originX = width/2
let originY = height/2

function getNewPath(e) {
    // Read SVG path from textarea
    let path = svgPathTextArea.value

    // Transform SVG path
    let [l, u, r, d] = getBounds(path)
    let pathCenterX = (r - l)/2
    let pathCenterY = (d - u)/2
    let transformed = svgpath(path)
        .translate(originX - pathCenterX, originY - pathCenterY)
        .toString()

    return transformed
}



// Display
let path = getNewPath({})
ctx.stroke(new Path2D(path))