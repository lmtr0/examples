fn main() {
    tonic_build::compile_protos("proto/echo.proto").unwrap();
}