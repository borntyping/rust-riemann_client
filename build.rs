use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .cargo_out_dir("generated_with_pure")
        .input("src/proto/riemann.proto")
        .include("src/proto")
        .run_from_script();
}
