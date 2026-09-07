use sp1_build::{build_program_with_args, BuildArgs};

/// Path-independent guest build: remap the cargo home (registry and git checkouts) and the working
/// tree root so panic locations and debug strings carry no machine-specific paths. Two machines
/// with the same sources, lockfile and toolchain then produce byte-identical ELFs and the same
/// verification key. (2 Sep 2026: without this, a build on another host differed by 32 embedded
/// registry paths and therefore by its vkey.)
fn main() {
    let cargo_home = std::env::var("CARGO_HOME")
        .ok()
        .or_else(|| std::env::var("HOME").ok().map(|h| format!("{h}/.cargo")))
        .expect("CARGO_HOME or HOME");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    // script/ -> row_binding_join_membership_sp1_candidate -> rust -> zeebeam-science -> BOSUN
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
