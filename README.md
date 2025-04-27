# Rust

To build the wgpu-lib wasm module, change to 03Emscripten\src\assets\wasm\wgpu-lib and type

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;wasm-pack build --target web

To build the ems-lib wasm module you will need the emscripten sdk and the wasm32-unknown-emscripten traget 

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;rustup target add wasm32-unknown-emscripten

Change to 03Emscripten\src\assets\wasm\ems-lib and type

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;cargo build --release --target wasm32-unknown-emscripten

after that switch back to 03Emscripten directory and type

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;npm install  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;npm run build-prod   (git deployment)  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;ng serve             (local development)  

Try it out:

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;https://pohldaniel.github.io/Rust/web-gpu/  

The example is from https://www.trion.de/news/2022/11/15/wgpu-webasm.html
