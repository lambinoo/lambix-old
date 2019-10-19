use core::ops::*;

macro_rules! op {
    ($name:ident, $tr:ident, $trass:ident, $fn_name:ident, $fnass_name:ident) => {
        impl $tr<u64> for $name {
            type Output = Self;
            fn $fn_name(self, rhs: u64) -> Self::Output {
                let a = (self.0 as u64).$fn_name(rhs);
                Self(a as _)
            }
        }

        impl $tr<usize> for $name {
            type Output = Self;
            fn $fn_name(self, rhs: usize) -> Self::Output {
                let a = (self.0 as usize).$fn_name(rhs);
                Self(a as _)
            }
        }

        impl $trass<u64> for $name {
            fn $fnass_name(&mut self, rhs: u64) {
                let result = (*self).$fn_name(rhs);
                self.0 = result.0 as _;
            }
        }

        impl $trass<usize> for $name {
            fn $fnass_name(&mut self, rhs: usize) {
                let result = (*self).$fn_name(rhs);
                self.0 = result.0 as _;
            }
        }

    }
}

macro_rules! address {
    ($name:ident; $type:ty) => {
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(transparent)]
        pub struct $name(pub *const $type);

        impl $name {
            pub const NULL: $name = $name(0 as _);

            pub fn wrapping_add(&self, value: usize) -> Self {
                Self(self.0.wrapping_add(value))
            }
        }

        impl Deref for $name {
            type Target = *const $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<u64> for $name {
            fn from(addr: u64) -> $name {
                $name(addr as _)
            }
        }

        impl From<usize> for $name {
            fn from(addr: usize) -> $name {
                $name(addr as _)
            }
        }

        impl From<$name> for u64 {
            fn from(addr: $name) -> u64 {
                addr.0 as _
            }
        }

        impl From<$name> for usize {
            fn from(addr: $name) -> usize {
                addr.0 as usize
            }
        }

        op!($name, BitOr, BitOrAssign, bitor, bitor_assign);
        op!($name, BitAnd, BitAndAssign, bitand, bitand_assign);
        op!($name, Shl, ShlAssign, shl, shl_assign);
        op!($name, Shr, ShrAssign, shr, shr_assign);
    }
}

address!(PhyAddr; u8);
address!(VirtAddr; u8);

