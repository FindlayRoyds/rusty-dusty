import init, { Grid } from "../public/wasm/wasm_crate.js";

async function runWasm() {
  let rustWasm = await init();

  const canvas = document.getElementById("display_canvas") as HTMLCanvasElement;
  const game = new Grid(100, 100);

  setInterval(() => {});

  game.draw_on_canvas(canvas);
}

runWasm();
