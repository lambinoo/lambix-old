#[macro_export]
macro_rules! per_cpu {
    ($(static $name:ident: $type:ty = $expr:expr;),*) => {
        $(
            #[link_section = "per_cpu_template"]
            static $name : $type = $expr;
        )*
    }
}

