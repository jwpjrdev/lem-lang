# Strings

The syntax for string variables is relatively unchanged, and looks as follows:
```js
let helloWorld = "Hello, world!";
```
This variable can be used anywhere within its scope, like in this block:
```rust,ignore
# let helloWorld = "Hello, world!";
println(helloWorld);
```
As might be expected, this would print `Hello, world!`.

Strings can be concatenated with the `+` operator. It cannot add two differing types outside of the `println` built-in, such as an integer and a string.

For example, this program outputs `Hello, world!`.
```js
let hello = "Hello, ";
let world = "world!";
let helloWorld = hello + world;

println(helloWorld);
```

This program errors because `string` and `num` are of mismatching types (string and integer):
```js
let str = "Look, a string!";
let num = 3;

println(str + num);
```
Integers can be observed more in-depth on [their page](./ch1-integers.md).

You can also split a string into an array of strings, like so:
```rust,ignore
let str = "Item 1, Item 2, Item 3";
let array = str.split(", ");
println(array);
```
```
["Item 1", "Item 2", "Item 3"]
```
