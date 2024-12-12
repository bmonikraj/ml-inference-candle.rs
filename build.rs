/**
 * Required for proto builds
 * https://github.com/hyperium/tonic/blob/master/tonic-build/README.md
 */

use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
            .build_server(false)
            .out_dir("src/model/grpc")
            .compile_protos(
                &["proto/grpc_service.proto", "proto/health.proto"],
                &["proto/"],
            )?;
    Ok(())
}