import init, * as wasm from "./wasm.js"
const SCALE = 3
const WIDTH = 160
const HEIGHT = 144

let canvas = document.getElementById("canvas")
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE

let ctx = canvas.getContext("2d")
ctx.fillStyle = "#FFFFFF"
ctx.fillRect(0, 0, canvas.width, canvas.height)

async function run() {
    await init()
    let gb = new wasm.GB()

    document.getElementById("fileinput").addEventListener("change", function (e) {
        let file = e.target.files[0]
        if (!file) {
            alert("Failed to read file")
            return
        }

        let fr = new FileReader()
        fr.onload = function () {
            let buffer = fr.result
            const rom = new Uint8Array(buffer)
            gb.load_rom(rom)
        }

        fr.readAsArrayBuffer(file)
    }, false)
}

run().catch(console.error)