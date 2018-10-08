extern crate prost_build;

fn main() {
    // Build spanner
    prost_build::compile_protos(
        &[
          "googleapis/google/spanner/v1/spanner.proto",
        ], &[
          "googleapis/",
        ])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
