macro_rules! export (
    ($type:ty, $($name:ident = $value:expr;)*) => {
        $(
            #[no_mangle]
            #[link_section = ".data.config"]
            pub static $name: $type = $value;
        )*
    }
);

pub static _1TB: usize = 1 << 40;
pub static _512GB: usize = _1TB / 2;
pub static _1GB: usize = 1 << 30;
pub static TOP: usize = !0;
