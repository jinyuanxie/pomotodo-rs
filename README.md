# Unofficial [Pomotodo][1] api for Rust.

## Description
This is an unofficial [Pomotodo][1] api for Rust Programming Language, base on [hyper][2].

For more details, just boswer the code, it is very simply.

Welcome to open issue and create PR.

## Example

```rust
extern crate pomotodo;

// use pomotodo::pomo::Pomo;
use pomotodo::session::Session;
// use pomotodo::account::Account;
// use pomotodo::todo::{Todo, SubTodo};

fn main() {
  let sess = Session::with_token("your_api_token").unwrap();
  
  println!("{}", sess.get_account().unwrap());
  
  // ...
}
```

## License
Please read the `LICENSE`.

[1]: https://pomotodo.com
[2]: https://hyper.rs

