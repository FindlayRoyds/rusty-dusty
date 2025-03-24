<script lang="ts">
  import { Kind } from "../../public/wasm/wasm_crate.js";
  export let kind: Kind;
  export let selectedKind: Kind;
  export let selectKind: (kind: Kind) => void;

  function handleClick() {
    selectKind(kind);
  }

  const kindNames = {
    [Kind.Air]: "Air",
    [Kind.Sand]: "Sand",
    [Kind.Wall]: "Wall",
  };
  const kindColors = {
    [Kind.Air]: "#add8e6",
    [Kind.Sand]: "#e6c88e",
    [Kind.Wall]: "#8b5a2b",
  };
  const kindEmojis = {
    [Kind.Air]: "🌬️",
    [Kind.Sand]: "🏖️",
    [Kind.Wall]: "🧱",
  };

  $: selected = kind === selectedKind;
  $: borderColor = selected ? kindColors[kind] : "#9a7856";
</script>

<button
  on:click={handleClick}
  style="border-color: {borderColor};"
  class:selected
>
  <span class="emoji" class:selected>
    {kindEmojis[kind]}
  </span>
  <span class="name">
    {kindNames[kind]}
  </span>
</button>

<style>
  button {
    width: 200px;
    height: 60px;
    border-radius: 12px;
    border: 4px solid;
    font-size: 24px;
    font-weight: bold;
    font-family: sans-serif;
    display: flex;
    align-items: center;
    padding-left: 16px;
    transition: all 0.2s ease;
    color: #5c4732;
    background-color: #f8f3ec;
  }

  button:hover {
    background-color: #fbf1e4;
    transform: translateY(-2px);
  }

  button.selected {
    background-color: #fce8d1;
  }

  .name {
    flex-grow: 1;
    text-align: center;
  }

  .emoji {
    filter: grayscale();
  }

  .emoji.selected {
    filter: grayscale(0%);
  }
</style>
