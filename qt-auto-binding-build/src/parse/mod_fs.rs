use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Result},
    path::{Path, PathBuf},
};

pub(crate) trait ReadModuleFs {
    fn read(&self, path: &Path, module: &str) -> Result<String>;
}

pub(crate) struct ModuleFsReader;

impl ModuleFsReader {
    fn file_path(path: &Path, module: &str) -> PathBuf {
        path.to_path_buf().join(format!("{}.rs", module))
    }

    fn mod_path(path: &Path, module: &str) -> PathBuf {
        path.to_path_buf().join(module).join("mod.rs")
    }

    fn read_content(path: &Path) -> Result<String> {
        let mut content = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

impl ReadModuleFs for ModuleFsReader {
    fn read(&self, path: &Path, module: &str) -> Result<String> {
        let file_path = ModuleFsReader::file_path(path, module);
        let mod_path = ModuleFsReader::mod_path(path, module);
        if file_path.exists() {
            ModuleFsReader::read_content(&file_path)
        } else if mod_path.exists() {
            ModuleFsReader::read_content(&mod_path)
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Could not find file for {}", module),
            ))
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ReadModuleFs;
    use std::{
        collections::HashMap,
        io::{Error, ErrorKind, Result},
        path::{Path, PathBuf},
    };

    pub(crate) struct TestModuleFsReader {
        results: HashMap<(PathBuf, String), Option<String>>,
    }

    impl TestModuleFsReader {
        pub(crate) fn new() -> Self {
            TestModuleFsReader {
                results: HashMap::new(),
            }
        }

        pub(crate) fn with_result(mut self, path: &Path, module: &str, result: &str) -> Self {
            self.results.insert(
                (path.to_path_buf(), module.to_string()),
                Some(result.to_string()),
            );
            self
        }
    }

    impl ReadModuleFs for TestModuleFsReader {
        fn read(&self, path: &Path, module: &str) -> Result<String> {
            let result = self
                .results
                .get(&(path.to_path_buf(), module.to_string()))
                .unwrap();

            if let Some(result) = result {
                Ok(result.clone())
            } else {
                Err(Error::new(ErrorKind::NotFound, "Not found"))
            }
        }
    }
}
