macro_rules! export_const (
    ($name:ident: $type:ty = $value:tt) => {
        #[no_mangle]
        #[link_section = ".data.config"]
        pub static $name: $type = $value;
    }
);

