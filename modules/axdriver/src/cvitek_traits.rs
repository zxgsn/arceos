use axalloc::global_allocator;
use axhal::mem::{phys_to_virt, virt_to_phys};

use driver_net::cvitek::CvitekNicTraits;

pub struct CvitekNicTraitsImpl;

impl CvitekNicTraits for CvitekNicTraitsImpl {
    fn phys_to_virt(pa: usize) -> usize {
        let va = phys_to_virt(pa.into()).as_usize();
        va
    }
    fn virt_to_phys(va: usize) -> usize {
        let pa = virt_to_phys(va.into()).as_usize();
        pa
    }
}