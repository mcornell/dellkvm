This is a simple tool that allows switching between the USB-C input and the HDMI 1 input on the Dell W3421WE monitor on ubuntu

It may work with other Dell monitors and other linux distros, but they have not been tested.  At this point, only deb packages have been created. The deb package will install the ddcutil package required to query the monitor.

To build, you're going to want the cargo-deb plugin to build a debian package

```sh
cargo install cargo-deb
cargo deb
```

to install:

```sh
sudo apt install target/debian/dellkvm_0.1_0_amd64.deb
```

to run:

```sh
sudo dellkvm
```

should yield output similar to this:
```
["VCP", "60", "SNC", "x11"]
"ddcutil" CommandArgs { inner: ["setvcp", "60", "0x1b"] }
stdout: 
stderr: 
```

Yes, this all could have been done withe a shell script, but YOLO Rust.