## Hello Rust, from Windows 10 to your PLC-Next

Compilation directions slightly adapted from Ioannis Valasakis' [quick and concise guide](https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b) for people running Windows 10.

Instead of using Virtualbox, Vagrant, Docker, mingw64, we'll use the Windows Subsystem for Linux to get the required tools and cross compile. There are many ways to skin this cat, but this is one of the easiest.

## Directions

1. Open up a Powershell terminal with admin rights and enable WSL. 

```
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

2. Install [Ubuntu](https://www.microsoft.com/en-us/p/ubuntu/9nblggh4msv6) from the Windows Store.

3. Launch the Ubuntu app or run `bash` from Powershell. Update your packages, it may take a while.

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

Cool, we can print "Hello World" to our console. But it's not terribly useful. So let's spin up a [web server](./web_services.md).