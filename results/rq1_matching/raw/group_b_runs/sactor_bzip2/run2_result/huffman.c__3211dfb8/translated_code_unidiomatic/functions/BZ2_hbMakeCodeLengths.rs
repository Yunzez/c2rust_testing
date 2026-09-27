pub unsafe fn BZ2_hbMakeCodeLengths(
    len: *mut libc::c_uchar,
    freq: *mut libc::c_int,
    alphaSize: libc::c_int,
    maxLen: libc::c_int,
) {
    #[inline(always)]
    fn weight_of(zz0: libc::c_int) -> libc::c_int {
        zz0 & (0xffffff00u32 as libc::c_int)
    }
    #[inline(always)]
    fn depth_of(zz1: libc::c_int) -> libc::c_int {
        zz1 & 0x000000ff
    }
    #[inline(always)]
    fn my_max(zz2: libc::c_int, zz3: libc::c_int) -> libc::c_int {
        if zz2 > zz3 {
            zz2
        } else {
            zz3
        }
    }
    #[inline(always)]
    fn add_weights(zw1: libc::c_int, zw2: libc::c_int) -> libc::c_int {
        (weight_of(zw1) + weight_of(zw2)) | (1 + my_max(depth_of(zw1), depth_of(zw2)))
    }
    #[inline(always)]
    unsafe fn assert_h(cond: bool, errcode: libc::c_int) {
        if !cond {
            BZ2_bz__AssertH__fail(errcode);
        }
    }
    const BZ_MAX_ALPHA_SIZE: usize = 258;
    let mut heap: [libc::c_int; BZ_MAX_ALPHA_SIZE + 2] = [0; BZ_MAX_ALPHA_SIZE + 2];
    let mut weight: [libc::c_int; BZ_MAX_ALPHA_SIZE * 2] = [0; BZ_MAX_ALPHA_SIZE * 2];
    let mut parent: [libc::c_int; BZ_MAX_ALPHA_SIZE * 2] = [0; BZ_MAX_ALPHA_SIZE * 2];
    fn upheap(z: libc::c_int, heap: &mut [libc::c_int], weight: &mut [libc::c_int]) {
        let mut zz: libc::c_int = z;
        let tmp: libc::c_int = heap[zz as usize];
        while weight[tmp as usize] < weight[heap[(zz >> 1) as usize] as usize] {
            heap[zz as usize] = heap[(zz >> 1) as usize];
            zz >>= 1;
        }
        heap[zz as usize] = tmp;
    }
    fn downheap(
        z: libc::c_int,
        heap: &mut [libc::c_int],
        weight: &mut [libc::c_int],
        n_heap: libc::c_int,
    ) {
        let mut zz: libc::c_int = z;
        let tmp: libc::c_int = heap[zz as usize];
        loop {
            let mut yy: libc::c_int = zz << 1;
            if yy > n_heap {
                break;
            }
            if yy < n_heap
                && weight[heap[(yy + 1) as usize] as usize] < weight[heap[yy as usize] as usize]
            {
                yy += 1;
            }
            if weight[tmp as usize] < weight[heap[yy as usize] as usize] {
                break;
            }
            heap[zz as usize] = heap[yy as usize];
            zz = yy;
        }
        heap[zz as usize] = tmp;
    }
    let mut n_nodes: libc::c_int;
    let mut n_heap: libc::c_int;
    let mut n1: libc::c_int;
    let mut n2: libc::c_int;
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut k: libc::c_int;
    let mut too_long: bool;
    i = 0;
    while i < alphaSize {
        let fi = *freq.add(i as usize);
        let v = if fi == 0 { 1 } else { fi };
        weight[(i + 1) as usize] = v << 8;
        i += 1;
    }
    loop {
        n_nodes = alphaSize;
        n_heap = 0;
        heap[0] = 0;
        weight[0] = 0;
        parent[0] = -2;
        i = 1;
        while i <= alphaSize {
            parent[i as usize] = -1;
            n_heap += 1;
            heap[n_heap as usize] = i;
            upheap(n_heap, &mut heap, &mut weight);
            i += 1;
        }
        assert_h(n_heap < (BZ_MAX_ALPHA_SIZE as libc::c_int + 2), 2001);
        while n_heap > 1 {
            n1 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            downheap(1, &mut heap, &mut weight, n_heap);
            n2 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            downheap(1, &mut heap, &mut weight, n_heap);
            n_nodes += 1;
            parent[n1 as usize] = n_nodes;
            parent[n2 as usize] = n_nodes;
            weight[n_nodes as usize] = add_weights(weight[n1 as usize], weight[n2 as usize]);
            parent[n_nodes as usize] = -1;
            n_heap += 1;
            heap[n_heap as usize] = n_nodes;
            upheap(n_heap, &mut heap, &mut weight);
        }
        assert_h(n_nodes < (BZ_MAX_ALPHA_SIZE as libc::c_int * 2), 2002);
        too_long = false;
        i = 1;
        while i <= alphaSize {
            j = 0;
            k = i;
            while parent[k as usize] >= 0 {
                k = parent[k as usize];
                j += 1;
            }
            *len.add((i - 1) as usize) = j as libc::c_uchar;
            if j > maxLen {
                too_long = true;
            }
            i += 1;
        }
        if !too_long {
            break;
        }
        i = 1;
        while i <= alphaSize {
            j = weight[i as usize] >> 8;
            j = 1 + (j / 2);
            weight[i as usize] = j << 8;
            i += 1;
        }
    }
}
