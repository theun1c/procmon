use std::process::Command;

fn main() {
    let mut top_cmd = Command::new("sh")
        .arg("-c")
        .arg("top -i -d 1 | grep %Cpu")
        .spawn()
        .expect("ls command failed to start");

    top_cmd.wait().expect("failed to wait child");
}
