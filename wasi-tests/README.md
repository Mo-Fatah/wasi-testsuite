## WASI tests

The testsuite is still in development and in its early stages. The testsuite is built upon [wasmtime's wasi-tests](https://github.com/bytecodealliance/wasmtime/tree/03077e0de9bc5bb92623d58a1e5d78b828fd1634/crates/test-programs/wasi-tests) as a starting point. The code strutcure, testing approach and some helper functions are based on wasmtime's tests.


## Running the Tests
- The tests can be run automatically against a specific engine with:
```
cargo run init <engine-name>
```
Currently, you can only run it with `wasmtime` and `wasmer`


## Contribution
- To add a new test function for a specific API, you can go to `src/bin`, find the API, add your test function and then call it from the below `main` function

- To add a new API, create the binary inside `src/bin`, and add your test cases. Then go to `src/bin/init.rs`, create a function for this API to prepare the tests conditions, then call this function from the [main](https://github.com/Mo-Fatah/wasi-testsuite/blob/943716752d32dc724d0c44e0140a2b0cfdcf00ca/wasi-tests/src/bin/init.rs#L4) function 
