// Copyright (c) 2025 Syswonder
// hvisor is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR
// FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
//
// Syswonder Website:
//      https://www.syswonder.org
//
// Authors:
//
use core::{ptr, usize};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::{error::HvResult, percpu::this_zone, zone::zone_error};

use super::GuestPhysAddr;

pub type MMIOHandler = fn(&mut MMIOAccess, usize) -> HvResult;

#[derive(Copy, Clone, Debug)]
pub struct MMIOAccess {
    /** Address to access, depending on the context, an absolute address or
     * relative offset to region start. */
    pub address: GuestPhysAddr,
    /** Size of the access. */
    pub size: usize,
    /** True if write access. */
    pub is_write: bool,
    /** The value to be written or the read value to return. */
    pub value: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct MMIORegion {
    pub start: GuestPhysAddr,
    pub size: usize,
}

#[derive(Debug)]
pub struct MMIOConfig {
    pub region: MMIORegion,
    pub handler: MMIOHandler,
    pub arg: usize,
}

impl MMIORegion {
    pub fn contains_region(&self, addr: GuestPhysAddr, sz: usize) -> bool {
        addr >= self.start && addr + (sz as usize) <= self.start + (self.size as usize)
    }
}

pub fn mmio_perform_access(base: usize, mmio: &mut MMIOAccess) {
    let addr = base as usize + mmio.address;

    unsafe {
        if mmio.is_write {
            match mmio.size {
                1 => ptr::write_volatile(addr as *mut u8, mmio.value as u8),
                2 => ptr::write_volatile(addr as *mut u16, mmio.value as u16),
                4 => ptr::write_volatile(addr as *mut u32, mmio.value as u32),
                8 => ptr::write_volatile(addr as *mut u64, mmio.value as u64),
                _ => {
                    zone_error!("invalid mmio size: {}", mmio.size);
                }
            }
        } else {
            mmio.value = match mmio.size {
                1 => ptr::read_volatile(addr as *mut u8) as _,
                2 => ptr::read_volatile(addr as *mut u16) as _,
                4 => ptr::read_volatile(addr as *mut u32) as _,
                8 => ptr::read_volatile(addr as *mut u64) as _,
                _ => {
                    zone_error!("invalid mmio size: {}", mmio.size);
                    usize::MAX
                }
            }
        }
    }
}

pub fn mmio_handle_access(mmio: &mut MMIOAccess) -> HvResult {
    let zone = this_zone();
    let res = zone.read().find_mmio_region(mmio.address, mmio.size);
    let zone_id = zone.read().id;
    drop(zone);
    match res {
        Some((region, handler, arg)) => {
            mmio.address -= region.start;
            match handler(mmio, arg) {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("mmio handler returned error: {:#x?}", e);
                    Err(e)
                }
            }
        }
        None => {
            // record the miss into a small ring buffer for aggregated inspection
            record_mmio_miss(zone_id, mmio);
            // single-line warning to keep logs compact
            warn!(
                "Zone {} unhandled mmio addr={:#x} size={} write={} value={:#x}",
                zone_id,
                mmio.address,
                mmio.size,
                mmio.is_write,
                mmio.value
            );
            maybe_dump_mmio_misses();
            hv_result_err!(EINVAL)
        }
    }
}

// Small ring buffer to keep recent unhandled MMIOs so we can dump an aggregated
// summary instead of a flood of single-line warnings.
const MMIO_MISS_CAP: usize = 128;
static MMIO_MISS_IDX: AtomicUsize = AtomicUsize::new(0);
static mut MMIO_MISSES: [(usize, usize, u8, usize, usize); MMIO_MISS_CAP] = [(0, 0, 0, 0, 0); MMIO_MISS_CAP];

fn record_mmio_miss(zone: usize, mmio: &MMIOAccess) {
    let idx = MMIO_MISS_IDX.fetch_add(1, Ordering::Relaxed);
    let slot = idx % MMIO_MISS_CAP;
    unsafe {
        MMIO_MISSES[slot] = (
            mmio.address as usize,
            mmio.size as usize,
            if mmio.is_write { 1 } else { 0 },
            mmio.value as usize,
            zone as usize,
        );
    }
}

fn maybe_dump_mmio_misses() {
    let idx = MMIO_MISS_IDX.load(Ordering::Relaxed);
    // every 32 misses, print an aggregated summary of recent entries
    if idx > 0 && (idx % 32 == 0) {
        warn!("--- MMIO miss summary (last {} entries) ---", MMIO_MISS_CAP);
        unsafe {
            for i in 0..MMIO_MISS_CAP {
                let (addr, size, is_write, value, zone) = MMIO_MISSES[i];
                if addr != 0 {
                    warn!(
                        "idx={} zone={} addr={:#x} size={} write={} value={:#x}",
                        i,
                        zone,
                        addr,
                        size,
                        is_write != 0,
                        value
                    );
                }
            }
        }
        warn!("--- end mmio miss summary ---");
    }
}

#[allow(dead_code)]
pub fn mmio_generic_handler(mmio: &mut MMIOAccess, base: usize) -> HvResult {
    mmio_perform_access(base, mmio);
    Ok(())
}
