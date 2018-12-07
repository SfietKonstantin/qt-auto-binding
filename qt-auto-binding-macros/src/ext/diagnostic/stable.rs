use super::DiagnosticExt;
use qt_auto_binding_core::diagnostic::{Diagnostic, Level};

impl DiagnosticExt for Diagnostic {
    fn emit(self) {
        match self.level {
            Level::Error => panic!("{}", self.message),
            Level::Warning => println!("Warning: {}", self.message),
            Level::Note => println!("Note: {}", self.message),
            Level::Help => println!("Help: {}", self.message),
        }
    }
}
