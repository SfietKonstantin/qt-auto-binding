use std::path::Path;
use std::process::Command;

pub fn query(qmake_path: &Path) -> Vec<u8> {
    let command = Command::new(qmake_path)
        .args(&["-query"])
        .output()
        .unwrap_or_else(|err| panic!("Failed to run {}: {}", qmake_path.display(), err));

    if command.status.success() {
        command.stdout
    } else {
        panic!(
            "{} failed: {}",
            qmake_path.display(),
            String::from_utf8_lossy(&command.stderr)
        )
    }
}
