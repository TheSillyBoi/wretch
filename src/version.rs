pub fn build_version() -> &'static str {
    Box::leak(format_version().into_boxed_str())
}

fn format_version() -> String {
    let base = env!("CARGO_PKG_VERSION");
    let commit = env!("GIT_HASH");
    let is_nightly = env!("NIGHTLY");

    if is_nightly == "true" {
        if !commit.is_empty() {
            format!("{base}-nightly ({commit})")
        } else {
            format!("{base}-nightly (commit unknown)")
        }
    } else if !commit.is_empty() {
        format!("{base} ({commit})")
    } else {
        format!("{base} (commit unknown)")
    }
}