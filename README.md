# Create React App + WebAssembly (Rust)

This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app) and [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template).

## Available Scripts

In the project directory, you can run:

### `yarn start`

Start dev server then watch `.rs` and `.ts` and rebuild if files are changed.

### `yarn test`

Run `wasm-pack test --firefox --headless` and `react-scripts test`.

### `yarn build`

Compile `.rs` to `.wasm` using `wasm-pack` then build React app using `create-react-app`.

## Learn More

- [Create React App documentation](https://facebook.github.io/create-react-app/docs/getting-started)
- [React documentation](https://reactjs.org/)
- [wasm-bindgen documentation](https://rustwasm.github.io/docs/wasm-bindgen/)
- [wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/)
- [wasm-pack-plugin documentation](https://github.com/wasm-tool/wasm-pack-plugin)

## Battery Included

- `console_error_panic_hook`: https://github.com/rustwasm/console_error_panic_hook
  > This crate lets you debug panics on `wasm32-unknown-unknown` by providing a panic hook that forwards panic messages to `console.error`.
- `wee_alloc`: https://github.com/rustwasm/wee_alloc
  > This crate is focused on targeting `WebAssembly`, producing a small `.wasm` code size, and having a simple, correct implementation.
