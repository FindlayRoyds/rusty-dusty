# Rusty Dusty WASM App

Rusty Dusty is a Rust WASM web app that runs a simulation of different grid-bound particles.
You can try it out at [dust.findlayroyds.com](http://dust.findlayroyds.com).
## How to play
The blue square is the canvas, where the particles exist.
You can click and drag to place particles.
On the right is the toolbar, where you can use the slider to change the brush size and select the type of particle.
## Technologies
The frontend is written in Svelte + TypeScript, and makes function calls to WASM functions written in Rust.
## Running
1. Ensure you have cargo and npm installed.
1. Install wasm-pack with `cargo install wasm-pack`.
1. Install npm dependencies with `npm install` (in the project's root directory).
1. Build the WASM with `./build-wasm.sh`.
1. Run `npm run dev` to start a local server, and visit the specified localhost link
(probably [localhost:5173](http://localhost:5173)).
