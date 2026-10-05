fn main() {
    println!("cargo:rerun-if-changed=schemas/13_supergraph.graphql");

    cynic_codegen::register_schema("fso2f")
        .from_sdl_file("schemas/13_supergraph.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
