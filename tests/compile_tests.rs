#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    #[cfg(feature = "std")]
    t.pass("tests/run-pass/*.rs");
    #[cfg(feature = "std")]
    t.compile_fail("tests/compile-fail/*.rs");
}
