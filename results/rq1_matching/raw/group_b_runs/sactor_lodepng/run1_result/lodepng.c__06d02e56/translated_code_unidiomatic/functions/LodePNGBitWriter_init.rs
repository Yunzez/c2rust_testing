pub unsafe fn LodePNGBitWriter_init(writer: *mut LodePNGBitWriter, data: *mut ucvector) {
    if writer.is_null() {
        return;
    }
    (*writer).data = data;
    (*writer).bp = 0;
}
