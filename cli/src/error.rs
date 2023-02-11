//! Defines an error type that any function or method defined in this crate returns.

use std::error as stderr;
use std::fmt;

/// Convenient type for making more concise wrapping the standard error trait
/// object into a Box.
pub type BoxError = Box<dyn stderr::Error + Send + Sync>;

/// The error type that expose general kinds of errors that are common to all
/// the modules of this crate.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Identifies unexpected errors which happen because of the state of the system where the
    /// application is running, for example, insufficient resources, OS failures, etc.
    Internal(Internal),
    /// Identifies errors due to invalid arguments passed to function or methods or assigned values
    /// to configurations.
    InvalidArguments(Args),
    /// Identifies errors related with one of the external systems that the application rely on.
    External(External),
}

impl Error {
    /// Convenient constructor for creating an InvalidArguments Error.
    /// See [`Args`] documentation to know about the convention for the value of
    /// the `names` parameter because this constructor panics if they are
    /// violated.
    pub(crate) fn invalid_arguments(names: &str, msg: &str) -> Self {
        Self::InvalidArguments(Args::new(names, msg))
    }

    /// Convenient constructor for creating a External Error.
    pub(crate) fn external(origin: BoxError, system: ExternalSystem) -> Self {
        Self::External(External {
            system,
            inner: origin,
        })
    }

    /// Convenient constructor for creating an Internal Error.
    pub(crate) fn internal(ctx_msg: &str, error: BoxError) -> Self {
        Self::Internal(Internal {
            ctx_msg: String::from(ctx_msg),
            error,
        })
    }
}

impl stderr::Error for Error {
    fn source(&self) -> Option<&(dyn stderr::Error + 'static)> {
        match self {
            Error::InvalidArguments { .. } => None,
            Error::Internal(i) => i.source(),
            Error::External(n) => n.source(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::InvalidArguments(a) => a.fmt(f),
            Error::Internal(i) => i.fmt(f),
            Error::External(n) => n.fmt(f),
        }
    }
}

/// Represents invalid arguments error regarding the business domain.
///
/// # Example
///
/// ```
/// use storj_uplink_lib::Error;
///
/// fn positive_non_zero_div_and_mul(a: i64, b: i64, div: i64) -> Result<i64, Error> {
///     if div == 0 {
///         return Err(Error::new_invalid_arguments("div", "div cannot be 0"));
///     }
///
///     if (a == 0 && b != 0) || (a != 0 && b == 0) {
///         return Err(Error::new_invalid_arguments(
///             "(a,b)", "a and b can only be 0 if both are 0",
///         ));
///     }
///
///     if (a >= 0 && b >= 0 && div > 0) || (a <= 0 && b <= 0 && div < 0 ) {
///         return Ok((a/div) * (b/div));
///     }
///
///     Err(Error::new_invalid_arguments(
///         "<all>", "all the arguments must be positive or negative, they cannot be mixed",
///     ))
/// }
/// ```
#[derive(Debug)]
pub struct Args {
    /// `names` is one or several parameters names; it has several conventions
    /// for expressing the involved parameters.
    ///
    /// * When a specific parameter is invalid its value is the exact parameter
    ///   name.
    /// * When the parameter is a list (vector, array, etc.), the invalid items
    ///   can be __optionally__ indicated using square brackets (e.g. `l[3,5,7]`).
    /// * when the parameter is struct, the invalid fields or method return
    ///   return values can be __optionally__ indicated using curly brackets
    ///   (e.g invalid field: `person{name}`, invalid method return value:
    ///   `person{full_name()}`, invalid fields/methods:
    ///   `employee{name, position()}`).
    /// * When several parameters are invalid, its value is the parameters names
    ///   wrapped in round brackets (e.g. `(p1,p3)`); it also accepts any of the
    ///   above combination of parameters types
    ///   (e.g. `(p1,l[2,10],person{name})`).
    /// * When all the function parameters are invalid, `<all>` is used.
    ///
    /// For enforcing the conventions across your code base use the
    /// [`Error::new_invalid_arguments`] constructor function.
    pub names: String,
    /// `msg` is a human friendly message that explains why the argument(s) are
    /// invalid.
    pub msg: String,
}

impl Args {
    // TODO: this constructor must enforce the names convention commented in the
    // documentation of this type and panic if they are violated because that
    // means that there is a bug in the code that uses it.
    pub(crate) fn new(names: &str, msg: &str) -> Self {
        Args {
            names: String::from(names),
            msg: String::from(msg),
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // TODO: format the message depending if the arguments are the whole
        // argument, structs fields, lists, etc.
        write!(
            f,
            "{} arguments have invalid values. {}",
            self.names, self.msg
        )
    }
}

#[derive(Debug)]
/// An unexpected error which happens due to the state of the system where the
/// application is running; for example, insufficient resources, OS failure,
/// hardware failure, etc.
pub struct Internal {
    /// A human friendly message to provide context of the error.
    pub ctx_msg: String,
    /// The received error which cannot be handled by the application and get
    /// wrapped by this instance.
    pub(crate) error: BoxError,
}

impl stderr::Error for Internal {
    fn source(&self) -> Option<&(dyn stderr::Error + 'static)> {
        Some(self.error.as_ref())
    }
}

impl fmt::Display for Internal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ctx_msg)
    }
}

/// An error caused by one of the external systems which the application rely on.
#[derive(Debug)]
pub struct External {
    pub system: ExternalSystem,
    pub(crate) inner: BoxError,
}

impl stderr::Error for External {
    fn source(&self) -> Option<&(dyn stderr::Error + 'static)> {
        Some(self.inner.as_ref())
    }
}

impl fmt::Display for External {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Extenral error produced by the {} system", self.system,)
    }
}

/// Indicates the external system that has reported the error.
#[derive(Debug, PartialEq)]
pub enum ExternalSystem {
    /// Ethereum error.
    Ethereum,
    /// IPFS error.
    IPFS,
}

impl fmt::Display for ExternalSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ExternalSystem::Ethereum => write!(f, "Ethereum"),
            ExternalSystem::IPFS => write!(f, "IPFS"),
        }
    }
}
