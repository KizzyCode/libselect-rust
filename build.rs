use cc::Build;


/// Select a target dependent select implementation
fn select_impl() -> &'static str {
    // Select the unix shim
    #[cfg(target_family = "unix")]
    return "src/select_impl/select_unix.c";

    // Select the windows shim
    #[cfg(target_family = "windows")]
    return "src/select_impl/select_win.c";

    // Panic if we run on a different platform
    #[allow(unreachable_code)]
    {
        panic!("No select shim available for the current target");
    }
}


fn main() {
    Build::new()
        .file(select_impl())
        .warnings_into_errors(true)
        .compile("select");
}
