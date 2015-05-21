# riemann_client [![](https://img.shields.io/crates/v/riemann_client.svg)](https://crates.io/crates/riemann_client) [![](https://img.shields.io/crates/l/riemann_client.svg)](https://crates.io/crates/riemann_client) [![](https://img.shields.io/travis/borntyping/rust-riemann_client.svg)](https://travis-ci.org/borntyping/rust-riemann_client)

A [Riemann](http://riemann.io/) client library and command line interface.

* [Source on GitHub](https://github.com/borntyping/rust-riemann_client)
* [Packages on Crates.io](https://crates.io/crates/riemann_client)
* [Builds on Travis CI](https://travis-ci.org/borntyping/rust-riemann_client)

Usage
-----

```
$ riemann-cli send --service riemann_cli --state ok --metric-d 11
--> { state: "ok" service: "riemann_cli" metric_d: 11 }
<-- { ok: true }
```

```
$ riemann-cli query 'service = "riemann_cli"'
HOSTNAME   TIME       SERVICE              METRIC     STATE
           1432128319 riemann_cli          11         ok
```

Run `riemann-cli --help` for a list of options availible for the command line interface.

See the `examples` directory for examples of querying and sending events with the library.

Development
-----------

To build the library alone, without the command line interface and it's dependencies, run `cargo build --lib --no-default-features`.

The protocol buffer defintion can be updated by replacing `src/proto/mod.proto` with the [latest defintion from the Riemann source](https://raw.githubusercontent.com/aphyr/riemann-java-client/master/src/main/proto/riemann/proto.proto) and running `make`. You will need to have `protoc` and `protoc-gen-rust` installed. `protoc` is provided by the `protobuf-compiler` package on Debian based systems. Instructions for installing `protoc-gen-rust` this are availible in the [README for rust-protobuf](https://github.com/stepancheg/rust-protobuf).

Licence
-------

`riemann_client` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

Authors
-------

Written by [Sam Clements](sam@borntyping.co.uk).
