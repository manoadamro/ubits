# ubits

![crates.io](https://img.shields.io/crates/v/ubits.svg)
![crates.io](https://img.shields.io/crates/d/ubits.svg)
![build](https://github.com/manoadamro/ubits/actions/workflows/rust.yml/badge.svg)

Bit fields and masks for rust!

Supports widths of `8`, `16`, `32`, `64` and `128` bits.

## Install

Available from [crates.io](https://crates.io/crates/ubits)

Add `ubits` to your `Cargo.toml`:
```
[dependencies]
ubits = "0.1.0"
```

## Documentation:

[Documentation is automatically built from main and deployed here.](https://manoadamro.github.io/ubits/doc/ubits/)

## Usage

Generate a bitfield struct with a flag enum...<br>
(The following examples all use this as a definition.)
```rust
use ubits::bitfield;

    bitfield! {
        (pub) ExampleField
        ExampleFlags u8 {
            0 : Flag0
            1 : Flag1
            2 : Flag2
            3 : Flag3
            4 : Flag4
            5 : Flag5
            6 : Flag6
            7 : Flag7
        }
    }
```

### Instances

From integer:
```rust
let from_integer = ExampleField(123);
assert_eq!(ExampleField(123), from_integer);
```

From a binary string:
```rust
let from_binary = ExampleField::from_binary_str("01111011");
assert_eq!(ExampleField(123), from_binary)
```

From ones:
```rust
let from_ones = ExampleField::ones();
assert_eq!("11111111", from_ones.as_binary());
assert_eq!(255, from_ones.as_integer());
```

From zeros:
```rust
let from_zeros = ExampleField::zeros();
assert_eq!("00000000", from_zeros.as_binary());
assert_eq!(0, from_zeros.as_integer());
```

### Field Access

Get bit value by field:
```rust
let field = ExampleField::from_binary_str("01010101");
assert!(field.get(ExampleFlags::Flag0));
assert!(!field.get(ExampleFlags::Flag1));
```

Get bit value by index:

```rust
let field = ExampleField::from_binary_str("01010101");
assert!(field.get_index(0));
assert!(!field.get_index(1));
```

Set bit value by field:

```rust
let mut field = ExampleField::from_binary_str("01010101");
field.set(ExampleFlags::Flag1);
field.set(ExampleFlags::Flag3);
assert_eq!("01011111", field.as_binary());
```

Set bit value by index:

```rust
let mut field = ExampleField::from_binary_str("01010101");
field.set_index(1);
field.set_index(3);
assert_eq!("01011111", field.as_binary());
```

Clear bit value by field:

```rust
let mut field = ExampleField::from_binary_str("01010101");
field.clear(ExampleFlags::Flag0);
field.clear(ExampleFlags::Flag2);
assert_eq!("01010000", field.as_binary());
```

Clear bit value by index:

```rust
let mut field = ExampleField::from_binary_str("01010101");
field.clear_index(0);
field.clear_index(2);
assert_eq!("01010000", field.as_binary());
```

Toggle bit value by field:

```rust
let mut field = ExampleField::from_binary_str("01010101");
field.toggle(ExampleFlags::Flag0);
assert_eq!("01010100", field.as_binary());
field.toggle(ExampleFlags::Flag0);
assert_eq!("01010101", field.as_binary());
```

Toggle bit value by index:

```rust
let mut field = ExampleField::from_binary_str("01010101");
field.toggle_index(0);
assert_eq!("01010100", field.as_binary());
field.toggle_index(0);
assert_eq!("01010101", field.as_binary());
```

### Combinations

Combine bit fields: <br>
(use `into_combined` to consume self)

```rust
let mut a = ExampleField::from_binary_str("01010101");
let b = ExampleField::from_binary_str("10101010");
assert_eq!("11111111", a.combine(b).as_binary());
```

Get the intersection of two bitfields: <br>
(use `into_intersection` to consume self)
```rust
let mut a = ExampleField::from_binary_str("11000011");
let b = ExampleField::from_binary_str("01111110");
assert_eq!("01000010", a.intersect(b).as_binary());
```

Get the diff of two bitfields: <br>
(use `into_diff` to consume self)

```rust
let mut a = ExampleField::from_binary_str("11000011");
let b = ExampleField::from_binary_str("01100110");
assert_eq!("10100101", a.diff(b).as_binary());
```

### Bitwise

Both bit field instances and flags use bitwise operators to change bit values.

```rust
let mut from_zeros = ExampleField::zeros();
assert_eq!("00000000", from_zeros.as_binary());

// set bit to 1
from_zeros |= ExampleFlags::Flag1;
assert_eq!("00000010", from_zeros.as_binary());

// set bit back to 0
from_zeros &= ExampleFlags::Flag1;
assert_eq!("00000000", from_zeros.as_binary());

// toggle a bit
from_zeros ^= ExampleFlags::Flag1;
assert_eq!("00000010", from_zeros.as_binary());

from_zeros ^= ExampleFlags::Flag1;
assert_eq!("00000000", from_zeros.as_binary());
```

Operations can also be chained together:

```rust
let mut from_zeros = ExampleField::zeros() | ExampleFlags::Flag1 | ExampleFlags::Flag3;
assert_eq!("00001010", from_zeros.as_binary());

```

Bitfield instances can also be created from combining flags:

```rust
let mut from_zeros = ExampleFlags::Flag1 | ExampleFlags::Flag3;
assert_eq!("00001010", from_zeros.as_binary());

```

## Fields named with flags

The generated flags enum allows you to access bits by name.
The flag has an associated [`u8`] value,
which determines the index its target bit.
(See [`crate::bitfield`] for more info)

With the following input...
```no_compile
1 0 1 0 0 1 1 0
```

and the following flags...
```no_compile
0 : f1
1 : f1
2 : f2
3 : f3
4 : f4
5 : f5
6 : f6
7 : f7
```

we end up with this layout.

| name      | f7  | f6  | f5  | f4  | f3  | f2  | f1  | f0  |
|-----------|-----|-----|-----|-----|-----|-----|-----|-----|
| bit value | 1   | 0   | 1   | 0   | 0   | 1   | 1   | 0   |
| index     | 7   | 6   | 5   | 4   | 3   | 2   | 1   | 0   |


With the same input and only the first few flags:

```no_compile
0 : f0
1 : f1
2 : f2
```

we end up with this layout.

| name      |    |    |    |    |    | f2 | f1 | f0 |
|-----------|----|----|----|----|----|----|----|----|
| bit value | 1  | 0  | 1  | 0  | 0  | 1  | 1  | 0  |
| index     | 7  | 6  | 5  | 4  | 3  | 2  | 1  | 0  |


Using the same input, but with dispersed flags:

```no_compile
1 : f0
3 : f1
6 : f2
```

we end up with this layout.

| name      |    | f2 |    |     | f1 |    | f0  |    |
|-----------|----|----|----|-----|----|----|-----|----|
| bit value | 1  | 0  | 1  | 0   | 0  | 1  | 1   | 0  |
| index     | 7  | 6  | 5  | 4   | 3  | 2  | 1   | 0  |
