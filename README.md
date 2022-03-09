# rust_wasm_life_demo
Quick project to learn how to target WebAssembly from Rust.

This uses the tools and templates provided by the `rustwasm` group, which are great but maybe abstracting away some stuff that I wanted to learn about.  I may make my own minimal template for doing rust/wasm/canvas apps without node dependencies.

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

#### Build:
`wasm-pack build`

I want to learn how this actually works, i.e. installing and using targets through
rustup &c, but we will follow the tutorial here and let wasm-pack figure it all out for us...

#### Stuff it into a webpage:
Tutorial uses an npm template which I'm not super excited about, but again we'll go along with it.  

`npm init wasm-app www`

This inits an npm project from `wasm-app` into a new 'www' directory.  In so doing, npm realizes that it doesn't have such a template, and that it needs to install `create-wasm-app`, an npm package.  I'm not sure how it resolves this, but at the moment I don't care.

##### Actually using this:
To use this for real, I would fill in fresh names and stuff into the various package.json's and cargo.toml.

One of these templates was pulled in as a full git repo, not as raw files...so I can either add it as a submodule or manually un-git-ify it to manage the files as part of my repo.  Ugly.

### Second iteration:
In an attempt to distance myself from the JS ecosystem as much as possible,
I may like to use `--target web` to directly 

May be good to look here:
`https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html`

I may not even want to use WASM-bindgen.  It sounds like the WASM machine has a simple chunk of memory that you can read into and out of in JS.  So, for example, if I wanted to do something simple like draw bitmap onto a canvas, I could literally allocate a framebuffer in Rust and send a pointer over to JS, and have a single JS thread that just reads the framebuffer and draws to canvas.  ^And all that is assuming that I can't use the canvas API directly from Rust.  I'm not even sure if that's a limitation or not.

