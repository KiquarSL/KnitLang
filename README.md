# KnitLang
> In developent

## Example
```rust
pkg main;

use std::io;

fn main() -> i32 {
    i32 a = 67;
    i32 b = 33;
    i32 c = a + b;
    
    imut bool is = c == 100;
    
    if is {
        c = 0;
    }
    io::print("Hello, world");
    ret 0;
}
```
