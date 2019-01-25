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