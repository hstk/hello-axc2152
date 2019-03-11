# Calling C++ from Rust

Calling foreign functions is hard. A programming language neatly encapsulates a lot of different notions about program structure that suddenly become painful as soon as you step outside the framework. Luckily, `bindgen` abstracts a way a lot of this pain.

1. Let's get the needed dependency: `clang`. This assumes you're running Ubuntu 18.04 (you can double check by running `lsb_release -a`); if not, check out the [packages](http://apt.llvm.org/) on the LLVM site and choose the appropriate index.

``` console
wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
sudo apt-add-repository "deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-8 main"
sudo apt-get update
sudo apt-get install clang-8
```

2. Add `bindgen` to your project dependencies in `Cargo.toml`.

``` toml
[dependencies]
warp = "0.1.11"
bindgen = "0.48.1"
```

3. Call `cargo build` to make sure everything works as intended.

