//! Checking utilities

use crate::diagnostic::Diagnostic;

/// Result of a check
pub type CheckResult = Result<(), Vec<Diagnostic>>;

/// Checks an input
///
/// This trait describes a check to be performed on an input.
/// A check either returns success, that is `Ok(())`, or error,
/// that is a list of [`Diagnostic`]s
///
/// [`Diagnostic`]: ../diagnostic/struct.diagnostic.html
pub trait Check<T> {
    /// Perform the check on an input
    ///
    /// Implement this method to perform a check on
    /// an input.
    fn check(&mut self, input: &T) -> CheckResult;
}

/// A composite checker
///
/// A checker contains several [`Check`]s. For a given input,
/// it will apply each check, one after the other.
///
/// [`Check`]: trait.Check.html
pub struct Checker<T> {
    checks: Vec<Box<dyn Check<T>>>,
}

impl<T> Checker<T> {
    /// Create a new `Checker` without [`Check`]
    ///
    /// [`Check`]: trait.Check.html
    pub(crate) fn new() -> Self {
        Checker { checks: Vec::new() }
    }

    /// Add a check [`Check`]
    ///
    /// This chaining method can be used to add a [`Check`] to
    /// a `Checker`. `Check`s are executed in the same order as
    /// they are added.
    ///
    /// [`Check`]: trait.Check.html
    pub(crate) fn with_check(mut self, check: Box<dyn Check<T>>) -> Self {
        self.checks.push(check);
        self
    }

    /// Perform the check on an input
    ///
    /// This method will check the input based on all [`Check`]s that have
    /// been added to this `Checker`.
    ///
    /// [`Check`]: trait.Check.html
    pub(crate) fn check(&mut self, input: &T) -> CheckResult {
        self.checks
            .iter_mut()
            .map(|check| check.check(input))
            .fold(Ok(()), Checker::<T>::fold_result)
    }

    fn fold_result(first: CheckResult, second: CheckResult) -> CheckResult {
        match (first, second) {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(diagnostics)) => Err(diagnostics),
            (Err(diagnostics), Ok(_)) => Err(diagnostics),
            (Err(mut first), Err(mut second)) => {
                first.append(&mut second);
                Err(first)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::Level;

    struct DivByNCheck {
        n: i32,
    }

    impl DivByNCheck {
        fn new(n: i32) -> Self {
            DivByNCheck { n }
        }
    }

    impl Check<i32> for DivByNCheck {
        fn check(&mut self, input: &i32) -> CheckResult {
            if input % self.n == 0 {
                Ok(())
            } else {
                let diagnostic = Diagnostic::new(Level::Error)
                    .with_message(format!("This number cannot be divided by {}", self.n));
                Err(vec![diagnostic])
            }
        }
    }

    #[test]
    fn test_check() {
        let mut checker = Checker::new() //
            .with_check(Box::new(DivByNCheck::new(2)));

        assert!(checker.check(&1).is_err());
        assert_eq!(checker.check(&1).unwrap_err().len(), 1);

        assert!(checker.check(&2).is_ok());

        assert!(checker.check(&3).is_err());
        assert_eq!(checker.check(&3).unwrap_err().len(), 1);

        assert!(checker.check(&4).is_ok());
    }

    #[test]
    fn test_multiple_checks() {
        let mut checker = Checker::new()
            .with_check(Box::new(DivByNCheck::new(2)))
            .with_check(Box::new(DivByNCheck::new(3)));

        assert!(checker.check(&1).is_err());
        assert_eq!(checker.check(&1).unwrap_err().len(), 2);

        assert!(checker.check(&2).is_err());
        assert_eq!(checker.check(&2).unwrap_err().len(), 1);

        assert!(checker.check(&3).is_err());
        assert_eq!(checker.check(&3).unwrap_err().len(), 1);

        assert!(checker.check(&6).is_ok());
    }
}
