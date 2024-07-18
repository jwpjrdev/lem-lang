# Introduction

Lem is a minimal, only-what-you-need scripting language, built as an alternative to Bash.

## Why?

While Bash has proven powerful again and again, its scripts have also proved unmaintainable time and time again. This is in part due to its unclear syntax. Lem aims to have a simplistic, readable syntax that makes maintaining scripts much more enjoyable.

Unlike Bash, Lem attempts to provide a single way to perform a single operation. Bash is notably able to do one thing in four or five different ways, which can make it hard for maintainers to remember what does what without comments littered across the script.

Lem also has intuitive methods, in hopes that you can guess it before you have to look it up. For example, splitting a string is simple, and there is only one correct way to do it: `"Hello, world!".split(" ")`. In Bash, this is much more complex than it needs to be. See [here](https://stackoverflow.com/a/45201229) for 9 ways to split a string, with only one of them being the proper method.

## How?

By simplifying the syntax and removing redundant methods of performing operations, a lot can be overcome. Many maintenance issues can be overcome by making it easier to read scripts.

## Getting Started
See the [getting started](./getting-started.md) page for installation and a first script.