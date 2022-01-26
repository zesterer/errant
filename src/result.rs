use super::*;
use std::process::Termination;

#[derive(Copy, Clone)]
pub enum TrackedResult<T, E> {
    Ok(T),
    Err(E),
}

#[track_caller]
pub fn err<T, E: Error>(payload: E::Payload) -> TrackedResult<T, E> {
    TrackedResult::Err(E::create(payload, Location::caller()))
}

impl<T, E: Error> Try for TrackedResult<T, E> {
    type Output = T;
    type Residual = TrackedResult<!, E>;

    fn from_output(output: Self::Output) -> Self {
        TrackedResult::Ok(output)
    }

    #[track_caller]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(x) => ControlFlow::Continue(x),
            Self::Err(mut e) => {
                e.handle_try(Location::caller());
                ControlFlow::Break(TrackedResult::Err(e))
            },
        }
    }
}

impl<T, E> FromResidual<TrackedResult<!, E>> for TrackedResult<T, E> {
    fn from_residual(res: TrackedResult<!, E>) -> Self {
        match res {
            TrackedResult::Ok(x) => x,
            TrackedResult::Err(e) => Self::Err(e),
        }
    }
}

impl<T: fmt::Debug, E: fmt::Debug> fmt::Debug for TrackedResult<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ok(x) => write!(f, "Ok({:?})", x),
            Self::Err(e) => write!(f, "{:?}", e),
        }
    }
}

impl<E: Termination> Termination for TrackedResult<(), E> {
    fn report(self) -> i32 {
        match self {
            Self::Ok(()) => 0,
            Self::Err(e) => e.report(),
        }
    }
}

impl<E: Termination> Termination for TrackedResult<!, E> {
    fn report(self) -> i32 {
        match self {
            Self::Ok(x) => x,
            Self::Err(e) => e.report(),
        }
    }
}
