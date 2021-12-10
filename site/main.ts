import init, { greet } from "../pkg/simulation_wasm.js";
init()
    .then(() => {
        // here we go!
        greet("morrowind")
    })