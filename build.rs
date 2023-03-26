fn main() {
    prost_build::compile_protos(&["src/protobuf/eldp.proto"], &["src/"]).unwrap();
}