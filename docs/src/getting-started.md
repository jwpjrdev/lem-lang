# Getting Started

## Installation

To install Lem on your machine, there are a couple options:

If you have Rust and Cargo installed on your machine:
```
$ cargo install lem
```
If you don't, or you don't want to build Lem from scratch, there are pre-built binaries available:
```
$ <curl & unzip release from GitHub>
$ <don't forget about adding the binary to PATH>
```

## Your First Script

All Lem scripts have a name, and end with a file extension of `.lem`. This requirement will be loosened in the future (see #2).

Create a file that ends in `.lem`, such as `helloWorld.lem`:
```
$ touch helloWorld.lem
```
Open the file in your text editor of choice, and write the following:
<!-- for now, use JS for code samples until syntax highlighting is added for Lem -->
```js
println("Hello, world!")
```
Now, open your terminal, `cd` to the working directory, and run the following:
```
$ lem helloWorld.lem
```
This is what you should see:
```
$ lem helloWorld
Hello, world!
```
Congratulations! You have written your first Lem script!
