use core::ops::*;
use core::mem::align_of;
use core::ptr::NonNull;

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
        pub struct $name(pub usize);

        impl $name {
            pub const fn new(value: usize) -> Self {
                Self(value)
            }

            pub const fn null() -> $name {
                Self::new(0)
            }

            pub const fn is_null(&self) -> bool {
                self.0 == 0
            }

            pub const fn as_ptr<T>(self) -> *const T {
                self.0 as _
            }

            pub const fn as_mut_ptr<T>(self) -> *mut T {
                self.0 as _
            }
        }

        impl<T> From<&T> for $name {
            fn from(addr: &T) -> $name {
                Self::from(addr as *const T)
            }
        }

        impl<T> From<&mut T> for $name {
            fn from(addr: &mut T) -> $name {
                Self::from(addr as *mut T)
            }
        }

        impl<T> From<*mut T> for $name {
            fn from(addr: *mut T) -> $name {
                Self(addr as usize)
            }
        }
        
        impl<T> From<*const T> for $name {
            fn from(addr: *const T) -> $name {
                Self(addr as usize)
            }
        }

        impl<T> From<NonNull<T>> for $name {
            fn from(ptr: NonNull<T>) -> Self {
                Self::from(ptr.as_ptr())
            }
        }

        impl From<usize> for $name {
            fn from(addr: usize) -> Self {
                Self(addr)
            }
        }

        impl From<$name> for usize {
            fn from(addr: $name) -> usize {
                addr.0
            }
        }

        impl $name {
            pub fn distance(&self, end: PhyAddr) -> usize {
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


            pub fn is_aligned(&self, align: usize) -> bool {
                let addr = usize::from(*self);
                (addr & (align - 1)) == 0
            }

            pub fn wrapping_add(&self, value: usize) -> Self {
                $name(self.0.wrapping_add(value))
            }

            pub fn wrapping_sub(&self, value: usize) -> Self {
                $name(self.0.wrapping_sub(value))
            }

            pub fn align<T>(&self) -> $name {
                Self::from(self.wrapping_add(
                    self.as_ptr::<u8>().align_offset(align_of::<T>())
                ))
            }

            pub fn align_to(&self, value: usize) -> $name {
                self.wrapping_add(self.as_ptr::<u8>().align_offset(value))
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

