<script lang="ts">
    import Canvas from './lib/Canvas.svelte'
    import type {Config} from './types'

    let config: Config = { goalFPS: 60, brushRadius: 7 };
</script>

<main>
  <Canvas {config} />
</main>

