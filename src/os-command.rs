use std::process::Command;

fn os_commands_example_1() {
    let echo_cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "Echo aye there from Windows!"])
            .output()
            .expect("Failed execute command")
    } else {
        Command::new("sh")
            .args(["-c", "Echo aye there from Penguin"])
            .output()
            .expect("Failed execute command")
    };

    println!("\n\n");
    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("Could not parse byte response.");
    println!("{}", cmd_output);
    println!("\n\n");
}

fn os_commands_example_2() {
    println!("\n\n");

    let mut cmd_root = Command::new("ls");

    cmd_root.status().expect("Failed to execute list command.");

    println!("\n\n");

    cmd_root
        .current_dir("src")
        .status()
        .expect("Failed to execute list command.");

    println!("\n\n");
}

fn main() {
    // os_commands_example_1()
    os_commands_example_2()
}
