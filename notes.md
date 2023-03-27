# MAJP's notes on Rust
These are some notes I took while following the book [The Rust Programming Language](https://doc.rust-lang.org/book/).

### Mutability
If a variable is immutable (default), its value cannot change. You cannot shadow the variable either. This would produce a compilation error:

```rust
fn main {
    let x = 5;
    x = 1;
}
```

But this would not:

```rust
fn main {
    let mut x = 5;
    x = 1;
}
```

### Constants
Constants are like immutable variables, but there are some differences:

1. You cannot use `mut` with a constant.
2. The value _must_ be annotated.
3. Constants can be declared in any scope, including the global scope.
4. Constants can only be declared as constant expressions. They cannot come from an expression whose result is only available at runtime.

This is a constant:

```rust
const NUMBER_OF_SECONDS_IN_A_MINUTE: u8 = 60;
```

Rust is able to evaluate a number of expressions at compile time, so the following would also work:

```rust
const APPROXIMATELY_PI: f32 = 22 / 7;
```

### Shadowing
In the following expression, the first variable is shadowed by the second one:
```rust
let x = 1
let x = 2
```

To reiterate, shadowing and mutability are not the same. For a immutable variable, we can only shadow using `let`.

#### A note on typing when shadowing
We can have pseudo-dynamic typing by using then `let` keyword like this:

```rust
let characters = "abc"  // Is a String
let characters = characters.len()  // Is a u32
```

But this wouldn't compile:

```rust
let mut characters = "abc"
characters = characters.len()  // will fail due to change in type
```


#### A note on scope
Shadowing only happens in the current scope.

Here is an example:

```rust
fn main() {
    let x = 5;
    println!("{x}")
    {
        let x = x * 2
        println!("{x}")
    }
    println!("{x}")
}
```

Output:

```shell
5
10
5
```