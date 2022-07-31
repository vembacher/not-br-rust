# not-br-wasm Example

```shell
# Install wasm-pack, which is needed to build the library.
cargo install wasm-pack

# Build the library.
cd not-br-wasm/
wasm-pack build --target web


# Build example.
cd ../../examples/node-frontend
npm install

# Run example.
npm run serve
```