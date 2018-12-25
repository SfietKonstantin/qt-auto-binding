use super::DiagnosticExt;
use proc_macro;
use proc_macro2::Span;
use qt_auto_binding_core::diagnostic::{Diagnostic, Level};

fn convert_spans(spans: Vec<Span>) -> Vec<proc_macro::Span> {
    spans.into_iter().map(Span::unstable).collect::<Vec<_>>()
}

impl DiagnosticExt for Diagnostic {
    fn emit(self) {
        let level = match self.level {
            Level::Error => proc_macro::Level::Error,
            Level::Warning => proc_macro::Level::Warning,
            Level::Note => proc_macro::Level::Note,
            Level::Help => proc_macro::Level::Help,
        };
        let mut diagnostic = proc_macro::Diagnostic::new(level, self.message);
        diagnostic.set_spans(convert_spans(self.spans));

        for child in self.children {
            match child.level {
                Level::Error => {
                    diagnostic = diagnostic.span_error(convert_spans(child.spans), child.message)
                }
                Level::Warning => {
                    diagnostic = diagnostic.span_warning(convert_spans(child.spans), child.message)
                }
                Level::Note => {
                    diagnostic = diagnostic.span_note(convert_spans(child.spans), child.message)
                }
                Level::Help => {
                    diagnostic = diagnostic.span_help(convert_spans(child.spans), child.message)
                }
            }
        }

        diagnostic.emit();
    }
}
