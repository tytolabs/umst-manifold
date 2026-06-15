#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/rank_drop_fail.rs");
    t.pass("tests/ui/rank_preserve_pass.rs");
}
