fn main() {
    let schema = "schemas/13_supergraph.graphql";
    println!("cargo:rerun-if-changed={schema}");

    // The supergraph is generated during the workshop (`cargo make supergraph`),
    // so the build must not fail before it exists.
    if !std::path::Path::new(schema).exists() {
        return;
    }

    cynic_codegen::register_schema("fso2f")
        .from_sdl_file(schema)
        .unwrap()
        .as_default()
        .unwrap();
}
