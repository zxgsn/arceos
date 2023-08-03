use alloc::sync::Arc;
use driver_common::BaseDriverOps;
use driver_common::DevError;
use driver_common::DevResult;

use crate::NetDriverOps;
use alloc::boxed::Box;
use core::any::Any;
use core::any::TypeId;

use core::marker::PhantomData;
use core::ptr;
use core::ptr::{read_volatile, write_volatile};

unsafe impl<A: CvitekNicTraits> Sync for CvitekNic<A> {}
unsafe impl<A: CvitekNicTraits> Send for CvitekNic<A> {}

use super::CvitekNicDevice;

pub struct CvitekNic<A>
where
    A: CvitekNicTraits,
{
    device: CvitekNicDevice,
    phantom: PhantomData<A>,
}

impl <A> CvitekNic<A> 
where
    A: CvitekNicTraits,
{
    pub fn init(iobase: usize) -> Self {
        // bstgmac_select_phy(0, traits_impl);
        info!("-----------------cvitek_nic init-----------------");
        let device = CvitekNicDevice::new(iobase);
        Self {
            device,
            phantom: PhantomData,
        }
    }
}

impl <A:CvitekNicTraits> BaseDriverOps for CvitekNic<A> {
    fn device_name(&self) -> &str {
        todo!()
    }

    fn device_type(&self) -> driver_common::DeviceType {
        driver_common::DeviceType::Net
    }
}

impl<A:CvitekNicTraits> NetDriverOps for CvitekNic<A> {
    fn mac_address(&self) -> crate::EthernetAddress {
        todo!()
    }

    fn tx_queue_size(&self) -> usize {
        1
    }

    fn rx_queue_size(&self) -> usize {
        1
    }

    fn can_receive(&self) -> bool {
        false
    }

    fn can_transmit(&self) -> bool {
        false
    }
/*
    fn fill_rx_buffers(&mut self, buf_pool: &'a crate::NetBufferPool) -> DevResult {
        todo!()
    }

    fn prepare_tx_buffer(&self, tx_buf: &mut crate::NetBuffer, packet_len: usize) -> DevResult {
        todo!()
    }

    fn receive(&mut self) -> DevResult<crate::NetBufferBox<'a>> {
        todo!()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: crate::NetBufferBox<'a>) -> DevResult {
        todo!()
    }

    fn transmit(&mut self, tx_buf: &crate::NetBuffer) -> DevResult {
        todo!()
    }*/

    fn recycle_tx_buffers(&mut self) -> DevResult {
        todo!()
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<crate::NetBufPtr> {
        todo!()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: crate::NetBufPtr) -> DevResult {
        todo!()
    }

    fn transmit(&mut self, tx_buf: crate::NetBufPtr) -> DevResult {
        todo!()
    }

    fn receive(&mut self) -> DevResult<crate::NetBufPtr> {
        todo!()
    }
}

pub fn cvitek_gmac_select_phy<A: CvitekNicTraits>(id: usize, traits: A) {
    let addr = A::phys_to_virt(0x33000000);
    let new_addr = addr + 0x54 >> 2;
    let mut reg_val = unsafe { read_volatile(new_addr as *mut u32) };
    reg_val |= 1 << id;
    unsafe {
        write_volatile(new_addr as *mut u32, reg_val as _);
    }

    let addr = A::phys_to_virt(0x33001008);
    let mut reg_val = unsafe { read_volatile(addr as *mut u32) };

    if id == 0 {
        reg_val &= !((1 << 11) | (1 << 15)); //gmac0: IN00:bit11 IN01:bit12  pps0:bit15
    } else if id == 1 {
        reg_val &= !((1 << 13) | (1 << 16)); //gmac1: IN10:bit13 IN11:bit14  pps1:bit16
    }
    unsafe {
        write_volatile(addr as *mut u32, reg_val as _);
    }
}

pub trait CvitekNicTraits {
    fn phys_to_virt(pa: usize) -> usize {
        pa
    }
    fn virt_to_phys(va: usize) -> usize {
        va
    }
}