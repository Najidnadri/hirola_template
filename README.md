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



And that's it. Enter your project developer and you can now start developing with hot reloading using this command
```
trunk serve
```



## Note

Since Hirola is still under development, breaking changes may happen, same as this template. If you are looking for something more matured and stable. I recommend Yew, Percy, Seed, Perseus or Sycamore. 

