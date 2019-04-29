use core::slice;

/// Helper function for returning 8-byte strings stored after a struct
pub unsafe fn c_str_at_end<T>(t: &T, string: usize) -> &[u8] {
    let mut ptr = (t as *const T).add(1) as *const u8;
    let mut j = 0;
    let mut i = 0;
    loop {
        if *ptr.add(i) == 0 {
            if j == string {
                break;
            } else {
                ptr = ptr.add(i);
                i = 0;
                j += 1;
                continue;
            }
        }
        i += 1;
    }
    slice::from_raw_parts(ptr, i)
}

/// Helper function for returning 16-byte strings stored after a struct
pub unsafe fn w_str_at_end<T>(t: &T, string: usize) -> &[u16] {
    let mut ptr = (t as *const T).add(1) as *const u16;
    let mut j = 0;
    let mut i = 0;
    loop {
        if *ptr.add(i) == 0 {
            if j == string {
                break;
            } else {
                ptr = ptr.add(i);
                i = 0;
                j += 1;
                continue;
            }
        }
        i += 1;
    }
    slice::from_raw_parts(ptr, i)
}
