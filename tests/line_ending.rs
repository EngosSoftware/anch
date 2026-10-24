use anch::LineEnding;

#[test]
fn getting_first_char_should_work() {
    assert_eq!('\n', LineEnding::Lf.first());
    assert_eq!('\r', LineEnding::Cr.first());
    assert_eq!('\r', LineEnding::CrLf.first());
}

#[test]
fn getting_last_char_should_work() {
    assert_eq!('\n', LineEnding::Lf.last());
    assert_eq!('\r', LineEnding::Cr.last());
    assert_eq!('\n', LineEnding::CrLf.last());
}

#[test]
fn getting_reference_should_work() {
    assert_eq!("\n", LineEnding::Lf.as_ref());
    assert_eq!("\r", LineEnding::Cr.as_ref());
    assert_eq!("\r\n", LineEnding::CrLf.as_ref());
}
