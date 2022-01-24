#![feature(never_type, try_trait_v2, termination_trait_lib)]

pub mod result;
pub mod error;
pub mod backtrace;

pub mod prelude {
    pub use super::result::{
        TrackedResult::Ok,
        err as Err,
    };

    pub type Result<T, E> = super::result::TrackedResult<T, super::Backtrace<E>>;
}

pub use core::{
    panic::Location,
    fmt,
    ops::{Try, ControlFlow, FromResidual, Range},
};
use crate::{
    error::Error,
    backtrace::Backtrace,
};
