use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::{collections::VecDeque, vec::Vec};
use core::marker::PhantomData;
use core::ptr::NonNull;
use core::time::Duration;
use core::{mem, ptr};

use super::cvitek_defs::*;

pub struct CvitekNicDevice {
    iobase: usize,
    // rx_rings: Vec<RxRing>,
    // tx_rings: Vec<TxRing>,
}

impl CvitekNicDevice {
    pub fn new(iobase: usize) -> Self {
        let mut nic = CvitekNicDevice {
            iobase,
            // rx_rings: Vec::new(),
            // tx_rings: Vec::new(),
        };
        nic.init();
        nic
    }

    pub fn init(&mut self) {
        // alloc rx_ring and tx_ring
        // for _ in 0..NUM_RX_QUEUE {
        //     self.rx_rings.push(RxRing::new());
        // }

        // for _ in 0..NUM_TX_QUEUE {
        //     self.tx_rings.push(TxRing::new());
        // }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct RxDes {
    pub rdes0: u32,
    pub rdes1: u32,
    pub rdes2: u32,
    pub rdes3: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct TxDes {
    pub tdes0: u32,
    pub tdes1: u32,
    pub tdes2: u32,
    pub tdes3: u32,
}

pub struct RxRing {
    pub iobase: usize,
    pub rd: Dma<RxDes>,
}

impl RxRing {
    pub fn new() -> Self {
        todo!()
        // let size = mem::size_of::<RxDes>() * 512;
        // let pages = (size + 0x1000 - 1) / 0x1000;

        // let vaddr = if let Ok(vaddr) = global_allocator().alloc_pages(pages, 0x1000) {
        //     vaddr
        // } else {
        //     panic!("RxRing alloc_pages failed");
        // };

        // let paddr = virt_to_phys(vaddr.into());

        // let mut phy_addr = paddr as usize;
        // let rd = Dma::new(vaddr as _, paddr, 512);

        // Self {
        //     iobase: 0,
        //     rd: rd
        // }
    }

    /// Release the next RDes to the DMA engine
    pub fn set_idx_addr_owner(&mut self, idx: usize, buffer_phy_addr: usize) {
        let mut rd = RxDes {
            rdes0: 0,
            rdes1: 0,
            rdes2: 0,
            rdes3: 0,
        };

        // dwmac_desc_set_addr
        rd.rdes0 = buffer_phy_addr as u32;
        rd.rdes1 = ((buffer_phy_addr >> 32) & 0xFF) as u32;

        // dwmac_set_rx_owner
        rd.rdes3 |= RDES3_OWN | RDES3_BUFFER1_VALID_ADDR;
        rd.rdes3 |= RDES3_INT_ON_COMPLETION_EN;

        self.rd.write_volatile(idx, &rd);
    }

    pub fn set_head_tail_ptr(&mut self) {
        // unsafe {
        //     let rd_addr = self.rd.phy_addr as usize;

        //     writel(
        //         self.iobase + DMA_CHAN_RX_BASE_ADDR_HI,
        //         (rd_addr >> 32) as u32,
        //     );
        //     writel(
        //         self.iobase + DMA_CHAN_RX_BASE_ADDR,
        //         (rd_addr & 0xFFFFFFFF) as u32,
        //     );

        //     writel(
        //         self.iobase + DMA_CHAN_RX_END_ADDR,
        //         rd_addr as u32 + (511 * core::mem::size_of::<RxDes>()) as u32,
        //     );
        // }
    }
}

pub struct TxRing {
    pub td: Dma<TxDes>,
}

impl TxRing {
    pub fn new() -> Self {
        todo!()
        // let size = mem::size_of::<RxDes>() * 512;
        // let pages = (size + 0x1000 - 1) / 0x1000;

        // let vaddr = if let Ok(vaddr) = global_allocator().alloc_pages(pages, 0x1000) {
        //     vaddr
        // } else {
        //     panic!("RxRing alloc_pages failed");
        // };

        // let paddr = virt_to_phys(vaddr.into());

        // let mut phy_addr = paddr as usize;

        // let td = Dma::new(vaddr as _, paddr, 512);

        // Self { td: td }
    }
}

pub struct Dma<T> {
    pub count: usize,
    pub phy_addr: usize,
    pub cpu_addr: *mut T,
}

impl<T> Dma<T> {
    pub fn new(cpu_addr: *mut T, phy_addr: usize, count: usize) -> Self {
        Self {
            count: count,
            phy_addr: phy_addr,
            cpu_addr: cpu_addr,
        }
    }

    pub fn read_volatile(&self, index: usize) -> Option<T> {
        if index >= self.count {
            // pr_info!("read_volatile index:{:?} count:{:?}", index, self.count);
            return None;
        }

        let ptr = self.cpu_addr.wrapping_add(index);

        // SAFETY: We just checked that the index is within bounds.
        Some(unsafe { ptr.read() })
    }

    pub fn write_volatile(&self, index: usize, value: &T) -> bool
    where
        T: Copy,
    {
        if index >= self.count {
            // pr_info!("read_volatile index:{:?} count:{:?}", index, self.count);
            return false;
        }

        let ptr = self.cpu_addr.wrapping_add(index);

        // SAFETY: We just checked that the index is within bounds.
        unsafe { ptr.write(*value) };
        true
    }
}