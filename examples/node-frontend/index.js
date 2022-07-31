const input = document.querySelector("#input-0");
const output = document.querySelector("#output-0");

import("./node_modules/not-br-wasm/not_br_wasm.js").then((js) => {
    input.addEventListener("input", evt => {
        console.log(evt.target.value)
        // input: &str, frequency: u64, bold_percentage: f64, output_type: &str
        let outVal = js.process(evt.target.value, 1, 0.5, "HTML");
        console.log(outVal)
        output.innerHTML = outVal;
    })
    // js.greet("WebAssembly with NPM");
});
