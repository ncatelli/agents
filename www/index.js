import * as wasm from 'agents';

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";

// Construct the universe, and get its width and height.
const width = wasm.board_width();
const height = wasm.board_height();

const RED_MASK = 0xff << 16;
const GREEN_MASK = 0xff << 8;
const BLUE_MASK = 0xff;

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById('canvas');
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

function drawGrid() {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    let x = i * (CELL_SIZE + 1) + 1;
    let y = (CELL_SIZE + 1) * height + 1;
    ctx.moveTo(x, 0);
    ctx.lineTo(x, y);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    let x = (CELL_SIZE + 1) * width + 1;
    let y = j * (CELL_SIZE + 1) + 1;
    ctx.moveTo(0, y);
    ctx.lineTo(x, y);
  }

  ctx.stroke();

  let colors = wasm.tick();
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      let i = y * height + x;
      let color = colors[i];
      ctx.fillStyle = 'rgb(' + (color & RED_MASK).toString() + ',' + (color & GREEN_MASK).toString() + ',' + (color & BLUE_MASK).toString() + ')';
      ctx.fillRect((CELL_SIZE + 1) * x + 2, (CELL_SIZE + 1) * y + 2, CELL_SIZE - 1, CELL_SIZE - 1);
    }
  }
}

let fps = 10;

function loop() {
  drawGrid();

  setTimeout(() => {
    requestAnimationFrame(loop);
  }, 1000.0 / fps);
}

document.getElementById('runcode').addEventListener('click', () => {
  wasm.run(editor.getValue());
});

loop();

document.getElementById('editor').innerHTML = `agent red_agent:
    set color = 16711680
    set x = 40
    set y = 40
    face NW
    loop:
        move 1
        goto loop
agent blue_agent:
    set color = 255
    set x = 40
    set y = 40
    face NE
    loop:
        move 2
        goto loop
`

let editor = ace.edit('editor');
editor.setTheme("ace/theme/monokai");
