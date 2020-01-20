#[macro_use]
mod util;
use util::*;

static _KBASE: usize = KERNELSPACE_BASE;

/* Memory Layout */
export![
    usize,
    USERSPACE_BASE = 0;
    USERSPACE_END = _1TB * 128;

    // 16M TB hole
    KERNELSPACE_BASE = TOP - (_1TB * 128) + 1;
    // 8TB guard hole
    PHYSICAL_MEMORY_MAPPING_BASE = KERNELSPACE_BASE + _1TB * 8;
    PHYSICAL_MEMORY_MAPPING_END = PHYSICAL_MEMORY_MAPPING_BASE + _1TB * 64;
    // 512GB guard hole
    VMALLOC_BASE = PHYSICAL_MEMORY_MAPPING_END + _512GB;
    VMALLOC_END = VMALLOC_BASE + 32 * _1TB;
    // 512GB guard hole
    PAGE_MAP_BASE = VMALLOC_END + _512GB;
    PAGE_MAP_END = PAGE_MAP_BASE + _1TB;
    // 512GB guard hole
    KERNEL_START = PAGE_MAP_END + _512GB;
    KERNEL_END = KERNEL_START + _1GB / 2;
];

