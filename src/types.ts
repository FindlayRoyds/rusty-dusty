import init, { Kind } from "../public/wasm/wasm_crate.js";

export type Config = {
  goalFPS: number; // essentially goal fps
  brushRadius: number;
  selectedKind: Kind;
  useHashbrown: boolean;
};
