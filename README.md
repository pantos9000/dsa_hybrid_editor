# DSA Hybrid Editor

This Editor is is a small tool for a home-made roleplaying system, the *DSA Hybrid Ruleset*, a
melange between *Savage Worlds* and *DSA v4*.

The editor allows to create and edit a character, and to compare it to another character,
simulating a simple duel between the two. Apart from displaying different Probabilities
(e.g. to win), the editor also displays the probability gradient for each possible change, i.e.
how the win chance will be affected when increasing or decreasing a value.

[![Build status](https://github.com/pantos9000/dsa_hybrid_editor/actions/workflows/ci.yml/badge.svg)](https://github.com/pantos9000/dsa_hybrid_editor/actions)

[![GitHub Release](https://img.shields.io/github/v/release/pantos9000/dsa_hybrid_editor?label=latest%20version)](https://github.com/pantos9000/dsa_hybrid_editor/releases)


## Installation/Usage

* Navigate to the [release page](https://github.com/pantos9000/dsa_hybrid_editor/releases)
* Download the archive for your platform, e.g. `x86_64-pc-windows-msvc`
* (Optional) Check the sha256 sum of your download
* Unpack the archive and run the executable

## Building

If you have the [cargo/rust toolchain](https://www.rust-lang.org/) installed on your system, run:
```bash
cargo build --release
```
