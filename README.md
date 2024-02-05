# Partial Context

[![tests](https://img.shields.io/github/actions/workflow/status/joshua-auchincloss/partial-context/test.yaml?label=Tests)](https://github.com/joshua-auchincloss/partial-context)
[![latest](https://img.shields.io/crates/v/partial-context)](https://crates.io/crates/partial-context)
[![downloads](https://img.shields.io/crates/dr/partial-context)](https://crates.io/crates/partial-context)

Basic traits for deterministic partial / context variants

## Usage

In your `cargo.toml`

```toml
[dependencies]
partial-context = "0.1"
```

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
