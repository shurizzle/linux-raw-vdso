pub unsafe fn elf_hash(mut ptr: *const u8) -> u32 {
    let mut h: u32 = 0;
    while *ptr != 0 {
        h = (h << 4).wrapping_add((*ptr) as u32);
        let g = h & 0xf000_0000;
        if g != 0 {
            h ^= g >> 24;
        }
        h &= !g;

        ptr = ptr.add(1);
    }
    h
}

pub unsafe fn streq(mut a: *const u8, mut b: *const u8) -> bool {
    while *a != 0 && (*a == *b) {
        a = a.add(1);
        b = b.add(1);
    }
    *a == *b
}
