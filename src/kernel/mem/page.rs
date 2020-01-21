use alloc::boxed::Box;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

#[repr(align(4096))]
struct InnerPage([u8; Page::PAGE_SIZE]);

pub struct Page {
    page: Box<InnerPage>
}

impl Page {
    pub const PAGE_SIZE: usize = 4096;

    pub fn new() -> Page {
        let page = unsafe { Box::new_zeroed().assume_init() };
        Page { page }
    }

    pub fn new_uninit() -> MaybeUninit<Page> {
        let page = unsafe { Box::new_uninit().assume_init() };
        MaybeUninit::new(Page { page }) 
    }
}

impl Deref for Page {
    type Target = [u8; 4096];
    
    fn deref(&self) -> &Self::Target {
        &self.page.0
    }
}

impl DerefMut for Page {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.page.0
    }
}

