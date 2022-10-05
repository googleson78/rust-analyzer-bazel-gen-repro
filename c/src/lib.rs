pub fn function() -> i32 {
    return 1;
}

#[test]
fn bla() {
    assert_eq!(a::function(), 2);
}
