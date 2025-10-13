use tonic_prost_build::configure;

fn main() {
    unsafe { std::env::set_var("PROTOC", protobuf_src::protoc()) };
    configure()
        .build_server(false)
        .compile_protos(
            &[
                "mev-protos/auth.proto",
                "mev-protos/block.proto",
                "mev-protos/block_engine.proto",
                "mev-protos/bundle.proto",
                "mev-protos/packet.proto",
                "mev-protos/relayer.proto",
                "mev-protos/searcher.proto",
                "mev-protos/shared.proto",
                "mev-protos/shredstream.proto",
                "mev-protos/trace_shred.proto",
            ],
            &["mev-protos"],
        )
        .unwrap();
}
