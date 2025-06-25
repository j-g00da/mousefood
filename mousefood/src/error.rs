//! Mousefood `Error` enum.

/// Represents backend error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Drawing to the display failed.
    #[error("drawing to DrawTarget failed")]
    DrawError,

    /// Selected [`ClearType`](ratatui_core::backend::ClearType) is not supported by Mousefood.
    #[error("ClearType::{0} is not supported by Mousefood")]
    ClearTypeUnsupported(alloc::string::String),
}
