// === PageFrameNumber === //

#[derive(Clone, Copy, Debug)]
pub struct PageFrameNumber(usize);

impl Into<PhysicalAddress> for PageFrameNumber {
    fn into(self) -> PhysicalAddress {
        PhysicalAddress(self.0 << 12)
    }
}

impl crate::traits::Convertable<usize> for PageFrameNumber {
    fn to(self) -> usize {
        self.0
    }
}

impl crate::traits::Convertable<PageFrameNumber> for usize {
    fn to(self) -> PageFrameNumber {
        PageFrameNumber(self)
    }
}

// === PhysicalAddress === //

#[derive(Clone, Copy, Debug)]
pub struct PhysicalAddress(usize);

impl Into<PageFrameNumber> for PhysicalAddress {
    fn into(self) -> PageFrameNumber {
        PageFrameNumber(self.0 >> 12)
    }
}

impl crate::traits::Convertable<usize> for PhysicalAddress {
    fn to(self) -> usize {
        self.0
    }
}

impl crate::traits::Convertable<PhysicalAddress> for usize {
    fn to(self) -> PhysicalAddress {
        PhysicalAddress(self)
    }
}

// === VirtualAddress === //

#[derive(Clone, Copy, Debug)]
pub struct VirtualAddress(usize);

impl crate::traits::Convertable<usize> for VirtualAddress {
    fn to(self) -> usize {
        self.0
    }
}

impl crate::traits::Convertable<VirtualAddress> for usize {
    fn to(self) -> VirtualAddress {
        VirtualAddress(self)
    }
}
