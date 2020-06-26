import { Universe } from "wasm-playground";
import { memory } from "wasm-playground/wasm_playground_bg"

const width = 256;
const height = 80;

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext('2d');

const universe = Universe.new(width, height);
const image = new ImageData(new Uint8ClampedArray(memory.buffer, universe.texture(), 4 * width * height), width, height);

const renderLoop = () => {
    universe.tick();
    ctx.putImageData(image, 0, 0);
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);