#[cfg(not(feature = "nightly"))]
mod stable;
#[cfg(feature = "nightly")]
mod unstable;

pub(crate) trait DiagnosticExt {
    fn emit(self);
}
