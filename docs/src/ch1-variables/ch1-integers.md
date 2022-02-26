# Integers

In Lem, all numbers are treated as floating-point.
For example, the number 2:
```js
let two = 2
```
The `println` built-in can print strings, integers, arrays, and booleans. It can also concatenate mismatching types, but it will not pretty-print arrays.
```js
println("Look, a number: " + 6)
```
As demonstrated on the [strings page](./ch1-strings.md), concatenating mismatching types outside of the `println` built-in will cause an error.
