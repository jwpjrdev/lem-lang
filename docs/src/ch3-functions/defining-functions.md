# Defining Functions

## Basic Functions

The syntax to define a basic function with no parameters is as follows:
```rust,ignore
fn helloWorld() {
    println("Hello, world!");
}
```
Calling this function is quite straightforward:
```rust,ignore
# fn helloWorld() {
#     println("Hello, world!");
# }
# 
helloWorld();
```

## Parameterised Functions

To create a function with parameters, use the following syntax:
```rust,ignore
fn printMessage(message) {
    println(message);
}
```
Calling is simple:
```rust,ignore
# fn printMessage(message) {
#     println(message);
# }
# 
printMessage("Hello, world!");
```

You can also add multiple parameters:
```rust,ignore
fn printMessages(message1, message2) {
    println(message1);
    println(message2);
}
```