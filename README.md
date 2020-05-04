# netting

![build](https://github.com/Netting-mesh/netting/workflows/Rust/badge.svg?branch=master) 

Netting is an unopinionated, simple, fast service mesh written in Rust. Its main goal is to solve the
issue service mesh's face now with forcing users into using specific infrastructure as well as being
hard to integrate into an already exisiting kubernetes cluster. Netting will be a plug & play type of 
mesh.

This repository is the control plane service that will act as the management pod within a cluster.
[side-boat](https://github.com/Netting-mesh/side-boat) is the sidecar proxy that will be injected into each pod within a cluster.

## Installation

### Compiling from source

You can compile `netting` by running `cargo build`. You must have Rust Nightly enabled by default to compile.

