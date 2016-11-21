# Mu : build tooling & package management for Lean

`mu` is a build tool for Lean designed to make it easier to reliably distribute
and build both software and formalized mathematics with Lean.

Mu's design is heavily inspired by experience with other language's build
tools and package managers, primarily:
  - [Cargo](https://github.com/rust-lang/cargo)
  - [Cabal](https://www.haskell.org/cabal/)f
  - [SBT](http://www.scala-sbt.org/)
  - [Bundler](http://bundler.io/)

In its simplest form just drop into a directory with some Lean code and type:
```
mu build
```

The normal way to interact with `mu` is via a file named `Mu.toml` in
the root of your project.

Here is an example `Mu.toml`:

```toml
[package]
name = "cool-proofs"
version = "0.0.1"
authors = ["Jared Roesch <jroesch@cs.washington.edu>"]
```

At this time Mu is very much a work in progress and should be treated as
alpha quality software, if you happen to stumble upon it and would like to
contribute or provide feedback please contact me or file an issue.
