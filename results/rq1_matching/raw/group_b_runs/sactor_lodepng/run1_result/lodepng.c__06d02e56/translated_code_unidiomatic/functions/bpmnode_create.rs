pub unsafe fn bpmnode_create(
    lists: *mut BPMLists,
    weight: ::core::ffi::c_int,
    index: ::core::ffi::c_uint,
    tail: *mut BPMNode,
) -> *mut BPMNode {
    unsafe fn get_memory_node(lists: *mut BPMLists, i: ::core::ffi::c_uint) -> *mut BPMNode {
        (*lists).memory.add(i as usize)
    }
    unsafe fn get_freelist_slot(lists: *mut BPMLists, i: ::core::ffi::c_uint) -> *mut *mut BPMNode {
        (*lists).freelist.add(i as usize)
    }
    unsafe fn get_chain0_entry(lists: *mut BPMLists, i: ::core::ffi::c_uint) -> *mut *mut BPMNode {
        (*lists).chains0.add(i as usize)
    }
    unsafe fn get_chain1_entry(lists: *mut BPMLists, i: ::core::ffi::c_uint) -> *mut *mut BPMNode {
        (*lists).chains1.add(i as usize)
    }
    let mut i: ::core::ffi::c_uint;
    if (*lists).nextfree >= (*lists).numfree {
        i = 0;
        while i != (*lists).memsize {
            (*get_memory_node(lists, i)).in_use = 0;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i != (*lists).listsize {
            let mut node: *mut BPMNode;
            node = *get_chain0_entry(lists, i);
            while !node.is_null() {
                (*node).in_use = 1;
                node = (*node).tail;
            }
            node = *get_chain1_entry(lists, i);
            while !node.is_null() {
                (*node).in_use = 1;
                node = (*node).tail;
            }
            i = i.wrapping_add(1);
        }
        (*lists).numfree = 0;
        i = 0;
        while i != (*lists).memsize {
            let mem_node = get_memory_node(lists, i);
            if (*mem_node).in_use == 0 {
                let nf = (*lists).numfree;
                *get_freelist_slot(lists, nf) = mem_node;
                (*lists).numfree = nf.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        (*lists).nextfree = 0;
    }
    let nf = (*lists).nextfree;
    let result = *get_freelist_slot(lists, nf);
    (*lists).nextfree = nf.wrapping_add(1);
    (*result).weight = weight;
    (*result).index = index;
    (*result).tail = tail;
    result
}
