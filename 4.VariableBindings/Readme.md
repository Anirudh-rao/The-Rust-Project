# 4. Variable Bindings

Rust provides type safety via static typing. Variable bindings can be type annotated when declared. However, in most cases, the compiler will be able to infer the type of the variable from the context, heavily reducing the annotation burden.

Values (like literals) can be bound to variables, using the `let `binding.


## a. Mutability

Variable bindings are immutable by defauly but this can be overridder using the `mut` modifier;


## b. Scope and Shadowing

Variable bindings have a scope and are constrained to live in a block. A Block is a collection of statements enclosed by braces {}.


## c. Declare First

It is possible to declare variable bindings first and initialize them later, but all variable bindings must be initialized before they are used: the compiler forbids use of uninitialized variable bindings, as it would lead to undefined behavior.

It is not common to declare a variable binding and initialize it later in the function. It is more difficult for a reader to find the initialization when initialization is separated from declaration. It is common to declare and initialize a variable binding near where the variable will be used.

## d. Freezing

When data is bound by the same name immutably, it also freezes. Frozen data can’t be modified until the immutable binding goes out of scope.
