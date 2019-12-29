use core::ops::*;
use core::mem::align_of;

macro_rules! op {
    ($name:ident, $tr:ident, $trass:ident, $fn_name:ident, $fnass_name:ident) => {
        impl $tr<usize> for $name {
            type Output = Self;
            fn $fn_name(self, rhs: usize) -> Self::Output {
                let a = (self.0 as usize).$fn_name(rhs);
                Self(a as _)
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
                $name(self.0.wrapping_add(value))
            }

            pub fn align<T>(&self) -> $name {
                self.wrapping_add(self.0.align_offset(align_of::<T>()))
            }

            pub fn align_to(&self, value: usize) -> $name {
                self.wrapping_add(self.0.align_offset(value))
            }

            pub fn is_null(&self) -> bool {
                Self::NULL.0 == self.0
            }

            pub fn is_aligned(&self, align: usize) -> bool {
                let addr = usize::from(*self);
                (addr & (align - 1)) == 0
            }

            pub fn as_ptr(self) -> *const u8 {
                self.0
            }

            pub fn as_mut(self) -> *mut u8 {
                self.0 as _
            }
        }

        impl<T> From<&T> for $name {
            fn from(addr: &T) -> $name {
                $name(addr as *const T as _)
            }
        }
  
        impl From<usize> for $name {
            fn from(addr: usize) -> $name {
                $name(addr as _)
            }
        }

        impl From<$name> for usize {
            fn from(addr: $name) -> usize {
                addr.0 as usize
            }
        }

        impl $name {
            pub fn distance(&self, end: $name) -> usize {
                let end = usize::from(end);
                let start = usize::from(*self);

                if end > start {
                    end.wrapping_sub(start)
                } else {
                    0
                }
            }

            pub unsafe fn to_ref<'a, T>(self) -> &'a T {
                &*(self.0 as *mut T)
            }

            pub unsafe fn to_ref_mut<'a, T>(self) -> &'a mut T {
                &mut *(self.0 as *mut T)
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

