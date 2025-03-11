import init, { greet, add, get_people, Person } from "../public/wasm/wasm_crate.js"

async function runWasm() {
    await init()
    console.log(greet("Vite Typescript with Wasm"))

    const addResult = add(12, 18)
    document.body.textContent = `result: ${addResult}`

    let people = get_people()

    console.log(people)
    people.forEach(person => {
        console.log(`Name: ${person.name}, Age: ${person.age}`)
    })

    let newPerson = new Person("David", 40);
    people.push(newPerson);

    console.log("After adding a new person")
    people.forEach(person => {
        console.log(`Name: ${person.name}, Age: ${person.age}`)
    })
}

runWasm()