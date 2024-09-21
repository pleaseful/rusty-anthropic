#[macro_export]
macro_rules! setters {
    ($(
        $(#[$setter_attributes:meta])*
        $setter_ident:ident: $setter_type:ty,
    )*) => {
        $(
            $(#[$setter_attributes])*
            #[inline(always)]
            pub fn $setter_ident(mut self, $setter_ident: $setter_type) -> Self {
                self.$setter_ident = Some($setter_ident);
                self
            }
        )*
    };
}
