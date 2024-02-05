#[cfg(feature = "derive")]
pub use partial_context_codegen::PartialContext;

/// A trait which offers type-based variants. Type `L` is considered the `Partial` variant of `R`. Type `R` is implied by the presence of type `L`, allowing for context-based type-guards around null values at runtime.
///
///
///
/// ```
/// use partial_context::PartialContext;
///
/// #[derive(PartialContext, PartialEq, Debug)]
/// struct MyType {
///     some_field: int64,
///     #[context]
///     some_other: bool
/// }
///
/// fn needs_context<Type: PartialContext<PartialMyType, MyType>>(parts: Type) -> int64 {
///     parts.partial().some_field
/// }
///
/// fn main() {
///     // create a new partial with the non-context based values
///     let parts = PartialMyType::new(1);
///
///     let built = parts.with_context(true);
///
///     assert_eq!(built, MyType { some_field: 1, some_other: true })
/// }
/// ```
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
