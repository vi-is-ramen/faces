use crate::types::PageFrameNumber as PFN;

pub trait AbsPageFrameManager<F: crate::traits::AbsFlags> {
    // flags
    fn set_flags(&self, pfn: PFN, flag: F);
    fn clear_flags(&self, pfn: PFN, flag: F);
    fn check_flags(&self, pfn: PFN, flag: F);

    // synchronization
    fn lock(&self, pfn: PFN);
    fn free(&self, pfn: PFN);

    // boundary
    fn min(&self) -> PFN;
    fn max(&self) -> PFN;

    // counters
    fn inc0(&self, pfn: PFN); fn inc1(&self, pfn: PFN); fn inc2(&self, pfn: PFN); fn inc3(&self, pfn: PFN);
    fn dec0(&self, pfn: PFN); fn dec1(&self, pfn: PFN); fn dec2(&self, pfn: PFN); fn dec3(&self, pfn: PFN);
    fn get0(&self, pfn: PFN); fn get1(&self, pfn: PFN); fn get2(&self, pfn: PFN); fn get3(&self, pfn: PFN);

    // checks if PFN is controlled by this PFM
    fn present(&self, pfn: PFN) -> bool;

    // raw
    unsafe fn get_ptr(&self, pfn: PFN) -> *const ();
    unsafe fn get_mut(&self, pfn: PFN) -> *mut ();
}
