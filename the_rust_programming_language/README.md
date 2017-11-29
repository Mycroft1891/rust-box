# The Rust Programming Language (The Book)

[__<= GO BACK__](../README.md)

1. [Introduction](#introduction)
  1. Installation
  2. Hello World
2. [Guessing Game](#guessing-game)
3. [Common Programming Concepts](#common-programming-concepts)
  1. Variables and Mutability
  2.

## Introduction

Installing rusk is as simple as running this line in bash:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Create a Hello World program ([View Source](intro/introduction.rs))

```rust
fn main() {}
```

`fn` defines a function and `main()` like in other compiled languages is the entry point for our program.

Using cargo:

> Cargo is Rust’s build system and package manager
> Cargo takes care of building your code, downloading the libraries your code depends on, and building those libraries.

To create a project using cargo ([View Source](intro/introduction_cargo)):

```
cargo new project_name --bin
```


## Guessing Game ([View Source](guessing/guessing_game/src/main.rs))

Using cargo we can create the guessing game.
Now we have to install a random number generator crate in our dependencies `.toml`:

```
[dependencies]
rand = "0.3.14"
```

Now we can build our program and add it as an external library:

```rust
extern crate rand;
use rand::Rng;

rand::thread_rng().gen_range(1,101)
```


## Common Programming Concepts

### Variables and Mutability

Variables are immutable by default in Rust.
