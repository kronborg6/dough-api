use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Username {
    pub fn new(name: impl Into<String>) -> Result<Self, String> {
        let name = name.into();

        if name.len() < 5 {
            return Err(String::from(
                "name is invaled either to small or just write space",
            ));
        }

        Ok(Self(name))
    }
}
