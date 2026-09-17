# 4. Variable Bindings

Rust provides type safety via static typing. Variable bindings can be type annotated when declared. However, in most cases, the compiler will be able to infer the type of the variable from the context, heavily reducing the annotation burden.

Values (like literals) can be bound to variables, using the `let `binding.


## a. Mutability

Variable bindings are immutable by defauly but this can be overridder using the `mut` modifier;


### b. Scope and Shadowing

Variable bindings have a scope and are constrained to live in a block. A Block is a collection of statements enclosed by braces {}.


