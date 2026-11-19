//! Marks builds compiled without optimisation (`opt-level = 0`, which CI uses for tests because it
//! compiles several times faster) with `cfg(unoptimized)`, so timing guards in tests can widen
//! their budget instead of failing on code the compiler did not optimise.

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(unoptimized)");
    if std::env::var("OPT_LEVEL").as_deref() == Ok("0") {
        println!("cargo::rustc-cfg=unoptimized");
    }
}
