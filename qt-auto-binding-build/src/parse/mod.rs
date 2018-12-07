mod mod_fs;
mod errors;

use parse::mod_fs::{ModuleFsReader, ReadModuleFs};
use qt_auto_binding_core::{
    ext::iter::IteratorExt,
    parse::qobjects::from_stream,
    Object,
};
use self::errors::{Error, Result};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use syn::{
    parse_file,
    visit::{visit_file, Visit},
    ItemMod, Macro,
};

struct ProjectParser<R>
where
    R: ReadModuleFs,
{
    reader: R,
    objects: Vec<Object>,
}

impl<R> ProjectParser<R>
where
    R: ReadModuleFs,
{
    fn new(reader: R) -> Self {
        ProjectParser {
            reader,
            objects: Vec::new(),
        }
    }

    fn parse_recursively(&mut self, path: &Path, module: &str) -> Result<()> {
        println!("Parsing {}/{}", path.display(), module);

        let file = self.reader.read(path, module).map_err(|error| Error::IO {
            message: format!("Failed to read module {} in {}", module, path.display()),
            error,
        })?;
        let file = parse_file(&file).map_err(|_| Error::Source)?;

        let mut module_visitor = ModuleVisitor::new();
        visit_file(&mut module_visitor, &file);
        self.parse_module(&mut module_visitor)?;

        let new_path = ProjectParser::<R>::create_new_path(path, module);
        for sub_module in module_visitor.sub_modules {
            self.parse_recursively(&new_path, &sub_module)?;
        }

        Ok(())
    }

    fn parse(mut self) -> Result<Vec<Object>> {
        self.parse_recursively(Path::new("src"), "lib")?;
        self.check()?;
        Ok(self.objects)
    }

    fn create_new_path(path: &Path, module: &str) -> PathBuf {
        if module == "lib" {
            path.to_path_buf()
        } else {
            path.join(module)
        }
    }

    fn parse_module(&mut self, module_visitor: &mut ModuleVisitor) -> Result<()> {
        if !module_visitor.has_error {
            self.objects.append(&mut module_visitor.objects);
            Ok(())
        } else {
            Err(Error::Source)
        }
    }

    fn check(&self) -> Result<()> {
        let mut object_names = HashSet::new();
        let mut duplicated = None;
        let mut iter = self.objects.iter();

        while let (Some(object), None) = (iter.next(), duplicated) {
            let name = object.name();
            if object_names.contains(name) {
                duplicated = Some(name);
            } else {
                object_names.insert(name);
            }
        }

        if let Some(duplicated) = duplicated {
            Err(Error::DuplicatedObject {
                name: duplicated.to_string(),
            })
        } else {
            Ok(())
        }
    }
}

struct ModuleVisitor {
    sub_modules: Vec<String>,
    objects: Vec<Object>,
    has_error: bool,
}

impl ModuleVisitor {
    fn new() -> Self {
        ModuleVisitor {
            sub_modules: Vec::new(),
            objects: Vec::new(),
            has_error: false,
        }
    }
}

impl<'a> Visit<'a> for ModuleVisitor {
    fn visit_item_mod(&mut self, item: &'a ItemMod) {
        if item.content.is_none() {
            self.sub_modules.push(item.ident.to_string())
        }
    }

    fn visit_macro(&mut self, item: &'a Macro) {
        if let Some(macro_name) = item.path.segments.iter().single() {
            if macro_name.ident == "qobjects" {
                let result = from_stream(item.tts.clone());
                match result {
                    Ok(mut objects) => self.objects.append(&mut objects),
                    Err(_) => self.has_error = true,
                }
            }
        }
    }
}

pub(crate) fn parse() -> Result<Vec<Object>> {
    ProjectParser::new(ModuleFsReader).parse()
}

#[cfg(test)]
mod tests {
    use super::{mod_fs::tests::TestModuleFsReader, *};

    #[test]
    fn test_parse_recursively() {
        let lib = "mod a;\nmod b;";
        let mod_a = "mod a1;\nmod a2;";
        let mod_a1 = "";
        let mod_a2 = "";
        let mod_b = "mod internal {}";

        let reader = TestModuleFsReader::new()
            .with_result(Path::new("src"), "lib", lib)
            .with_result(Path::new("src"), "a", mod_a)
            .with_result(Path::new("src/a"), "a1", mod_a1)
            .with_result(Path::new("src/a"), "a2", mod_a2)
            .with_result(Path::new("src"), "b", mod_b);

        ProjectParser::new(reader).parse().unwrap();
    }

    #[test]
    fn test_parse_simple_qobjects() {
        let lib = "qobjects!{object MyObject{}}";
        let reader = TestModuleFsReader::new().with_result(Path::new("src"), "lib", lib);

        let results = ProjectParser::new(reader).parse().unwrap();

        assert_eq!(
            results,
            vec![Object::new("MyObject".to_string(), vec![], None)]
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_rust_code() {
        let lib = "fn main() {";
        let reader = TestModuleFsReader::new().with_result(Path::new("src"), "lib", lib);

        ProjectParser::new(reader).parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_qt_binding() {
        let lib = "qobjects!{object MyObject}";
        let reader = TestModuleFsReader::new().with_result(Path::new("src"), "lib", lib);

        ProjectParser::new(reader).parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_twice_the_same_name() {
        let lib = "qobjects!{object MyObject{}} qobjects!{object MyObject{}}";
        let reader = TestModuleFsReader::new().with_result(Path::new("src"), "lib", lib);

        ProjectParser::new(reader).parse().unwrap();
    }
}
