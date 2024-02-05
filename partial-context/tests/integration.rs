use partial_context::PartialContext;

#[derive(PartialContext, PartialEq, Eq, Debug, Clone)]
#[context_needs(
    #[derive(Clone)]
)]
pub struct Test {
    pub abc: i64,
    #[context]
    pub def: usize,
}

fn something_that_uses_abc<T: PartialContext<PartialTest, Test>>(test: T) -> i64 {
    test.partial().abc
}

#[test]
fn test_abstracted() {
    let parts = PartialTest::new(1);
    let whole = Test { abc: 1, def: 2 };

    assert_eq!(something_that_uses_abc(parts.clone()), 1);
    assert_eq!(something_that_uses_abc(whole.clone()), 1);

    let fin = parts.with_context(2);
    assert_eq!(whole, fin);
}

#[test]
fn test_needs_context() {
    let t = PartialTest::new(2);
    let y = t.with_context(4);

    assert_eq!(y, Test { abc: 2, def: 4 });
}

#[derive(PartialContext, PartialEq, Debug)]
struct TestMultiple {
    #[context]
    pub abc: i64,
    #[context]
    pub def: usize,

    pub g: bool,
}

#[test]
fn test_always_g() {
    let parts = PartialTestMultiple::new(true);
    let whole = parts.with_context(2, 3);

    assert_eq!(
        whole,
        TestMultiple {
            abc: 2,
            def: 3,
            g: true
        }
    )
}

#[derive(PartialContext, PartialEq, Debug)]
#[partial(name = "PartialCustom")]
struct CustomName {
    pub abc: bool,
    #[context]
    pub def: i64,
}

#[test]
fn test_custom() {
    let base = PartialCustom::new(true);
    let built = base.with_context(2);
    assert_eq!(built, CustomName { abc: true, def: 2 })
}
