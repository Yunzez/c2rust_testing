/* RQ4 coverage — single-TU amalgamation of the bzip2 1.0.8 *library* sources, i.e. exactly the
 * object files the shipped Makefile links into libbz2.a.  The differential-harness generator
 * (tools/stu_selector/gen_diff_harness.py) takes one C file per pair; this is the standard
 * amalgamation used elsewhere in this repo for the same reason.  No bzip2 code is modified. */
#include "blocksort.c"
#include "huffman.c"
#include "crctable.c"
#include "randtable.c"
#include "compress.c"
#include "decompress.c"
#include "bzlib.c"
