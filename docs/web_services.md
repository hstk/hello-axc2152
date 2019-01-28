# Web services

Cool, we can print "Hello World" to our console. One of the core requirements for a modern automation system is displaying some kind of system state to the user, either through outputs on a panel or a HMI. Using a web server, we can shuttle our data to the operator, to SCADA, central plant control, or what have you with a mix of static HTML, server rendered pages, JSON endpoints, and responsive web apps. So let's get a server going.

---

## 1. Open a port for our app

Normally we'd serve html over port 80, but the IEC-61336 system is using that for the web based HMI. To avoid interfering, we'll serve our Rust app over another port. It doesn't really matter what port you take, so long as its not currently occupied. 8080 has a long history of being an alternate port for html traffic, so we'll use that.

To do that, we'll have to modify our firewall to let traffic through that port. Our controller uses `nftables`. In `nftables` we have a list of tables, the tables have chains, and the chains have the active rules that control ports. In this particular configuration, we only have one table (`filter`), and two chains: one for input, one for output. To view all active rules, we'll use `nft list ruleset`.

We can see 8080 isn't open on our single table (named, aptly, `filter`), let's fix that. We'll retrieve a list of rules with `nft list table filter -na`. The flags give us a list of handles that allows sticking our new rule in a particular location; if you just add a rule, it'll append to the end and won't work properly.

```
admin@axcf2152:~$ sudo nft list ruleset -na
Password:
table ip filter {
        chain input {
                type filter hook input priority 0; policy accept;
                iif lo accept # handle 3
                icmp type echo-request counter packets 0 bytes 0 drop # handle 4
                icmp type echo-reply accept # handle 5
                ct state established accept # handle 6
                ct state related accept # handle 7
                tcp dport ssh accept # handle 8
                tcp dport http accept # handle 9
                tcp dport https accept # handle 10
                tcp dport 41100 accept # handle 11
                tcp dport 17725 accept # handle 12
                tcp dport 4840 accept # handle 13
                udp dport 34962-34964 accept # handle 14
                counter packets 2284 bytes 150014 drop # handle 15
        }

        chain output {
                type filter hook output priority 0; policy accept;
                ct state established accept # handle 16
                ct state related accept # handle 17
                ct state new accept # handle 18
                icmp type echo-request accept # handle 19
        }
}
```

We'll put our new rule right after https.

```
nft insert rule filter input position 45 tcp dport 8080 accept
```

Finally, the firewall script (`/etc/init.d/firewall`) will clobber our current settings if we reboot: it will flush (read: delete) all of the existing rules and restore them from a known good config off of the file `plcnext-filter`. If you want to save your current `plcnext-filter` file (never a bad decision), go ahead and make a copy.

```
mkdir /etc/nftables/backup
cp /etc/nftables/plcnext-filter /etc/nftables/backup
```

And finally we'll overwrite our `plcnext-filter` with our new version.

```
nft list table filter > /etc/nftables/plcnext-filter
```

If you run another `nft list ruleset`, you should see `tcp dport http-alt accept` appear in the input chain. That's our 8080 port. If you decide you want to wall off the port again, you can get the ruleset again and run `nft delete rule filter input handle _`, where _ is the position of the http-alt rule. To check everything, reboot the system.

```
admin@axcf2152:~$ sudo reboot

Broadcast message from root@axcf2152 (pts/0) (Mon May  7 10:17:14 2018):

The system is going down for reboot NOW!
admin@axcf2152:~$ exit
logout
Connection to 192.168.10.2 closed.
```

SSH back in, check your rules, ensure http-alt is still in your ruleset.

## 2. Scripting build and deployment

We're going to write up a quick makefile; essentially, it's a simple scripting tool that allows us to avoid typing out long, painful commands. `make` should come preinstalled with your WSL Ubuntu distro. Currently, my makefile looks something like this. To use it, just create a file called `makefile` in your project root and copy the following.

``` MAKE
plcAddr = 192.168.10.2

build:
	cargo build --target=armv7-unknown-linux-gnueabihf
	scp ./target/armv7-unknown-linux-gnueabihf/debug/hello-axc2152 admin@$(plcAddr):/opt/plcnext/
	ssh admin@$(plcAddr)

build-release:
	cargo build --target=armv7-unknown-linux-gnueabihf --release
	scp ./target/armv7-unknown-linux-gnueabihf/release/hello-axc2152 admin@$(plcAddr):/opt/plcnext/

ssh:
        ssh admin@$(plcAddr)
```

To build, copy your program to the controller, and SSH in, just run `make build` from the project root. Automate everything!

## 3. Update our Rust app

Let's make our app do something slightly more interesting. I've chosen Hyper as a web framework for this demo, but you could choose whatever flavor you desire. Throw the dependency in your `cargo.toml`.

```
[package]
name = "hello-axc2152"
...

[dependencies]
warp = "0.1.11"
```

Now, edit your `src/main.rs` to look like this; replace the vector in the argument (`[192, 168, 10, 2]`) to run with the address you've assigned your plcnext

```
extern crate warp;

use warp::Filter;

fn main() {
    println!("Serving Warp app...");
    let base = warp::any().map(|| "Hello from the AXC-2152!");
    let addr = [192, 168, 10, 2];

    warp::serve(base).run((addr, 8080));
}
}
```

Build and deploy to your AXC2152. Run `./hello-axc2152`, then open a browser and navigate to `http://192.168.10.2:8080/`. You should see 