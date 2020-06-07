//! Various error types returned by methods in the time crate.

use crate::alloc_prelude::*;
pub use crate::format::ParseError as Parse;
use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact
/// error returned. `Result<_, time::Error>` will work in these situations.
// Boxing the `ComponentRangeError` reduces the size of `Error` from 72 bytes to
// 16.
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[cfg_attr(supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConversionRange,
    ComponentRange(Box<ComponentRange>),
    Parse(Parse),
    Format(Format),
    IndeterminateOffset,
    #[cfg(not(supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Error {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            e @ Error::ConversionRange | e @ Error::IndeterminateOffset => e.fmt(f),
            Error::ComponentRange(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
            Error::Format(e) => e.fmt(f),
            #[cfg(not(supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(std)]
impl std::error::Error for Error {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            err @ Error::ConversionRange | err @ Error::IndeterminateOffset => Some(err),
            Error::ComponentRange(box_err) => Some(box_err.as_ref()),
            Error::Parse(err) => Some(err),
            Error::Format(err) => Some(err),
            #[cfg(not(supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConversionRange;

impl fmt::Display for ConversionRange {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(std)]
impl std::error::Error for ConversionRange {}

impl From<ConversionRange> for Error {
    #[inline(always)]
    fn from(_: ConversionRange) -> Self {
        Error::ConversionRange
    }
}

/// An error type indicating that a component provided to a method was out of
/// range, causing a failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need
// for a type parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentRange {
    /// Name of the component.
    pub component_name: &'static str,
    /// Minimum allowed value, inclusive.
    pub minimum: i64,
    /// Maximum allowed value, inclusive.
    pub maximum: i64,
    /// Value that was provided.
    pub value: i64,
    /// The minimum and/or maximum is only valid with the following values.
    pub(crate) given: Vec<(&'static str, i64)>,
}

impl fmt::Display for ComponentRange {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be in the range {}..={}",
            self.component_name, self.minimum, self.maximum
        )?;

        let mut iter = self.given.iter();
        if let Some((name, value)) = iter.next() {
            write!(f, " given {}={}", name, value)?;
            for (name, value) in iter {
                write!(f, ", {}={}", name, value)?;
            }
        }

        write!(f, " (was {})", self.value)
    }
}

impl From<ComponentRange> for Error {
    #[inline(always)]
    fn from(original: ComponentRange) -> Self {
        Error::ComponentRange(Box::new(original))
    }
}

#[cfg(std)]
impl std::error::Error for ComponentRange {}

impl From<Parse> for Error {
    #[inline(always)]
    fn from(original: Parse) -> Self {
        Error::Parse(original)
    }
}

/// The system's UTC offset could not be determined at the given datetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IndeterminateOffset;

impl fmt::Display for IndeterminateOffset {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The system's UTC offset could not be determined")
    }
}

#[cfg(std)]
impl std::error::Error for IndeterminateOffset {}

impl From<IndeterminateOffset> for Error {
    #[inline(always)]
    fn from(_: IndeterminateOffset) -> Self {
        Error::IndeterminateOffset
    }
}

/// An error occurred while formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(supports_non_exhaustive, non_exhaustive)]
pub enum Format {
    /// The format provided requires more information than the type provides.
    InsufficientTypeInformation,
    /// An error occurred while formatting into the provided stream.
    StdFmtError,
    #[cfg(not(supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Format {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::InsufficientTypeInformation => {
                f.write_str("The format provided requires more information than the type provides.")
            }
            Format::StdFmtError => fmt::Error.fmt(f),
            #[cfg(not(supports_non_exhaustive))]
            Format::__NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(std)]
impl std::error::Error for Format {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Format::StdFmtError => Some(&fmt::Error),
            _ => None,
        }
    }
}

// This is strictly necessary to be able to use `?` with various formatters.
impl From<fmt::Error> for Format {
    #[inline(always)]
    fn from(_: fmt::Error) -> Self {
        Format::StdFmtError
    }
}

impl From<Format> for Error {
    #[inline(always)]
    fn from(error: Format) -> Self {
        Error::Format(error)
    }
}
