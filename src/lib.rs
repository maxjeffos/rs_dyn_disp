pub mod byref;
pub mod byval;

pub trait Thing {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
}
