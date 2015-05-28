extern crate riemann_client;

use riemann_client::proto::Query;

#[test]
fn query_from_str() {
    Query::from("hello world");
}

#[test]
fn query_from_string() {
    Query::from("hello world".to_string());
}

#[test]
fn query_from_query() {
    Query::from(Query::new());
}
