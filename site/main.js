var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import init, { init_world } from "../pkg/simulation_wasm.js";
const CELL_SIZE = 5;
init()
    .then(webAssemblyObject => {
    // here we go!
    const xSize = 64, ySize = 64;
    let world = init_world(xSize, ySize);
    const context = getRenderingContext();
    const renderLoop = () => {
        console.log("hey...");
        world.move_by_x_ticks(1);
        drawWorld(world, context, xSize, ySize, webAssemblyObject.memory.buffer);
        sleep(1).then(() => {
            requestAnimationFrame(renderLoop);
        });
    };
    renderLoop();
});
function getRenderingContext() {
    const canvas = document.getElementById("canvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    let context = canvas.getContext("2d");
    if (typeof context === "undefined") {
        throw new Error("Canvas doesn't have 2D context");
    }
    return context;
}
const BYTES_PER_CELL = 8;
function drawWorld(world, context, width, height, memBuffer) {
    let ptr = world.get_cells_pointer();
    const mem = new Uint8Array(memBuffer, ptr, width * height * BYTES_PER_CELL);
    context.beginPath();
    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            context.fillStyle = getColor(x, y, width, mem);
            context.fillRect(x * (CELL_SIZE) + 1, y * (CELL_SIZE) + 1, CELL_SIZE, CELL_SIZE);
        }
    }
    context.stroke();
}
const ENTITY = 0, GROUND = 1, FOOD = 2;
const GROUND_COLOR = "rgb(0, 100, 0)";
const FOOD_COLOR = "yellow";
function getColor(x, y, width, mem) {
    const index = (y * width + x) * BYTES_PER_CELL;
    if (mem[index] == GROUND) {
        return GROUND_COLOR;
    }
    else if (mem[index] == FOOD) {
        return FOOD_COLOR;
    }
    else if (mem[index] == ENTITY) {
        return `rgb(${mem[index + 1]}, ${mem[index + 2]}, ${mem[index + 3]})`;
    }
    else {
        throw new Error("Unknown cell type!");
    }
}
function sleep(ms) {
    return __awaiter(this, void 0, void 0, function* () {
        return new Promise(resolve => setTimeout(resolve, ms));
    });
}
