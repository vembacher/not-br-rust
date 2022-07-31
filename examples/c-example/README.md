# not-br C Example

This builds and runs the C example as seen in [simple-example.c](./simple-example.c).

```shell
cargo build --release
cd examples/c-example
mkdir build
cd build
cmake ..
make
./simple-example
```