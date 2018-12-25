use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

pub(crate) struct Tool<'a> {
    name: &'static str,
    tool: &'a Path,
    args: Vec<OsString>,
}

impl<'a> Tool<'a> {
    pub(crate) fn moc(tool: &'a Path) -> Self {
        Tool {
            name: "moc",
            tool,
            args: Vec::new(),
        }
    }

    pub(crate) fn rcc(tool: &'a Path, name: &str) -> Self {
        let args = vec![OsString::from("-name"), OsString::from(name)];

        Tool {
            name: "rcc",
            tool,
            args,
        }
    }

    pub(crate) fn exec(&self, out_dir: &Path, input: &Path) -> PathBuf {
        let output = input
            .file_stem()
            .expect(&format!("{} takes files as input.", self.name));
        let output = out_dir.join(format!("{}_{}.cpp", self.name, output.to_string_lossy()));

        let command = {
            let args = &[
                OsString::from(input),
                OsString::from("-o"),
                OsString::from(&output),
            ];
            let args = self.args.iter().chain(args.into_iter());

            Command::new(self.tool).args(args).output().unwrap()
        };

        if command.status.success() {
            output
        } else {
            panic!(
                "Failed to execute {}.\n\n{}",
                self.name,
                String::from_utf8_lossy(&command.stderr)
            )
        }
    }
}
