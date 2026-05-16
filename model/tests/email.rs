use model::domain::user::email::Email;

#[test]
fn vailed_email() {
    assert!(Email::new("kron@code.com").is_ok())
}

#[test]
fn reject_minssing_chatroes() {
    assert!(Email::new("kroncode.com").is_err());
    assert!(Email::new("kron@code,com").is_err());
}

#[test]
fn reject_short_email() {
    assert!(Email::new("k@.c").is_err())
}

#[test]
fn reject_email_contain_space() {
    assert!(Email::new("kron @code.com").is_err());
    assert!(Email::new("k on@code.com").is_err());
    assert!(Email::new("kron@code .com").is_err())
}
