import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 11; // odd # of pixels required
const HALF = CELL_SIZE / 2;
const GRID_COLOR = "#d0d0d0";
const DEAD_COLOR = "#ffffff";
const ALIVE_COLOR = "#333333";

const canvas = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const width = universe.width();
const height = universe.height();

// 1px border around each cell and 1 cell border around overall canvas
canvas.height = (CELL_SIZE + 1) * height + CELL_SIZE;
canvas.width = (CELL_SIZE + 1) * width + CELL_SIZE;

const ctx = canvas.getContext('2d');

let frameReference = null;

const isPaused = () => frameReference === null;

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "Pause";
    renderLoop();
}

const pause = () => {
    playPauseButton.textContent = "Play";
    cancelAnimationFrame(frameReference);
    frameReference = null;
}

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
})

const renderLoop = () => {
    universe.tick();
    drawGrid();
    drawCells();

    frameReference = requestAnimationFrame(renderLoop);
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
    ctx.lineWidth = 1;
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + HALF, HALF);
        ctx.lineTo(i * (CELL_SIZE + 1) + HALF, (CELL_SIZE + 1) * height + HALF);
    }
    for (let i = 0; i <= height; i++) {
        ctx.moveTo(HALF, i * (CELL_SIZE + 1) + HALF);
        ctx.lineTo((CELL_SIZE + 1) * width + HALF, i * (CELL_SIZE + 1) + HALF);
    }
    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
    ctx.beginPath();
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
            ctx.fillRect(
                col * (CELL_SIZE + 1) + HALF,
                row * (CELL_SIZE + 1) + HALF,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);