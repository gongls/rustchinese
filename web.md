1. Get a recent version of Rust and Cargo
```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```
2. Let Cargo generate a new project directory and cd into it
```
$ cargo new nickel-demo --bin && cd nickel-demo
```
3. Edit the generated Cargo.toml file
```
[package]

name = "my-demo"
version = "0.0.1"
authors = ["Your Name <your.name@somewhere.com>"]


[dependencies]
nickel = "*"

```
4. Edit the generated main.rs file
```
#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:6767");
}

```
5. Run your app
```
http://localhost:6767
```
