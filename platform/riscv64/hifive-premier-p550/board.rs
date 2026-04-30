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
use crate::{arch::zone::HvArchZoneConfig, config::*};

#[allow(unused)]
pub const BOARD_NAME: &str = "hifive-premier-p550";

pub const BOARD_NCPUS: usize = 4;
pub const PLIC_BASE: usize = 0xc000000;
pub const BOARD_PLIC_INTERRUPTS_NUM: usize = 1023; // except irq 0
pub const SIFIVE_CCACHE_BASE: usize = 0x2010000; // SiFive composable cache controller
pub const SIFIVE_CCACHE_SIZE: usize = 0x4000; // 16KB

pub const ROOT_ZONE_DTB_ADDR: u64 = 0x8f000000;
pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x90000000;
pub const ROOT_ZONE_ENTRY: u64 = 0x90000000;
pub const ROOT_ZONE_CPUS: u64 = 0x7;

pub const ROOT_ZONE_NAME: &str = "root-linux";

pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 38] = [
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x80000000,
        virtual_start: 0x80000000,
        size: 0x4_0000_0000, // 修改这里，对应 16GB
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x9400000,
        virtual_start: 0x9400000,
        size: 0x10000,
    }, // ram
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x10030000,
        virtual_start: 0x10030000,
        size: 0x4000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50818000,
        virtual_start: 0x50818000,
        size: 0x4000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50800000,
        virtual_start: 0x50800000,
        size: 0x4000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50804000,
        virtual_start: 0x50804000,
        size: 0x4000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50808000,
        virtual_start: 0x50808000,
        size: 0x4000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x5080c000,
        virtual_start: 0x5080c000,
        size: 0x4000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50900000,
        virtual_start: 0x50900000,
        size: 0x10000,
    }, // serial0
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x51c00000,
        virtual_start: 0x51c00000,
        size: 0x400000,
    }, // npu
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50430000,
        virtual_start: 0x50430000,
        size: 0x10000,
    }, // emmc
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50450000,
        virtual_start: 0x50450000,
        size: 0x10000,
    }, // emmc
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50460000,
        virtual_start: 0x50460000,
        size: 0x10000,
    }, // sdio0 (sd-card)
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50480000,
        virtual_start: 0x50480000,
        size: 0x10000,
    }, // usb0
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50490000,
        virtual_start: 0x50490000,
        size: 0x10000,
    }, // usb1
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x504a0000,
        virtual_start: 0x504a0000,
        size: 0x10000,
    }, // usb0 phy_reg
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x504b0000,
        virtual_start: 0x504b0000,
        size: 0x10000,
    }, // usb1 phy_reg
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50440000,
        virtual_start: 0x50440000,
        size: 0x2000,
    }, // hsp_sp_top_csr
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x51808000,
        virtual_start: 0x51808000,
        size: 0x8000,
    }, 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x518c0000,
        virtual_start: 0x518c0000,
        size: 0x10000,
    }, 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x51828000,
        virtual_start: 0x51828000,
        size: 0x80000,
    }, // sys-crg (clock-controller, reset-controller) (SD card needs)
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x51810000,
        virtual_start: 0x51810000,
        size: 0x8000,
    }, // scu_sys_con (for iommu syscfg)
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50c00000,
        virtual_start: 0x50c00000,
        size: 0x100000,
    }, // iommu (smmu-v3)
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50400000,
        virtual_start: 0x50400000,
        size: 0x10000,
    }, // ethernet0
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50410000,
        virtual_start: 0x50410000,
        size: 0x10000,
    }, // ethernet1
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x51600000,
        virtual_start: 0x51600000,
        size: 0x200000,
    },
    // Cache controller is needed, otherwise terminal will report "VFS: cannot open root device..."
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x2010000,
        virtual_start: 0x2010000,
        size: 0x4000,
    }, // L3 cache-controller, now hvisor has virtual sifive ccache.
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x8000000,
        virtual_start: 0x8000000,
        size: 0x400000,
    }, 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50810000,
        virtual_start: 0x50810000,
        size: 0x4000,
    }, 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x50814000,
        virtual_start: 0x50814000,
        size: 0x4000,
    }, 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x51800000,
        virtual_start: 0x51800000,
        size: 0x8000,
    }, 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x51818000,
        virtual_start: 0x51818000,
        size: 0x1000,
    }, 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x52300000,
        virtual_start: 0x52300000,
        size: 0x40000,
    }, // ddr-controller0
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x52380000,
        virtual_start: 0x52380000,
        size: 0x40000,
    }, // ddr-controller1
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x52100000,
        virtual_start: 0x52100000,
        size: 0x50000,
    }, // d2d
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x5c000000,
        virtual_start: 0x5c000000,
        size: 0x8000,
    }, 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xc0_0000_0000,
        virtual_start: 0xc0_0000_0000,
        // mem-port -> sys-port (here easily equal to mem size), this is needed for uncache access in eic7700x soc.
        size: 0x4_0000_0000,
    }, // Sys-port. (here related to DMA)
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x800000000,
        virtual_start: 0x800000000,
        size: 0x100000000, // high alias window must be executable for guest instruction fetch
    },
];

// Note: all here's irqs are hardware irqs,
//  only these irq can be transferred to the physical PLIC.
pub const HW_IRQS: [u32; 28] = [
    0x39,
    0x4f, // emmc
    0x51, // sd-card
    0x55, // usb0
    0x56, // usb1
    0x64, // uart0
    0x3d, // ethernet0 macirq
    0x46, // ethernet1 macirq
    0x0a, // ddr-controller0 ecc
    0x12b, // ddr-controller1 ecc
    0x11f, // d2d irq0
    0x120, // d2d irq1
    0x164, // smmu eventq
    0x165, // smmu priq
    0x166, // smmu cmdq-sync
    0x168, // smmu gerror
    0x204,
    0x16b,
    0x57,
    0x58,
    0x59,
    0x5a,
    0x5b,
    0x5c,
    0x159,
    0x15a,
    0x15b,
    0x124,
];

// irqs belong to the root zone.
pub const ROOT_ZONE_IRQS: [u32; 28] = [
    0x39,
    0x4f, // emmc
    0x51, // sd-card
    0x55, // usb0
    0x56, // usb1
    0x64, // uart0
    0x3d, // ethernet0 macirq
    0x46, // ethernet1 macirq
    0x0a, // ddr-controller0 ecc
    0x12b, // ddr-controller1 ecc
    0x11f, // d2d irq0
    0x120, // d2d irq1
    0x164, // smmu eventq
    0x165, // smmu priq
    0x166, // smmu cmdq-sync
    0x168, // smmu gerror
    0x204,
    0x16b,
    0x57,
    0x58,
    0x59,
    0x5a,
    0x5b,
    0x5c,
    0x159,
    0x15a,
    0x15b,
    0x124,
];

pub const ROOT_ARCH_ZONE_CONFIG: HvArchZoneConfig = HvArchZoneConfig {
    plic_base: 0xc000000,
    plic_size: 0x4000000,
    aplic_base: 0xd000000,
    aplic_size: 0x8000,
};

// Virtio zone1 cmd:
//      nohup ./hvisor virtio start zone1-linux-virtio.json &
//      ./hvisor zone start zone1-linux.json
