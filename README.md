# Minimal Multithreaded WASM Example

This example shows how to use rayon and wasm_bindgen together which in the current implementation will exploit JS worker threads to be able to compute concurrently.

For some reason I can only get this to work in Chrome, wheras on FireFox it displays any combination of hanging or displaying this error: `SyntaxError: export declarations may only appear at top level of a module` for every thread on your computer.

...Despite the fact that the two examples I used to get this to work both work in FF and Chrome!
 - https://rreverser.com/wasm-bindgen-rayon-demo/
 - https://wasm-bindgen.netlify.app/exbuild/raytrace-parallel/

I'll need to investigate further!

To run, you need wasm-pack on your system: https://rustwasm.github.io/wasm-pack/installer/

Then build with: `wasm-pack build --target web`
serve with: `python3 serve.py`
then go to: http://127.0.0.1:8000/index.html in your browser.