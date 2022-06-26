use std::process::Command;

fn error_handling_example(dir: &str) {
    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    let _ = match list_cmd.current_dir(dir).status() {
        Ok(cmd) => Some(cmd),
        Err(_) => {
            println!("{}", "Directory not found");
            None
        }
    };

    println!("\n\n")
}

fn main() {
    error_handling_example("src");
    error_handling_example("lib");
}
