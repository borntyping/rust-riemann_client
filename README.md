# riemann_client [![](https://img.shields.io/crates/v/riemann_client.svg)](https://crates.io/crates/riemann_client) [![](https://img.shields.io/crates/l/riemann_client.svg)](https://crates.io/crates/riemann_client) [![](https://img.shields.io/travis/borntyping/rust-riemann_client.svg)](https://travis-ci.org/borntyping/rust-riemann_client)

A [Riemann](http://riemann.io/) client library and command line interface.

- [Source on GitHub](https://github.com/borntyping/rust-riemann_client)
- [Packages on Crates.io](https://crates.io/crates/riemann_client)
- [Builds on Travis CI](https://travis-ci.org/borntyping/rust-riemann_client)

## Usage

```bash
$ riemann-cli send --service riemann_cli --state ok --metric-d 11
--> { Event { time: None, state: Some("ok"), service: Some("riemann_cli"), host: None, description: None, tags: [], ttl: None, attributes: [], time_micros: None, metric_sint64: None, metric_d: Some(11.0), metric_f: None, special_fields: SpecialFields { unknown_fields: UnknownFields { fields: None }, cached_size: CachedSize { size: 0 } } } }
<-- { Msg { ok: Some(true), error: None, states: [], query: MessageField(None), events: [], special_fields: SpecialFields { unknown_fields: UnknownFields { fields: None }, cached_size: CachedSize { size: 0 } } } }
```

```bash
$ riemann-cli query 'service = "riemann_cli"'
HOSTNAME   TIME       SERVICE              METRIC     STATE
           1432128319 riemann_cli          11         ok
```

Run `riemann-cli --help` for a list of options available for the command line interface.

See the `examples` directory for examples of querying and sending events with the library.

## Development

To build the library alone, without the command line interface and it's dependencies, run `cargo build --lib --no-default-features`.

The protocol buffer definition can be updated by replacing `src/proto/mod.proto` with the [latest definition from the Riemann source](https://raw.githubusercontent.com/riemann/riemann-java-client/refs/heads/main/riemann-java-client/src/main/proto/riemann/proto.proto) and running `make`. You will need to have `protoc` and `protoc-gen-rust` installed. `protoc` is provided by the `protobuf-compiler` package on Debian based systems. Instructions for installing `protoc-gen-rust` this are available in the [README for rust-protobuf](https://github.com/stepancheg/rust-protobuf).

### mTLS

To create the test certificates the following commands were used.

```bash
$ cd test_certs

# create the CA
$ openssl genrsa -out ca.key 4096
$ openssl req -new -x509 -days 3650 -key ca.key \
  -out ca.crt \
  -subj "/CN=Riemann-Test-CA/O=Example/C=US"

# generate a certificate for the server
$ openssl genrsa -out riemann_server.key 4096

# needed as we run riemann under localhost; rustls would otherwise throw an error,
# that no DNS name from the certificate matches localhost
# adapted from https://gist.github.com/justinhartman/36cccc6ce26a01378369b35fd048748a#file-02_openssl-cnf
$ cat > riemann_server.cnf <<EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
prompt = no

[req_distinguished_name]
CN = riemann.example.com
O = Example
C = US

[v3_req]
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
DNS.2 = riemann.example.com
IP.1  = 127.0.0.1
EOF

$ openssl req -new -key riemann_server.key \
  -out riemann_server.csr \
  -config riemann_server.cnf

$ openssl x509 -req -in riemann_server.csr \
  -CA ca.crt -CAkey ca.key -CAcreateserial \
  -out riemann_server.crt -days 365 \
  -sha256 \
  -extensions v3_req -extfile riemann_server.cnf

# create the client cert
$ openssl genrsa -out client.key 4096
$ openssl req -new -key client.key \
  -out client.csr \
  -subj "/CN=riemann-client/O=Example/C=US"
$ openssl x509 -req -in client.csr \
  -CA ca.crt -CAkey ca.key -CAcreateserial \
  -out client.crt -days 365 \
  -sha256
```

The following is an example configuration for the server using TLS

```riemann.config
(logging/init {:file "riemann.log"})
(tcp-server {:host "0.0.0.0"
             :port 5554
             :tls? true
             :key "riemann_server.key"
             :cert "riemann_server.crt"
             :ca-cert "ca.crt"})

(instrumentation {:interval 1})

(periodically-expire 1)

(let [index (tap :index (index))]
  (streams
    (default :ttl 3
      (expired #(prn "Expired" %))
      (where (not (service #"^riemann "))
             index))))
```

To the TLS you should start the server in one terminal.

```bash
$ cd test_certs
$ riemann ./riemann.config
```

In another terminal you can use the provided examples to the functionality.
Note that the paths to `./test_certs/*` are hardcoded, thus the examples have to be run from the "root" directory.

```bash
$ cargo run --example query_tls
  Compiling riemann_client v0.9.0 (/home/marts/P/rust/rieman-main)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `/home/marts/.local/state/cargo/debug/examples/event_tls`
# as there is no event stored we generate one
$ cargo run --example event_tls
  Compiling riemann_client v0.9.0 (/home/marts/P/rust/rieman-main)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `/home/marts/.local/state/cargo/debug/examples/event_tls`

$ cargo run --example query_tls
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
   Running `/home/marts/.local/state/cargo/debug/examples/query_tls`
HOSTNAME   TIME       SERVICE                                                 METRIC     STATE     
nixos      1782054519 rust-riemann_client                                     128.128    ok
```

## Licence

`riemann_client` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

It was also directly inspired by the Python [riemann-client](http://github.com/borntyping/python-riemann-client) by the same author.

## Authors

Written by [Sam Clements](sam@borntyping.co.uk).
