# Faces

This crate provides some primitives and a huge bunch of ready traits for any needs. It created especially for unifying interfaces between crates and software components and make it easier to create adapters between two incompatible APIs.

## What is inside?

This crate split to two modules: `types` and `traits`. `types` contains some basic types/layouts (e. g. minimal implementation of u32 2D vector named `faces::types::uvec2`). `traits` consist of interfaces.

## How to use it?

Ya 'now how to install it, so here is how to use it.

1. define module. For instance, you have module `softrender` which implements some simple 2D graphics on CPU.
2. define what set of interfaces it have and requires. Let's say, it has interface `AbsSoftRenderer` which
provides render function and requires `AbsShape` interface for each object you want to see on the screen.
3. find ready bunch of traits you need in [docs](https://docs.rs/faces) of this crate or create your own
on top of existing ones. Also, PRs and any kind of contribution is welcome. Who knows, maybe your cool trait
is quite useful for others?

## License

As always, either MIT or Apache-2.0 on your choice.
