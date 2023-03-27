# 1. MAJP's notes on Rust
These are some notes I took while following the book [The Rust Programming Language](https://doc.rust-lang.org/book/).

- [1. MAJP's notes on Rust](#1-majps-notes-on-rust)
- [2. Mutability](#2-mutability)
- [3. Constants](#3-constants)
- [4. Shadowing](#4-shadowing)
  - [4.1. A note on typing when shadowing](#41-a-note-on-typing-when-shadowing)
  - [4.2. A note on scope](#42-a-note-on-scope)
- [5. Data types](#5-data-types)
  - [5.1. Scalar types](#51-scalar-types)
    - [5.1.1. Integers](#511-integers)
      - [5.1.1.1. Beware: Integer overflow](#5111-beware-integer-overflow)
    - [5.1.2. Destructuring a tuple](#512-destructuring-a-tuple)
  - [5.2. Arrays](#52-arrays)
  - [5.3. Vectors](#53-vectors)
- [Expressions and statements](#expressions-and-statements)
- [Inline logic](#inline-logic)
- [Loop with return value](#loop-with-return-value)
- [Labelling loops](#labelling-loops)



# 2. Mutability
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

# 3. Constants
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

# 4. Shadowing
In the following expression, the first variable is shadowed by the second one:
```rust
let x = 1
let x = 2
```

To reiterate, shadowing and mutability are not the same. For a immutable variable, we can only shadow using `let`.

## 4.1. A note on typing when shadowing
We can have pseudo-dynamic typing by using then `let` keyword like this:

```rust
let characters = "abc";  // Is a String
let characters = characters.len();  // Is a u32
```

But this wouldn't compile:

```rust
let mut characters = "abc"
characters = characters.len()  // will fail due to change in type
```


## 4.2. A note on scope
Shadowing only happens in the current scope.

Here is an example:

```rust
fn main() {
    let x = 5;
    println!("{x}");
    {
        let x = x * 2;
        println!("{x}");
    }
    println!("{x}");
}
```

Output:

```shell
5
10
5
```

# 5. Data types
Rust is statically typed, so it _must_ know the types of all variables. Often the compiler can infer the type. Sometimes it can't, so we have to use annotations.

## 5.1. Scalar types
There are four primary scalar types in Rust:

- Integers
- Floating-point numbers
- Booleans
- Characters

### 5.1.1. Integers
As known from other languages (`u8`, `i8`, `u128` and so on). Rust also has the `arch` type, annotated with `isize` and `usize`, which refers to the OS architecture (32 or 64 bit).

You can write an integer like

```rust
let my_int = 100;
```

Which defaults to `i32`, or you can write it like

```rust
let my_int = 100i8;
```

To force the `u8` type.

#### 5.1.1.1. Beware: Integer overflow
Integer overflow occrs when the size of an integer is outside its tyep's range. Rust has some deterministic but perhaps unexpected behaviour here when compiling in `--release` mode. See [here](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow).

###2.4.2. Compound types
Rust has types that consist of several other types. An example is the `tuple`:

```rust
let tup: (i32, f64, bool) = (500, 3.14, true);
```

### 5.1.2. Destructuring a tuple
We can destructure a tuple like this:

```rust
let tup: (i32, f64, bool) = (500, 3.14, true);
let (x, y, z) = tup;
```

We can also use dot indexing like this:

```rust
let x = tup.0;
let y = tup.1;
let z = tup.2;
```

## 5.2. Arrays
Arrays in Rust are collections of multiple values. Contrary to tuples, values in an array must have the same type. Here is an array:

```rust
let arr: [i8; 4] = [1, 2, 3, 4];  // Array of u8s with 4 elements
```

Arrays in Rust have a fixed length.

You can "fill" an array with the same value like this:

```rust
let zeros = [0; 5];  // Array with five zeros
```

You can index into an array like this:

```rust
let arr = [1, 2, 3];
let three = arr[2];
```

## 5.3. Vectors
Vectors are like arrays, but they can change in size. You can make a vector like this:

```rust
let mut vec = Vec::new();
```

or like this:

```rust
let mut vec = vec![1, 2, 3];
```

# Expressions and statements
Expressions and statements are different things. An expression has a return value and a statement does not.

```rust
let x = 1;  // Statement
let y = {
    let z = 5
    z + 2
};  // Expression (y = 7)
```

Note that the `z + 2` line does not end with `;`. Expressions don't end with semicolons. This makes them return a value.

# Inline logic
We can use inline logic for assignments like this:

```rust
let boolean = true;
let x = if boolean { 5 } else { 6 };  // Will evaluate to 5 because boolean is true.
```

# Loop with return value
We can make a loop expression that returns a value like this

```rust
let mut i = 0;
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
}
```

Here, `result` will have the value 20.

# Labelling loops
We can label a loop to give it a name like this:

```rust
let mut i = 0;
let mut j = 0;

'outer_loop: loop {
    i += 1;
    'inner_loop: loop {
        j += 2;
        if j == 52 {
            break;
        }
        if i == 10 {
            break 'outer_loop;
        }
    }
}
```

This will break the outer loop if `i == 10` and break the inner loop if `j == 52`.

