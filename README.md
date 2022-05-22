# termcol

Dependency-less library for simple terminal text coloring and formating

# Features

TODO

# Usage

Basic usage (checkout the examples/usage.rs file for more information)

```rust
use termcol::*;

fn main() {
    println!("{}red{}", color("red"), color("reset"));
    println!("{}bold{}", format("bold"), format("reset"));
    println!("{}", color_string("im blue", "blue"));
}
```

# LICENSE

This project is distributed under MIT license.
