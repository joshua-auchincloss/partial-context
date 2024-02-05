#[cfg(feature = "derive")]
pub use partial_context_codegen::PartialContext;

pub trait PartialContext<L, R>
where
    L: From<R>,
{
    fn has_context(&self) -> bool;
    fn needs_context(&self) -> bool {
        !self.has_context()
    }

    /// infallible since L is from R we can always fall back to L
    fn partial(self) -> L;

    /// can panic if R does not have context
    fn unwrap_context(self) -> R;

    fn context(self) -> Option<R>;
}
