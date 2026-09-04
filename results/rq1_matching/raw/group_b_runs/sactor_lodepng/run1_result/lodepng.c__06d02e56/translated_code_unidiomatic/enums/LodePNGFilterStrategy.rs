#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LodePNGFilterStrategy {
    LFS_ZERO = 0,
    LFS_ONE = 1,
    LFS_TWO = 2,
    LFS_THREE = 3,
    LFS_FOUR = 4,
    LFS_MINSUM = 5,
    LFS_ENTROPY = 6,
    LFS_BRUTE_FORCE = 7,
    LFS_PREDEFINED = 8,
}
