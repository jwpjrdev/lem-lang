# Arrays

Arrays can be defined with the following syntax:
```js
let array = ["Item 1", "Item 2", "Item 3"];
```

They can also be iterated over (see [loops & recursion](../ch2-control-flow/loops-and-recursion.md) for more).
For example, this program utilites a `for` loop.
```rs,ignore
# let array = ["Item 1", "Item 2", "Item 3"];
for item in array {
    println(item);
}
```
It outputs the following:
```
Item 1
Item 2
Item 3
```

You can also get a specific item from a list, like so:
```rs,ignore
# let array = ["Item 1", "Item 2", "Item 3"];
println(array[0]);
```
This program will print `Item 1`.
