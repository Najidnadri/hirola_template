# Template for Hirola web app framework.

This template comes with a nicely done web app design pattern and file structure. If you familiar with Vuejs or something similar to it, I cannot find it hard for you to understand the design pattern. This is mainly because Hirola is a web app framework that suppose to be a beginner friendly. 


# What is Hirola?

Converting from a Javascript/Typescript framework to a Rust wasm framework is not as easy as it sounds. Im pretty sure many have experienced this (including me). However, with Hirola, developing a webapp with Rust is suprisingly not THAT hard because of its simplicity. Hence, I created this template to furthermore help the front-end developers on developing their next project with Rust and Hirola.


## Usage

Make sure you have the latest Rust
```
rustup update
```

Install cargo generate 
```
cargo install cargo-generate
```
and Trunk
```
cargo install --locked trunk
```


Clone the template with this command and follow the instructions given.
```
cargo generate Najidnadri/hirola_template
```



And that's it. Enter your project development folder and you can now start developing with hot reloading using this command
```
trunk serve
```

Once you are satisfied with your app. You can build in release mode the app by running
```
trunk build --release
```

## Note

Since Hirola is still under development, breaking changes may happen, same as this template. If you are looking for something more matured and stable. I recommend Yew, Percy, Seed, or Perseus. 

## Tips

One of your bestfriend when developing WASM in rust would be the [web_sys](https://crates.io/crates/web-sys) crate. `web_sys` makes it easy for you to get most of the browser instances.
Try interacting and exploring how to use the crate. 

After that you can try looking at [gloo](https://crates.io/crates/gloo), and its siblings crates. When developing with `web_sys` and `wasm_bindgen` you will notice there will be a lot of boilerplates. `gloo` takes away the boilerplate for you. However, I really recommend you to not jump straight into `gloo` and instead learn how `web_sys` and `wasm_bindgen` works. 

