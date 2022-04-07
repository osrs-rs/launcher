use std::{env, process::Command};

fn main() {
    // Set the access token (used for account related details)
    env::set_var("JX_ACCESS_TOKEN", "123");

    // Set the refresh token (unused for the time being, but assumed for refreshing the token in the client)
    env::set_var("JX_REFRESH_TOKEN", "456");

    // Launch the Oldschool Runescape client
    Command::new("JagexLauncher")
        .args(["oldschool", "../oldschool/oldschool.prm"])
        .spawn()
        .expect("failed spawning JagexLauncher");
}
