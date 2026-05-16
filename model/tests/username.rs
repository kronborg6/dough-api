use model::domain::user::username::Username;

#[test]
fn vailed_username() {
    assert!(Username::new("Kronborg").is_ok());
}

#[test]
fn reject_short_username() {
    assert!(Username::new("Kro").is_err());
}
