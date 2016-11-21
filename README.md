# Gallus: Modernize your Coq developments!

`gallus` is a build tool for Coq designed to make it easier to reliably
distribute and build software written in Coq.

Gallus' design is heavily inspired by experience with other language's build
tools and package managers, primarily:
  - [Cargo](https://github.com/rust-lang/cargo)
  - [Cabal](https://www.haskell.org/cabal/),
  - [SBT](http://www.scala-sbt.org/)
  - [Bundler](http://bundler.io/)

It is designed to eventually interoperate with variety of existing tools
that are used with Coq, such as:
- make
- opam
- coqdep
- and hopefully more

In its simplest form just drop into a directory with some Coq code and type:
```
gallus build
```

The normal way to interact with `gallus` is via a file named `Gallus.toml` in
the root of your project.

Here is an example `Gallus.toml`:

```toml
[package]
name = "cool-proofs"
version = "0.0.1"
authors = ["Jared Roesch <jroesch@cs.washington.edu>"]
```

At this time Gallus is very much a work in progress and should be treated as
alpha quality software, if you happen to stumble upon it and would like to
contribute or provide feedback please contact me or file an issue.
