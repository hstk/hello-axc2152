# Hello Rust, from Windows 10 to your PLC-Next

My compilation directions are adapted from Ioannis Valasakis' [quick and concise guide](https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b) to target Windows 10, which seems to be the default in the automation world. Thanks!

Instead of using Virtualbox, Vagrant, Docker, mingw64, we'll use the Windows Subsystem for Linux to get the required tools and cross compile. There are many ways to skin this cat, but this is one of the easiest. As a bonus, it's really easy to use WSL to interface with the AXC-2152 for configuration purposes; it comes with an SSH client, no need to install Windows utilities or run a VM.

## Directions

1. Open up a Powershell terminal with admin rights and enable WSL.

``` powershell
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

2. Install [Ubuntu](https://www.microsoft.com/en-us/p/ubuntu/9nblggh4msv6) from the Windows Store. This should install the latest release of Ubuntu; at the time of this writing, 18.04 Bionic.

3. Launch the Ubuntu app or run `bash` from Powershell. We'll install the needed dependencies and then update packages. It may take a while. I had an issue with some dependencies in `libc6`; you might have to [uninstall and reinstall](https://askubuntu.com/questions/1079797/how-do-i-fix-an-error-with-libc6-dev-armhf-cross-in-ubuntu-18-04-when-trying-to/1080618#1080618) a few.

``` console
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install build-essential gcc-arm-linux-gnueabihf make
```

4. Get `rustup` and the `gcc` utils for the target architecture.

``` console
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup target add armv7-unknown-linux-gnueabihf
```

5. Create a default global config for the linker when cross compiling for ARMv7. Alternately, you can drop the same `config` file in a `.cargo/` directory sitting at the root of your project.

``` console
echo -e "[target.armv7-unknown-linux-gnueabihf]\nlinker = \"arm-linux-gnueabihf-gcc\"" >> ~/.cargo/config
```

6. Use cargo to spin up a new project and run it. You should get some output in your console. By default, this builds and runs the project for the architecture of the machine you're currently running.

``` console
cargo new hello-axc2152
cargo run
```

7. Let's build the binaries for the AXC-2152.

``` console
cargo build --target=armv7-unknown-linux-gnueabihf
```

8. Copy the appropriate binary to your controller.

``` console
scp ./target/armv7-unknown-linux-gnueabihf/debug/hello-axc2152 admin@192.168.x.x:/opt/plcnext/
```

9. SSH in and run your program!

``` console
user@λ:~$ ssh admin@192.168.10.2
admin@192.168.10.2's password:
Last login: Fri May  4 05:43:06 2018 from 192.168.10.10
admin@axcf2152:~$ ls
certificates  hello-axc2152  logs  lttng  profinet  projects
admin@axcf2152:~$ ./hello-axc2152
Hello, world!
```

We can print "Hello World" to our console and execute arbitrary Rust code, but console apps on a controller aren't super useful. So let's spin up a [web server](./web_services.md).
