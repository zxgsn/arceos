use axalloc::global_allocator;
use axhal::mem::{phys_to_virt, virt_to_phys};
use axhal::time::{busy_wait, Duration};


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
    fn dma_alloc_pages(pages: usize) -> (usize, usize) {
        let vaddr = if let Ok(vaddr) = global_allocator().alloc_pages(pages, 0x1000) {
            vaddr
        } else {
            panic!("RxRing alloc_pages failed");
        };
        let paddr = virt_to_phys(vaddr.into()).as_usize();

        // info!("dma_alloc_pages vaddr:{:x} paddr:{:x}", vaddr, paddr);
        (vaddr, paddr)
    }

    fn dma_free_pages(vaddr: usize, pages: usize) {
        global_allocator().dealloc_pages(vaddr, pages);
    }

    fn mdelay(_m_times:usize)
    {
        busy_wait(Duration::from_millis(_m_times.try_into().unwrap()));
    }
}