//! Diagnostics
//!
//! This module contains a light shim over [`proc_macro`]'s diagnostic API.
//!
//! [`proc_macro`]: https://doc.rust-lang.org/proc_macro/index.html
use proc_macro2::Span;

/// A diagnostic level
#[derive(Debug)]
pub enum Level {
    /// An error
    Error,
    /// A warning
    Warning,
    /// A note
    Note,
    /// A help
    Help,
}

/// A diagnostic
///
/// A diagnostic describes an issues that happened when parsing code provided to [`qt_binding`].
///
/// [`qt_binding`]: ../../qt_binding/index.html
#[derive(Debug)]
pub struct Diagnostic {
    /// Level
    pub level: Level,
    /// Message
    pub message: String,
    /// Spans
    pub spans: Vec<Span>,
    /// Children
    pub children: Vec<Diagnostic>,
}

impl Diagnostic {
    /// Creates a new diagnostic
    ///
    /// A diagnostic is always created with a `Level`.
    pub fn new(level: Level) -> Self {
        Diagnostic {
            level,
            message: String::new(),
            spans: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Set the diagnostic's message
    pub fn with_message<T>(mut self, message: T) -> Self
    where
        T: Into<String>,
    {
        self.message = message.into();
        self
    }

    /// Set the diagnostic's span
    pub fn with_span(mut self, span: Span) -> Self {
        self.spans = vec![span];
        self
    }

    /// Set the diagnostic's spans
    pub fn with_spans(mut self, spans: Vec<Span>) -> Self {
        self.spans = spans;
        self
    }

    /// Add a child diagnostic
    pub fn add_child(mut self, child: Diagnostic) -> Self {
        self.children.push(child);
        self
    }
}
