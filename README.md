# Partial Context

Basic traits for deterministic partial / context variants

## Usage

```rust
use partial_context::PartialContext;

#[derive(PartialContext, PartialEq, Eq, Debug, Clone)]
#[context_needs(
    #[derive(Clone)]
)]
pub struct Test {
    pub abc: i64,
    // mark the field (s) as optional with an unsized type marker
    #[context]
    pub c: usize,
}

fn something_that_uses_abc<T: PartialContext<PartialTest, Test>>(test: T) -> i64 {
    test.partial().abc
}

fn abstracted() {
    let parts = PartialTest::new(1);
    let whole = Test { abc: 1, c: 2 };

    assert_eq!(something_that_uses_abc(parts.clone()), 1);
    assert_eq!(something_that_uses_abc(whole.clone()), 1);

    let fin = parts.with_context(2);
    assert_eq!(whole, fin);
}
```
