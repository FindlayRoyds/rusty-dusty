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

function resizeCanvas(canvas: HTMLCanvasElement): number {
  const width = window.innerWidth;
  const height = window.innerHeight;

  const cellSize = Math.floor(
    Math.min(width / NUM_CELLS_X, height / NUM_CELLS_Y)
  );

  canvas.width = NUM_CELLS_X * cellSize;
  canvas.height = NUM_CELLS_Y * cellSize;

  return cellSize;
}

function addMouseListener(
  canvas: HTMLCanvasElement,
  grid: Game,
  pixelSizeRef: { value: number },
  config: Config
): () => void {
  let isDragging = false;
  let x = 0;
  let y = 0;
  let lastX: number | null = null;
  let lastY: number | null = null;
  let lastTime = Date.now();

  const handleMouseMove = (event: MouseEvent) => {
    if (isDragging) {
      const rect = canvas.getBoundingClientRect();
      x = event.clientX - rect.left;
      y = event.clientY - rect.top;
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

  const update = () => {
    if (isDragging) {
      const cellX = x / pixelSizeRef.value;
      const cellY = y / pixelSizeRef.value;

      if (lastX !== null && lastY !== null) {
        // line from last mouse point so the drawing isn't patchy
        const points = bresenham(lastX, lastY, cellX, cellY);
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

  canvas.addEventListener("mousedown", handleMouseDown);
  canvas.addEventListener("mousemove", handleMouseMove);
  canvas.addEventListener("mouseup", handleMouseUp);
  canvas.addEventListener("mouseleave", handleMouseUp);

  return update;
}

async function runWasm(config: Config) {
  await init();

  const canvas = document.getElementById("display_canvas") as HTMLCanvasElement;
  const grid = new Game(NUM_CELLS_X, NUM_CELLS_Y);
  testPerformance("initialising", () => grid.initialise());

  let pixelSizeRef = { value: Math.floor(resizeCanvas(canvas)) };

  window.addEventListener("resize", () => {
    pixelSizeRef.value = resizeCanvas(canvas);
    grid.draw(canvas, pixelSizeRef.value);
  });

  let mouse_callback = addMouseListener(canvas, grid, pixelSizeRef, config);

  setInterval(
    () => {
      testPerformance("updating", () => grid.update());
      testPerformance("drawing", () => grid.draw(canvas, pixelSizeRef.value));
      mouse_callback();
    },
    config.goalFPS == -1 ? 0 : 1000 / config.goalFPS
  );
}

let config: Config = { goalFPS: 60, brushRadius: 7 };
runWasm(config);
