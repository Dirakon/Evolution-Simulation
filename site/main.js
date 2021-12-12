import init, { init_world } from "../pkg/simulation_wasm.js";
//import {memory} from '../pkg/simulation_wasm_bg.wasm'
init()
    .then(obj => {
    // here we go!
    const xSize = 3, ySize = 4;
    let world = init_world(xSize, ySize);
    const canvas = document.getElementById("canvas");
    let context = canvas.getContext("2d");
    const renderLoop = () => {
        world.move_by_x_ticks(1);
        drawWorld(world, context, xSize, ySize, obj.memory.buffer);
        requestAnimationFrame(renderLoop);
    };
    renderLoop();
});
const CELL_SIZE = 5;
function drawWorld(world, context, width, height, memBuffer) {
    let ptr = world.get_pixels_pointer();
    const mem = new Uint8Array(memBuffer, ptr, width * height * 3);
    context.beginPath();
    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            context.fillStyle = getColor(x, y, width, mem);
            context.fillRect(x * (CELL_SIZE + 1) + 1, y * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
        }
    }
    context.stroke();
}
function getColor(x, y, width, mem) {
    const index = (y * width + x) * 3;
    return `rgb(${mem[index]}, ${mem[index + 1]}, ${mem[index + 2]})`;
}
