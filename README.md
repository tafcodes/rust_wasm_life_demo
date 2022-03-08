# rust_wasm_life_demo
Quick project to learn how to target WebAssembly from Rust

## Setup
There's a few ways to do this... 

### First iteration:
The first time, I'll just follow the happy path presented in a tutorial I found.

#### Install `wasm-pack`.  
This is packaged in the AUR, so I just install it as a 
system package.

#### Install `cargo-generate`
This looks pretty simple and pointless...it just clones a git repo into a new
cargo project directory to use it as a template?  Whatever, I'll try it.

#### NPM
I already have a system version of NPM, and currently no real node env management
because I haven't worked with it in a while...so hopefully this stuff will all
play nice with the node and npm versions I happen to have already installed.

#### Template:
`cargo generate --git https://github.com/rustwasm/wasm-pack-template`

### Second iteration:
In an attempt to distance myself from the JS ecosystem as much as possible,
I may like to use `--target web` to directly 