# Rust

To build the wasm module, change to 02WebGPU\src\assets\wasm\wgpu-lib and type

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;wasm-pack build --target web

after that switch back to 02WebGPU directory and type

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;npm install  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;npm run build-prod   (git deployment)  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;ng serve             (local development)  

Try it out:

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;https://pohldaniel.github.io/Rust/web-gpu/
