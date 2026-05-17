use crate::domain::error::visibility::VisibilityError;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityOptions {
    Public = 1,
    Private = 2,
    FriendsOnly = 3,
    Hidden = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Visibility(VisibilityOptions);

impl Default for Visibility {
    fn default() -> Self {
        Self(VisibilityOptions::Public)
    }
}

impl TryFrom<u8> for Visibility {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self(VisibilityOptions::Public)),
            2 => Ok(Self(VisibilityOptions::Private)),
            3 => Ok(Self(VisibilityOptions::FriendsOnly)),
            4 => Ok(Self(VisibilityOptions::Hidden)),
            _ => Err(()),
        }
    }
}

impl Visibility {
    pub fn new(value: VisibilityOptions) -> Self {
        Self(value)
    }

    pub fn change(&mut self, value: VisibilityOptions) -> Result<(), VisibilityError> {
        self.0 = value;
        Ok(())
    }

    pub fn is_public(&self) -> bool {
        matches!(self.0, VisibilityOptions::Public)
    }

    pub fn is_private(&self) -> bool {
        matches!(self.0, VisibilityOptions::Private)
    }

    pub fn is_friends_only(&self) -> bool {
        matches!(self.0, VisibilityOptions::FriendsOnly)
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self.0, VisibilityOptions::Hidden)
    }

    pub fn as_u8(self) -> u8 {
        self.0 as u8
    }
}
