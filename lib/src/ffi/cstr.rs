use core::str::{self, Utf8Error};
use cty::c_char;

#[repr(transparent)]
pub struct CStr {
    inner: [u8],
}

impl CStr {
    pub unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &CStr {
        &*(bytes as *const [u8] as *const CStr)
    }

    pub unsafe fn from_ptr<'a>(ptr: *const c_char) -> &'a CStr {
        let ptr = ptr as *const u8;
        let len = c_strlen(ptr);
        CStr::from_bytes_with_nul_unchecked(core::slice::from_raw_parts(ptr, len + 1))
    }

    pub unsafe fn as_bytes(&self) -> &[u8] {
        &self.inner[..self.inner.len() - 1]
    }

    pub unsafe fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }
}

unsafe fn c_strlen(ptr: *const u8) -> usize {
    let mut computed_size = 0;
    while *ptr.wrapping_add(computed_size) != b'\0' {
        computed_size += 1;
    }
    computed_size
}
