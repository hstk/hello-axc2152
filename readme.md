# Rust on the AXC2152

[Compilation directions adapted from Ioannis Valasakis](https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b).

Luckily, the AXC2152 is an ARMv7, which we can target it with languages and tools that use LLVM. Including Rust!

Rust is a homerun choice for embedded applications. It has a unique linear type system that makes it well suited for resource constrained environments, eliminating the need for a garbage collector. The borrow checker allows the compiler to eliminate large cases of bugs involved in mutating variables. The tooling is fantastic, the community is large and beginner friendly.

## Hello Rust, from Windows 10 to your PLC-Next

Compilation directions slightly adapted from Ioannis Valasakis' [quick and concise guide](https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b) for people running Windows 10.

Instead of using Virtualbox, Vagrant, Docker, mingw64, we'll use the Windows Subsystem for Linux to get the required tools and cross compile. There are many ways to skin this cat, but this is one of the easiest.

1. Open up a Powershell terminal with admin rights and run `Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
`.

2. Install [Ubuntu](https://www.microsoft.com/en-us/p/ubuntu/9nblggh4msv6) from the Windows Store.

3. Launch the Ubuntu app or run `bash` from Powershell. Update your packages.

```bash
sudo apt-get update
sudo apt-get upgrade
```

4. Get `rustup` and the `gcc` utils for the target architecture.

```bash
sudo apt-get install gcc-arm-linux-gnueabihf
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup target add armv7-unknown-linux-gnueabihf
```

5. Create a default global config for the linker when cross compiling for ARMv7. Alternately, you can drop the same `config` file in a `.cargo/` directory sitting at the root of your project.

```bash
echo -e "[target.armv7-unknown-linux-gnueabihf]\nlinker = \"arm-linux-gnueabihf-gcc\"" >> ~/.cargo/config
```

6. Use cargo to spin up a new project and run it. This will 
```
cargo new hello-axc2152
cargo run
```

7. Copy the binary to your controller.
```
scp ./target/armv7-unknown-linux-gnueabihf/debug/hello-axc2152 admin@192.168.x.x:/opt/plcnext/
```

8. SSH in and run your program!
```
user@λ:~$ ssh admin@192.168.10.2
admin@192.168.10.2's password:
Last login: Fri May  4 05:43:06 2018 from 192.168.10.10
admin@axcf2152:~$ ls
certificates  hello-axc2152  logs  lttng  profinet  projects
admin@axcf2152:~$ ./hello-axc2152
Hello, world!
```

## Web services

Cool, we can print "Hello World" to our console. It's not that useful, so let's try something close to actually running a machine. We'll actually simulate a real control routine. 

We'll
-configure a project in PCWE
-pull some data from PCWE using a TCP connection
-deserialize into some useful Rust types
-do some calculations with it and return a struct that sets some outputs
-send that struct back to PCWE
-deserialize it again in our IEC-61336 program and write an output
-serve up an HMI with those values over a web server, using HTML and JSON

## Next steps

For ease of building and deployment, use a makefile to write down the reptitive build tasks. Create a makefile in your project root and use `make build` to ease the deployment.

``` make
plcAddr = 192.168.10.2

build:
  cargo build --target=armv7-unknown-linux-gnueabihf
  scp ./target/armv7-unknown-linux-gnueabihf/debug/hello-axc2152 admin@$(plcAddr):/opt/plcnext/
  ssh admin@$(plcAddr)

release:
  cargo build --target=armv7-unknown-linux-gnueabihf --release
  scp ./target/armv7-unknown-linux-gnueabihf/release/hello-axc2152 admin@$(plcAddr):/opt/plcnext/

ssh:
  ssh admin@$(plcAddr)
```

---

Use git to manage your version control! Of the software engineering practices that automation engineers don't seem to widely use, version control is probably the single biggest thing. Your Rust program is a collection of flat files: using a few simple commands, you can keep a log of all changes made, instantly backup your work to multiple locations, make changes effortlessly, get multiple sets of eyes on proposed designs. Git should come by default on the WSL; 

---

Write unit tests. Debugging in the field is hard, why not just write some unit or integration tests 
