use super::*;

pub trait Error: fmt::Debug {
    type Payload;

    fn create(payload: Self::Payload, loc: &'static Location) -> Self where Self: Sized;
    fn handle_try(&mut self, loc: &'static Location) {}
}
