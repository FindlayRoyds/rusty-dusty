import init, { Game } from "../public/wasm/wasm_crate.js";
import bresenham from "bresenham";

const NUM_CELLS_X = 200;
const NUM_CELLS_Y = 200;
const LOGGING = true;
interface Config {
  goalFPS: number; // essentially goal fps
  brushRadius: number;
}

function testPerformance(name: string, fn: () => void) {
  if (!LOGGING) {
    fn();
    return;
  }
  const start = performance.now();
  fn();
  const end = performance.now();
  console.log(`${name} took ${end - start} ms`);
}

function resizeCanvas(canvas: HTMLCanvasElement) {
  const width = window.innerWidth;
  const height = window.innerHeight;

  const cellSize = Math.min(width / NUM_CELLS_X, height / NUM_CELLS_Y);

  canvas.width = NUM_CELLS_X * cellSize;
  canvas.height = NUM_CELLS_Y * cellSize;

  return cellSize;
}

function addClickListener(
  canvas: HTMLCanvasElement,
  grid: Game,
  pixelSizeRef: { value: number },
  config: Config
) {
  let isDragging = false;
  let lastX: number | null = null;
  let lastY: number | null = null;
  let lastTime = Date.now();

  const handleMouseMove = (event: MouseEvent) => {
    if (isDragging) {
      const rect = canvas.getBoundingClientRect();
      const x = event.clientX - rect.left;
      const y = event.clientY - rect.top;

      const cellX = Math.floor(x / pixelSizeRef.value);
      const cellY = Math.floor(y / pixelSizeRef.value);
      if (cellX == lastX && cellY == lastY) {
        return;
      }

      if (lastX !== null && lastY !== null) {
        const points = bresenham(lastX, lastY, cellX, cellY);
        points.shift();
        const timeStep = (Date.now() - lastTime) / points.length;
        points.forEach((point, index) => {
          grid.click_at(
            point.x,
            point.y,
            config.brushRadius,
            lastTime + index * timeStep
          );
        });
      } else {
        grid.click_at(cellX, cellY, config.brushRadius, Date.now());
      }

      lastX = cellX;
      lastY = cellY;
      lastTime = Date.now();
    }
  };

  const handleMouseDown = (event: MouseEvent) => {
    isDragging = true;
    handleMouseMove(event);
  };

  const handleMouseUp = () => {
    isDragging = false;
    lastX = null;
    lastY = null;
  };

  canvas.addEventListener("mousedown", handleMouseDown);
  canvas.addEventListener("mousemove", handleMouseMove);
  canvas.addEventListener("mouseup", handleMouseUp);
  canvas.addEventListener("mouseleave", handleMouseUp);
}

async function runWasm(config: Config) {
  await init();

  const canvas = document.getElementById("display_canvas") as HTMLCanvasElement;
  const grid = new Game(NUM_CELLS_X, NUM_CELLS_Y);
  testPerformance("initialising", () => grid.initialise());

  let pixelSizeRef = { value: Math.floor(resizeCanvas(canvas)) };

  window.addEventListener("resize", () => {
    pixelSizeRef.value = Math.floor(resizeCanvas(canvas));
    grid.draw(canvas, pixelSizeRef.value);
  });

  addClickListener(canvas, grid, pixelSizeRef, config);

  setInterval(
    () => {
      testPerformance("updating", () => grid.update());
      testPerformance("drawing", () => grid.draw(canvas, pixelSizeRef.value));
    },
    config.goalFPS == -1 ? 0 : 1000 / config.goalFPS
  );
}

let config: Config = { goalFPS: 60, brushRadius: 15 };
runWasm(config);
