#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(pub String);

impl Email {
    pub fn new(name: impl Into<String>) -> Result<Self, String> {
        let name = name.into();
        if name.matches('@').count() != 1
            || !name.contains('.')
            || name.contains("..")
            || name.len() <= 4
            || name.contains(' ')
        {
            return Err(String::from("email is invaled"));
        }

        let parts: Vec<&str> = name.split('@').collect();

        if parts.len() != 2 || parts[0].is_empty() || parts[1].matches('.').count() != 1 {
            return Err(String::from("email is invaled 2"));
        }

        Ok(Self(name))
    }
}
