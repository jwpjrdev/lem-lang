# Conditional Statements

## If Statements
```rs,ignore
let raining = true;
let temperature = 82;
let warmOut = temperature >= 70;

if raining && warmOut {
    println("It's terribly humid outside...");
} else if raining && !warmOut {
    println("It's cold and wet today...");
} else if !raining && warmOut {
    println("It's a good day to go outside!");
} else if !raining && !warmOut {
    println("It's chilly outside!");
}
```
See [comparison operators](comparison-operators.md) for more information.
