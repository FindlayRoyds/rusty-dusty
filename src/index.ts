import init, {
  greet,
  add,
  get_people,
  Person,
  store_value_at_idx_zero,
  read_wasm_memory_buffer_at_idx_one,
  get_wasm_memory_buffer_pointer,
} from "../public/wasm/wasm_crate.js";

async function runWasm() {
  let rustWasm = await init();
  console.log(greet("Vite Typescript with Wasm"));

  const addResult = add(12, 18);
  document.body.textContent = `result: ${addResult}`;

  let people = get_people();

  console.log(people);
  people.forEach((person) => {
    console.log(`Name: ${person.name}, Age: ${person.age}`);
  });

  let newPerson = new Person("David", 40);
  people.push(newPerson);

  console.log("After adding a new person");
  people.forEach((person) => {
    console.log(`Name: ${person.name}, Age: ${person.age}`);
  });

  store_value_at_idx_zero(24);

  let wasmMemory = new Uint8Array(rustWasm.memory.buffer);
  let bufferPointer = get_wasm_memory_buffer_pointer();
  console.log(wasmMemory[bufferPointer + 0]);
  wasmMemory[bufferPointer + 1] = 10;
  console.log(read_wasm_memory_buffer_at_idx_one());
}

runWasm();
