# Conversion In Rust

Primitives types can be converted to each other through casting.


Rust address conversion between custom type(i.e struct and enum) by the use of traits. 
The Generic conversion will use the `From` and`Into` Traits. However there are more specific ones for the more common cases in particular when converting to and from String `S`.


## From and Into

The From and Into trais are inherntly linked and this is actually part of its implementation. If you are able to conver Type A from Type B, then it should be easy to believe that we should be able to convert type B to type A.


### From

The `From` trait allows  for a type to define how to create itself from another type hence providing a very simple mechanism for converting between several types. There are numerous implementations of this trait within the standard library for conversion of primitive and common types.


### Into 

The `Into` trait is simply the reciprocal of the `From` trait.It defines how to convert a type into another type. Calling `into()` typically required us to specify the result type as the compile is unable to determine this most of the time.



### From and Into are interchangable

`From` and `Into` are designed to be complementary. We do not need to provide an implementation for both traits. If you have implemented the `from` trait for your type,`Into` will call it when necessary.

Note:Converse is not true:Implementing `Into` from your type will automatically provide it with an implementation of `From`.
