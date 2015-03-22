# riemann_client [![](https://img.shields.io/github/tag/borntyping/rust-riemann_client.svg)](https://github.com/borntyping/rust-riemann_client/tags) [![](https://img.shields.io/travis/borntyping/rust-riemann_client.svg)](https://travis-ci.org/borntyping/rust-riemann_client) [![](https://img.shields.io/github/issues/borntyping/rust-riemann_client.svg)](https://github.com/borntyping/rust-riemann_client/issues)

A [Riemann](http://riemann.io/) client library for [Rust](http://www.rust-lang.org/).

* [Source on GitHub](https://github.com/borntyping/rust-riemann_client)
* [Packages on Crates.io](https://crates.io/crates/riemann_client)
* [Builds on Travis CI](https://travis-ci.org/borntyping/rust-riemann_client)

Usage
-----

See the usage examples in `./examples`.

Development
-----------

The protocol buffer defintion can be updated by replacing `src/proto/mod.proto` with the [latest defintion from the Riemann source](https://raw.githubusercontent.com/aphyr/riemann-java-client/master/src/main/proto/riemann/proto.proto) and running `make`.

You will need to have `protoc` and `protoc-gen-rust` installed. `protoc` is provided by the `protobuf-compiler` package on Debian based systems. Instructions for installing `protoc-gen-rust` this are availible in the [README for rust-protobuf](https://github.com/stepancheg/rust-protobuf).

Licence
-------

`riemann_client` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

Authors
-------

Written by [Sam Clements](sam@borntyping.co.uk).
