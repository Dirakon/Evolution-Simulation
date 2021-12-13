import init, { init_world } from "../pkg/simulation_wasm.js";
const CELL_SIZE = 5;
const GROUND_TILE_COLOR = [0, 0, 0];
init()
    .then(webAssemblyObject => {
    // here we go!
    const xSize = 3, ySize = 4;
    let world = init_world(xSize, ySize);
    const context = getRenderingContext();
    const renderLoop = () => {
        world.move_by_x_ticks(1);
        drawWorld(world, context, xSize, ySize, webAssemblyObject.memory.buffer);
        requestAnimationFrame(renderLoop);
    };
    renderLoop();
});
function getRenderingContext() {
    const canvas = document.getElementById("canvas");
    let context = canvas.getContext("2d");
    if (typeof context === "undefined") {
        throw new Error("Canvas doesn't have 2D context");
    }
    return context;
}
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
