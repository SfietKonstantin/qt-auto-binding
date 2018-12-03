use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

pub(crate) fn exec(moc_path: &Path, out_dir: &Path, input: &Path) -> PathBuf {
    let output = input.file_stem().expect("moc takes files as input.");
    let output = out_dir.join(format!("moc_{}.cpp", output.to_string_lossy()));

    let command = {
        let input_arg = input.as_os_str();
        let o_flag_arg = OsStr::new("-o");
        let output_arg = output.as_os_str();

        Command::new(moc_path)
            .args(&[input_arg, o_flag_arg, output_arg])
            .output()
            .unwrap()
    };

    if command.status.success() {
        output
    } else {
        panic!(
            "Failed to execute moc.\n\n{}",
            String::from_utf8_lossy(&command.stderr)
        )
    }
}
