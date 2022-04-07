use std::{env, process::Command};

fn main() {
    env::set_var("JX_ACCESS_TOKEN", "123");
    env::set_var("JX_REFRESH_TOKEN", "456");

    Command::new("awer");
}
