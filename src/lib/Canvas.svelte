<script lang="ts">
  import { onMount } from "svelte";
  import init, { Game } from "../../public/wasm/wasm_crate.js";
  import type { Config } from "../types";
  import bresenham from "bresenham";

  export let config = { goalFPS: 60, brushRadius: 7 };
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

  function resizeCanvas(canvas: HTMLCanvasElement) {
    canvas.width = NUM_CELLS_X;
    canvas.height = NUM_CELLS_Y;
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
          //   line from last mouse point so the drawing isn't patchy
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

  async function runWasm() {
    await init();

    //   const canvas = document.getElementById("display_canvas") as HTMLCanvasElement;
    grid = new Game(NUM_CELLS_X, NUM_CELLS_Y);
    testPerformance("initialising", () => grid.initialise());

    let pixelSizeRef = { value: resizeCanvas(canvas) };

    window.addEventListener("resize", () => {
      pixelSizeRef.value = resizeCanvas(canvas);
      grid.draw(canvas);
    });

    let mouse_callback = addMouseListener(canvas, grid, config);

    setInterval(
      () => {
        testPerformance("updating", () => grid.update());
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
    display: block;
    margin: auto;
    border: 1px solid black;
    width: 100vh;
    height: 100vh;
    image-rendering: pixelated;
    image-rendering: crisp-edges;
  }
</style>
