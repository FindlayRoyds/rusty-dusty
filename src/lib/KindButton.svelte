<script lang="ts">
  import { Kind } from "../../public/wasm/wasm_crate.js";
  export let kind: Kind;
  export let selectedKind: Kind;
  export let selectKind: (kind: Kind) => void;

  function handleClick() {
    selectKind(kind);
  }

  const kindProperties = {
    [Kind.Air]: { name: "Clear", color: "#e55656", emoji: "🚫" },
    [Kind.Sand]: { name: "Sand", color: "#e6c88e", emoji: "🏖️" },
    [Kind.Wall]: { name: "Wall", color: "#8b5a2b", emoji: "🧱" },
    [Kind.Water]: { name: "Water", color: "#4287f5", emoji: "💧" },
    [Kind.Fire]: { name: "Fire", color: "#FF6600", emoji: "🔥" },
    [Kind.Wood]: { name: "Wood", color: "#502b0f", emoji: "🪵" },
  };

  $: selected = kind === selectedKind;
  $: kindInfo = kindProperties[kind] || {
    name: Kind[kind],
    color: "#9a7856",
    emoji: "❓",
  };
  $: borderColor = selected ? kindInfo.color : "#9a7856";
</script>

<button
  on:click={handleClick}
  style="border-color: {borderColor};"
  class:selected
>
  <span class="emoji" class:selected>
    {kindInfo.emoji}
  </span>
  <span class="name">
    {kindInfo.name}
  </span>
</button>

<style>
  button {
    width: 100%;
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
