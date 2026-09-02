# 3. Custom Types

Rust Custom Data Types are formed mainly throught the two keywords:
1. `struct`: Define a structure
2. `enum`: define an enumeration

Constants can also be created via the `const` and `static` keywords.


## 1. Structures

There are three types of structures(structs) that can be created using the `struct` keyword:

-  Tuple structs which are basically named tuples
-  The classic C structs
-  Unit structs which are filed-less are useful for generics.
  

## 2. Enums
The enum keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as a struct is also valid in an enum.