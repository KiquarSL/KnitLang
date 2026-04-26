# KnitLang
> Project for learning lang-dev

Code .knit trasnlating to .c

## Usage

```
knit <knit file>
```


## Example
```rust


use stdio

fn main() -> i32 {
    i32 a = 67;
    i32 b = 33;
    i32 c = a + b;
    
    imut bool is = c == 100;
    
    if is {
        c = 0;
    }
    printf("%s\n", "Hello, world");
    ret 0;
}
```
