<script lang="ts">
  import Canvas from "./lib/Canvas.svelte";
  import KindButton from "./lib/KindButton.svelte";
  import type { Config } from "./types";
  import { Kind } from "../public/wasm/wasm_crate.js";

  let config: Config = { goalFPS: 60, brushRadius: 7 };
  let selectedKind: Kind = Kind.Sand;

  function selectKind(kind: Kind) {
    selectedKind = kind;
  }

  function updateBrushSize(event: Event) {
    const target = event.target as HTMLInputElement;
    config.brushRadius = parseInt(target.value);
  }
</script>

<main>
  <div class="container">
    <div class="canvas-container">
      <Canvas {config} {selectedKind} />
    </div>
    <div class="divider"></div>
    <div class="toolbar">
      <div class="toolbar-section top-section">
        <div class="slider-container">
          <div class="label">Brush Size</div>
          <input
            type="range"
            id="brush-size"
            min="1"
            max="30"
            value={config.brushRadius}
            on:input={updateBrushSize}
          />
        </div>
      </div>
      <div class="toolbar-divider"></div>
      <div class="toolbar-section bottom-section">
        <div class="button-container">
          <KindButton kind={Kind.Air} {selectedKind} {selectKind} />
          <KindButton kind={Kind.Sand} {selectedKind} {selectKind} />
          <KindButton kind={Kind.Wall} {selectedKind} {selectKind} />
        </div>
      </div>
    </div>
  </div>
</main>

<style>
  main {
    background-color: #f8f5f1;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    width: 100vw;
    padding: 24px;
    box-sizing: border-box;
  }

  .container {
    display: flex;
    gap: 12px;
    padding: 12px;
    height: 100%;
    background-color: #f0e8e0;
    box-sizing: border-box;
    border-radius: 16px;
    border: 4px solid #9a7856;
  }

  .canvas-container {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  .divider {
    width: 3px;
    border-radius: 2px;
    height: 100%;
    margin: auto 0;
    background-color: #9a7856;
  }

  .toolbar {
    display: flex;
    flex-direction: column;
    min-width: 180px;
  }

  .toolbar-section {
    padding: 12px;
  }

  .top-section {
    margin-bottom: 8px;
  }

  .bottom-section {
    flex-grow: 1;
  }

  .toolbar-divider {
    height: 3px;
    border-radius: 2px;
    width: 50%;
    background-color: #9a7856;
    margin: 4px auto;
    /* opacity: 0.7; */
  }

  .slider-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .label {
    font-size: 20px;
    color: #333;
    text-align: center;
    font-size: 20px;
    font-weight: bold;
    font-family: sans-serif;
    color: #5c4732;
  }

  input[type="range"] {
    width: 100%;
    accent-color: #9a7856;
  }

  .button-container {
    display: grid;
    gap: 12px;
    align-content: start;
  }
</style>
