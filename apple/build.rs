use embuild::build::{CfgArgs, LinkArgs};

fn main() {
    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    LinkArgs::output_propagated("ESP_IDF").expect("LinkArgs output_propagated failed");

    // not sure what this is
    let cfg = CfgArgs::try_from_env("ESP_IDF").expect("CfgArgs try_from_env failed");
    cfg.output();
}
