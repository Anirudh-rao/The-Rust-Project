# Primitives 


## Primitives 
Rust provides access to wide variety of ``primitives``. A Sample Includes

#### Scalar Types

1. Sigined Integers: `i8`, `i16`,`i32`,`i128` and `isize`(Pointer Size)
2. Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
3. Floating Point: `f32`,`f64`
4. char: Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
4. bool : Either `true` or `false`.
4. The Unit Type `()` , whose only possinlt value is an empty tuple:`()`

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

### Compound Types
1. Arrays like ``[1, 2, 3]``
2. Tuples like ``(1, true)``

Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64. Note that Rust can also infer types from context.


## Literals and Operators

Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true` and the unit type `()` can be expressed using literals.

Intgers can be alternatively be expressed using hexadecimal , octal or binary notation using these prefixed respectively: `0x`, `0o` or `0b`.

Underscores can be inserted in numeric literals to improve readability, e.g. `1_000` is the same as `1000`, and `0.000_001` is the same as `0.000001`.

Rust also supports scientific `E-notation`, e.g. `1e6`, `7.6e-4`. The associated type is `f64`.

We need to tell the compiler the type of the literals we use. For now, we'll use the `u32` suffix to indicate that the literal is an unsigned `32-bit integer`, and the `i32` suffix to indicate that it's a signed `32-bit integer`.

