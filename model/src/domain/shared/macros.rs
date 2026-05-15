#[macro_export]
macro_rules! impl_uuid_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub uuid::Uuid);

        impl std::ops::Deref for $name {
            type Target = uuid::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
