use miette::Diagnostic;
use std::{error::Error, fmt::Display};

#[derive(Debug, Diagnostic)]
pub enum SetupError {
    #[diagnostic(code(didweb::config))]
    Config {
        #[source_code]
        src: String,
        #[label("config error")]
        err_span: (usize, usize),
        msg: String,
    },

    #[diagnostic(code(didweb::domain))]
    Domain {
        #[source_code]
        src: String,
        #[label("domain validation failed")]
        err_span: (usize, usize),
        msg: String,
    },

    #[diagnostic(code(didweb::document))]
    Document {
        #[source_code]
        src: String,
        #[label("did document error")]
        err_span: (usize, usize),
        msg: String,
    },

    #[diagnostic(code(didweb::fs))]
    FileSystem {
        #[source_code]
        src: String,
        #[label("filesystem error")]
        err_span: (usize, usize),
        msg: String,
    },

    #[diagnostic(code(didweb::input))]
    Input {
        #[source_code]
        src: String,
        #[label("input error")]
        err_span: (usize, usize),
        msg: String,
    },

    #[diagnostic(code(didweb::url))]
    Url {
        #[source_code]
        src: String,
        #[label("URL parse error")]
        err_span: (usize, usize),
        msg: String,
    },
}

pub type SetupResult<T> = miette::Result<T>;

impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::Config { msg, .. } => write!(f, "Configuration error: {}", msg),
            SetupError::Domain { msg, .. } => write!(f, "Domain error: {}", msg),
            SetupError::Document { msg, .. } => write!(f, "DID document error: {}", msg),
            SetupError::FileSystem { msg, .. } => write!(f, "Filesystem error: {}", msg),
            SetupError::Input { msg, .. } => write!(f, "Input error: {}", msg),
            SetupError::Url { msg, .. } => write!(f, "URL parse error: {}", msg),
        }
    }
}

impl Error for SetupError {}

impl SetupError {
    pub fn config(msg: impl Into<String>, src: impl AsRef<str>) -> Self {
        let src = src.as_ref().to_string();
        SetupError::Config {
            msg: msg.into(),
            src: src.clone(),
            err_span: (0, src.len()),
        }
    }

    pub fn domain(msg: impl Into<String>, src: impl AsRef<str>) -> Self {
        let src = src.as_ref().to_string();
        SetupError::Domain {
            msg: msg.into(),
            src: src.clone(),
            err_span: (0, src.len()),
        }
    }

    pub fn document(msg: impl Into<String>, src: impl AsRef<str>) -> Self {
        let src = src.as_ref().to_string();
        SetupError::Document {
            msg: msg.into(),
            src: src.clone(),
            err_span: (0, src.len()),
        }
    }

    pub fn fs(msg: impl Into<String>, src: impl AsRef<str>) -> Self {
        let src = src.as_ref().to_string();
        SetupError::FileSystem {
            msg: msg.into(),
            src: src.clone(),
            err_span: (0, src.len()),
        }
    }

    pub fn input(msg: impl Into<String>, src: impl AsRef<str>) -> Self {
        let src = src.as_ref().to_string();
        SetupError::Input {
            msg: msg.into(),
            src: src.clone(),
            err_span: (0, src.len()),
        }
    }

    pub fn url(msg: impl Into<String>, src: impl AsRef<str>) -> Self {
        let src = src.as_ref().to_string();
        SetupError::Url {
            msg: msg.into(),
            src: src.clone(),
            err_span: (0, src.len()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SetupError::config("test error", "test source");
        assert_eq!(err.to_string(), "Configuration error: test error");
    }

    #[test]
    fn test_error_construction() {
        let err = SetupError::config("test error", "test source");
        match err {
            SetupError::Config { msg, src, err_span } => {
                assert_eq!(msg, "test error");
                assert_eq!(src, "test source");
                assert_eq!(err_span, (0, "test source".len()));
            }
            _ => panic!("Wrong error variant"),
        }
    }
}
