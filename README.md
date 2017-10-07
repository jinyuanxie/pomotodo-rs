# Pomotodo SDK for Rust Programming Language.

An **unofficial** [Pomotodo][pomotodo] SDK for Rust Programming Language, base on [reqwest][reqwest].

## Usage

To use `pomotodo`, first add the following to your `Cargo.toml`:

```toml
[dependencies]
pomotodo = "0.2"
```

Then, add the following to your crate root:
```rust
extern crate pomotodo;
```

The following is an example to print your account information:

```rust
extern crate pomotodo;

use pomotodo::Client;

fn main() {
  let client = Client::new("YOUR_ACCESS_TOKEN");
  
  println!("{}", client.account().unwrap());
}
```

[pomotodo]: https://pomotodo.com
[reqwest]: https://github.com/seanmonstar/reqwest
