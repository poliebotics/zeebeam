use sp1_build::{build_program_with_args, BuildArgs};

/// Path-independent guest build, identical in method to the per-row relation's build.rs: remap the
/// cargo home and the working tree root so the ELF embeds no machine path, and build --locked.
fn main() {
    let cargo_home = std::env::var("CARGO_HOME")
        .ok()
        .or_else(|| std::env::var("HOME").ok().map(|h| format!("{h}/.cargo")))
        .expect("CARGO_HOME or HOME");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    // script/ -> zeebeam_chain_sp1_candidate -> rust -> zeebeam-science -> BOSUN
    let root = std::path::Path::new(&manifest)
        .ancestors()
        .nth(4)
        .expect("working tree root")
        .to_string_lossy()
        .to_string();
    let args = BuildArgs {
        rustflags: vec![
            format!("--remap-path-prefix={cargo_home}=/cargo"),
            format!("--remap-path-prefix={root}=/bosun"),
        ],
        locked: true,
        ..Default::default()
    };
    build_program_with_args("../program", args)
}
