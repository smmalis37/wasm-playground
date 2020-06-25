import { Universe } from "wasm-playground";
import { memory } from "wasm-playground/wasm_playground_bg"

const width = 64;
const height = 64;
const CELL_SIZE = 8; // px

const canvas = document.getElementById("game-of-life-canvas");
const universe = Universe.new(width, height);
const ctx = canvas.getContext('2d');

const GRID_COLOR = "#CCCCCC";

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const getIndex = (row, column) => {
    return row * width + column;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint32Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            const cell = cells[idx];

            ctx.fillStyle = "#" + cell.toString(16).padStart(6, "0");

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

const updateUI = () => {
    drawGrid();
    drawCells();
}

const renderLoop = () => {
    universe.tick();
    updateUI();
    requestAnimationFrame(renderLoop);
};

updateUI();
requestAnimationFrame(renderLoop);