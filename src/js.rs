pub fn usize_to_u32(mut v: Vec<usize>) -> Vec<u32> {
    let ratio = std::mem::size_of::<usize>() / std::mem::size_of::<u32>();
    let length = v.len() * ratio;
    let capacity = v.capacity() * ratio;
    let ptr = v.as_mut_ptr() as *mut u32;
    std::mem::forget(v);

    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
}

pub fn u32_to_usize(mut v: Vec<u32>) -> Vec<usize> {
    let ratio = std::mem::size_of::<u32>() / std::mem::size_of::<usize>();
    let length = v.len() * ratio;
    let capacity = v.capacity() * ratio;
    let ptr = v.as_mut_ptr() as *mut usize;
    std::mem::forget(v);

    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
}
