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
