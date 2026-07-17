// mod mod_pb;

// pub use self::mod_pb::*;
pub mod mod_pb {
    include!(concat!(env!("OUT_DIR"), "/generated_with_pure/mod.rs"));
}
pub use self::mod_pb::riemann::*;

#[test]
fn query_from_str() {
    let _ = Query::from("hello world");
}

#[test]
fn query_from_string() {
    let _ = Query::from("hello world".to_string());
}

#[test]
fn query_from_query() {
    let _ = Query::from(Query::new());
}
