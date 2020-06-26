import { Universe } from "wasm-playground";
import { memory } from "wasm-playground/wasm_playground_bg"

const width = 1024;
const height = 512;

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext('2d');

const universe = Universe.new(width, height);

const drawCells = () => {
    const cellsPtr = universe.cells();
    const image = new ImageData(new Uint8ClampedArray(memory.buffer, cellsPtr, 4 * width * height), width, height);
    ctx.putImageData(image, 0, 0);
};

const renderLoop = () => {
    universe.tick();
    drawCells();
    requestAnimationFrame(renderLoop);
};

drawCells();
requestAnimationFrame(renderLoop);