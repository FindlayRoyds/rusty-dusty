<script lang="ts">
  import { onMount } from "svelte";
  import init, { Game } from "../../public/wasm/wasm_crate.js";
  import type { Config } from "../types";
  import bresenham from "bresenham";

  export let config: Config;

  let canvas: HTMLCanvasElement;
  let grid: Game;
  let pixelSizeRef = { value: 1 };

  const NUM_CELLS_X = 200;
  const NUM_CELLS_Y = 200;
  const LOGGING = false;

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

  function addMouseListener(
    canvas: HTMLCanvasElement,
    grid: Game,
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
        const scaleX = canvas.width / rect.width;
        const scaleY = canvas.height / rect.height;

        x = (event.clientX - rect.left) * scaleX;
        y = (event.clientY - rect.top) * scaleY;
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
          // Draw a line from the last mouse point so the drawing isn't patchy
          const points = bresenham(lastX, lastY, cellX, cellY);
          const timeStep = (Date.now() - lastTime) / points.length;
          points.forEach((point, index) => {
            grid.click_at(
              point.x,
              point.y,
              config.brushRadius,
              lastTime + index * timeStep,
              config.selectedKind
            );
          });
        } else {
          grid.click_at(
            cellX,
            cellY,
            config.brushRadius,
            Date.now(),
            config.selectedKind
          );
        }

        lastX = cellX;
        lastY = cellY;
        lastTime = Date.now();
      }
    };

    canvas.addEventListener("mousedown", handleMouseDown);
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUp);
    // canvas.addEventListener("mouseleave", handleMouseUp);

    return update;
  }

  async function runWasm() {
    await init();

    grid = new Game(NUM_CELLS_X, NUM_CELLS_Y);
    testPerformance("initialising", () => grid.initialise());

    canvas.width = NUM_CELLS_X;
    canvas.height = NUM_CELLS_Y;

    let mouse_callback = addMouseListener(canvas, grid, config);

    setInterval(
      () => {
        testPerformance("updating", () => grid.update(config.useHashbrown));
        testPerformance("drawing", () => grid.draw(canvas));
        mouse_callback();
      },
      config.goalFPS == -1 ? 0 : 1000 / config.goalFPS
    );
  }

  onMount(() => {
    if (!canvas) {
      console.error("Canvas not initialized before running WASM.");
      return;
    }
    runWasm();
  });
</script>

<canvas bind:this={canvas} id="display_canvas"></canvas>

<style>
  canvas {
    image-rendering: pixelated;
    image-rendering: crisp-edges;
    height: 100%;
  }
</style>
