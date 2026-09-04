// [SYSTEM: /home/yunzez/c2rust_testing/tools/frameworks/ptrtrans_rebuild/SVF/llvm-14.0.0.obj/lib/clang/14.0.0/include/stddef.h]
typedef long unsigned int size_t;
// [SYSTEM: /home/yunzez/c2rust_testing/tools/frameworks/ptrtrans_rebuild/SVF/llvm-14.0.0.obj/lib/clang/14.0.0/include/stddef.h]
typedef int wchar_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/floatn.h]
typedef _Complex float __cfloat128 __attribute__ ((__mode__ (__TC__)));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/floatn.h]
typedef __float128 _Float128;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/floatn-common.h]
typedef float _Float32;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/floatn-common.h]
typedef double _Float64;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/floatn-common.h]
typedef double _Float32x;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/floatn-common.h]
typedef long double _Float64x;
// [SYSTEM: /usr/include/stdlib.h]
typedef struct {
    int quot;
    int rem;
  } div_t;
// [SYSTEM: /usr/include/stdlib.h]
typedef struct {
    long int quot;
    long int rem;
  } ldiv_t;
// [SYSTEM: /usr/include/stdlib.h]
__extension__ typedef struct {
    long long int quot;
    long long int rem;
  } lldiv_t;
// [SYSTEM: /usr/include/stdlib.h]
extern size_t __ctype_get_mb_cur_max (void) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdlib.h]
extern double atof (const char *__nptr) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int atoi (const char *__nptr) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern long int atol (const char *__nptr) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
__extension__ extern long long int atoll (const char *__nptr) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern double strtod (const char *__restrict __nptr, char **__restrict __endptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern float strtof (const char *__restrict __nptr, char **__restrict __endptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern long double strtold (const char *__restrict __nptr, char **__restrict __endptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern long int strtol (const char *__restrict __nptr, char **__restrict __endptr, int __base) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern unsigned long int strtoul (const char *__restrict __nptr, char **__restrict __endptr, int __base) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
__extension__ extern long long int strtoq (const char *__restrict __nptr, char **__restrict __endptr, int __base) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
__extension__ extern unsigned long long int strtouq (const char *__restrict __nptr, char **__restrict __endptr, int __base) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
__extension__ extern long long int strtoll (const char *__restrict __nptr, char **__restrict __endptr, int __base) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
__extension__ extern unsigned long long int strtoull (const char *__restrict __nptr, char **__restrict __endptr, int __base) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern char *l64a (long int __n) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdlib.h]
extern long int a64l (const char *__s) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned char __u_char;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned short int __u_short;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned int __u_int;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __u_long;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef signed char __int8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned char __uint8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef signed short int __int16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned short int __uint16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef signed int __int32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned int __uint32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef signed long int __int64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __uint64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __int8_t __int_least8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __uint8_t __uint_least8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __int16_t __int_least16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __uint16_t __uint_least16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __int32_t __int_least32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __uint32_t __uint_least32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __int64_t __int_least64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __uint64_t __uint_least64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __quad_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __u_quad_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __intmax_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __uintmax_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __dev_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned int __uid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned int __gid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __ino_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __ino64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned int __mode_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __nlink_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __off_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __off64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef int __pid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef struct { int __val[2]; } __fsid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __clock_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __rlim_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __rlim64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned int __id_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __time_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned int __useconds_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __suseconds_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __suseconds64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef int __daddr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef int __key_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef int __clockid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef void * __timer_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __blksize_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __blkcnt_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __blkcnt64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __fsblkcnt_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __fsblkcnt64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __fsfilcnt_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __fsfilcnt64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __fsword_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __ssize_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __syscall_slong_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned long int __syscall_ulong_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef __off64_t __loff_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef char *__caddr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef long int __intptr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef unsigned int __socklen_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types.h]
typedef int __sig_atomic_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __u_char u_char;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __u_short u_short;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __u_int u_int;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __u_long u_long;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __quad_t quad_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __u_quad_t u_quad_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __fsid_t fsid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __loff_t loff_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __ino_t ino_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __dev_t dev_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __gid_t gid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __mode_t mode_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __nlink_t nlink_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __uid_t uid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __off_t off_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __pid_t pid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __id_t id_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __ssize_t ssize_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __daddr_t daddr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __caddr_t caddr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __key_t key_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/clock_t.h]
typedef __clock_t clock_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/clockid_t.h]
typedef __clockid_t clockid_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/time_t.h]
typedef __time_t time_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/timer_t.h]
typedef __timer_t timer_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef unsigned long int ulong;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef unsigned short int ushort;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef unsigned int uint;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-intn.h]
typedef __int8_t int8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-intn.h]
typedef __int16_t int16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-intn.h]
typedef __int32_t int32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-intn.h]
typedef __int64_t int64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __uint8_t u_int8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __uint16_t u_int16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __uint32_t u_int32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __uint64_t u_int64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef int register_t __attribute__ ((__mode__ (__word__)));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/byteswap.h]
static __inline __uint16_t __bswap_16 (__uint16_t __bsx) {
  return ((__uint16_t) ((((__bsx) >> 8) & 0xff) | (((__bsx) & 0xff) << 8)));
}
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/byteswap.h]
static __inline __uint32_t __bswap_32 (__uint32_t __bsx) {
  return ((((__bsx) & 0xff000000u) >> 24) | (((__bsx) & 0x00ff0000u) >> 8) | (((__bsx) & 0x0000ff00u) << 8) | (((__bsx) & 0x000000ffu) << 24));
}
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/byteswap.h]
__extension__ static __inline __uint64_t __bswap_64 (__uint64_t __bsx) {
  return ((((__bsx) & 0xff00000000000000ull) >> 56) | (((__bsx) & 0x00ff000000000000ull) >> 40) | (((__bsx) & 0x0000ff0000000000ull) >> 24) | (((__bsx) & 0x000000ff00000000ull) >> 8) | (((__bsx) & 0x00000000ff000000ull) << 8) | (((__bsx) & 0x0000000000ff0000ull) << 24) | (((__bsx) & 0x000000000000ff00ull) << 40) | (((__bsx) & 0x00000000000000ffull) << 56));
}
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/uintn-identity.h]
static __inline __uint16_t __uint16_identity (__uint16_t __x) {
  return __x;
}
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/uintn-identity.h]
static __inline __uint32_t __uint32_identity (__uint32_t __x) {
  return __x;
}
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/uintn-identity.h]
static __inline __uint64_t __uint64_identity (__uint64_t __x) {
  return __x;
}
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/__sigset_t.h]
typedef struct {
  unsigned long int __val[(1024 / (8 * sizeof (unsigned long int)))];
} __sigset_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/sigset_t.h]
typedef __sigset_t sigset_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/struct_timeval.h]
struct timeval {
  __time_t tv_sec;
  __suseconds_t tv_usec;
};
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/struct_timespec.h]
struct timespec {
  __time_t tv_sec;
  __syscall_slong_t tv_nsec;
};
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/select.h]
typedef __suseconds_t suseconds_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/select.h]
typedef long int __fd_mask;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/select.h]
typedef struct {
    __fd_mask __fds_bits[1024 / (8 * (int) sizeof (__fd_mask))];
  } fd_set;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/select.h]
typedef __fd_mask fd_mask;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/select.h]
extern int select (int __nfds, fd_set *__restrict __readfds, fd_set *__restrict __writefds, fd_set *__restrict __exceptfds, struct timeval *__restrict __timeout);
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/select.h]
extern int pselect (int __nfds, fd_set *__restrict __readfds, fd_set *__restrict __writefds, fd_set *__restrict __exceptfds, const struct timespec *__restrict __timeout, const __sigset_t *__restrict __sigmask);
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __blksize_t blksize_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __blkcnt_t blkcnt_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __fsblkcnt_t fsblkcnt_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/sys/types.h]
typedef __fsfilcnt_t fsfilcnt_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/atomic_wide_counter.h]
typedef union {
  __extension__ unsigned long long int __value64;
  struct {
    unsigned int __low;
    unsigned int __high;
  } __value32;
} __atomic_wide_counter;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/thread-shared-types.h]
typedef struct __pthread_internal_list {
  struct __pthread_internal_list *__prev;
  struct __pthread_internal_list *__next;
} __pthread_list_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/thread-shared-types.h]
typedef struct __pthread_internal_slist {
  struct __pthread_internal_slist *__next;
} __pthread_slist_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/struct_mutex.h]
struct __pthread_mutex_s {
  int __lock;
  unsigned int __count;
  int __owner;
  unsigned int __nusers;
  int __kind;
  short __spins;
  short __unused;
  __pthread_list_t __list;
};
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/struct_rwlock.h]
struct __pthread_rwlock_arch_t {
  unsigned int __readers;
  unsigned int __writers;
  unsigned int __wrphase_futex;
  unsigned int __writers_futex;
  unsigned int __pad3;
  unsigned int __pad4;
  int __cur_writer;
  int __shared;
  unsigned long int __pad1;
  unsigned long int __pad2;
  unsigned int __flags;
};
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/thread-shared-types.h]
struct __pthread_cond_s {
  __atomic_wide_counter __wseq;
  __atomic_wide_counter __g1_start;
  unsigned int __g_size[2] ;
  unsigned int __g1_orig_size;
  unsigned int __wrefs;
  unsigned int __g_signals[2];
  unsigned int __unused_initialized_1;
  unsigned int __unused_initialized_2;
};
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/thread-shared-types.h]
typedef unsigned int __tss_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/thread-shared-types.h]
typedef unsigned long int __thrd_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/thread-shared-types.h]
typedef struct {
  int __data ;
} __once_flag;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef unsigned long int pthread_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union {
  char __size[4];
  int __align;
} pthread_mutexattr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union {
  char __size[4];
  int __align;
} pthread_condattr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef unsigned int pthread_key_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef int pthread_once_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
union pthread_attr_t {
  char __size[56];
  long int __align;
};
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union pthread_attr_t pthread_attr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union {
  struct __pthread_mutex_s __data;
  char __size[40];
  long int __align;
} pthread_mutex_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union {
  struct __pthread_cond_s __data;
  char __size[48];
  __extension__ long long int __align;
} pthread_cond_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union {
  struct __pthread_rwlock_arch_t __data;
  char __size[56];
  long int __align;
} pthread_rwlock_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union {
  char __size[8];
  long int __align;
} pthread_rwlockattr_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef volatile int pthread_spinlock_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union {
  char __size[32];
  long int __align;
} pthread_barrier_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/pthreadtypes.h]
typedef union {
  char __size[4];
  int __align;
} pthread_barrierattr_t;
// [SYSTEM: /usr/include/stdlib.h]
extern long int random (void) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern void srandom (unsigned int __seed) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern char *initstate (unsigned int __seed, char *__statebuf, size_t __statelen) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdlib.h]
extern char *setstate (char *__statebuf) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
struct random_data {
    int32_t *fptr;
    int32_t *rptr;
    int32_t *state;
    int rand_type;
    int rand_deg;
    int rand_sep;
    int32_t *end_ptr;
  };
// [SYSTEM: /usr/include/stdlib.h]
extern int random_r (struct random_data *__restrict __buf, int32_t *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int srandom_r (unsigned int __seed, struct random_data *__buf) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int initstate_r (unsigned int __seed, char *__restrict __statebuf, size_t __statelen, struct random_data *__restrict __buf) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2, 4)));
// [SYSTEM: /usr/include/stdlib.h]
extern int setstate_r (char *__restrict __statebuf, struct random_data *__restrict __buf) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int rand (void) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern void srand (unsigned int __seed) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern int rand_r (unsigned int *__seed) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern double drand48 (void) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern double erand48 (unsigned short int __xsubi[3]) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern long int lrand48 (void) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern long int nrand48 (unsigned short int __xsubi[3]) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern long int mrand48 (void) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern long int jrand48 (unsigned short int __xsubi[3]) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern void srand48 (long int __seedval) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern unsigned short int *seed48 (unsigned short int __seed16v[3]) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern void lcong48 (unsigned short int __param[7]) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
struct drand48_data {
    unsigned short int __x[3];
    unsigned short int __old_x[3];
    unsigned short int __c;
    unsigned short int __init;
    __extension__ unsigned long long int __a;
  };
// [SYSTEM: /usr/include/stdlib.h]
extern int drand48_r (struct drand48_data *__restrict __buffer, double *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int erand48_r (unsigned short int __xsubi[3], struct drand48_data *__restrict __buffer, double *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int lrand48_r (struct drand48_data *__restrict __buffer, long int *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int nrand48_r (unsigned short int __xsubi[3], struct drand48_data *__restrict __buffer, long int *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int mrand48_r (struct drand48_data *__restrict __buffer, long int *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int jrand48_r (unsigned short int __xsubi[3], struct drand48_data *__restrict __buffer, long int *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int srand48_r (long int __seedval, struct drand48_data *__buffer) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int seed48_r (unsigned short int __seed16v[3], struct drand48_data *__buffer) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int lcong48_r (unsigned short int __param[7], struct drand48_data *__buffer) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/stdlib.h]
extern __uint32_t arc4random (void) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void arc4random_buf (void *__buf, size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern __uint32_t arc4random_uniform (__uint32_t __upper_bound) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void *malloc (size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void *calloc (size_t __nmemb, size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void *realloc (void *__ptr, size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__warn_unused_result__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void free (void *__ptr) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern void *reallocarray (void *__ptr, size_t __nmemb, size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__warn_unused_result__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void *reallocarray (void *__ptr, size_t __nmemb, size_t __size) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/alloca.h]
extern void *alloca (size_t __size) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern void *valloc (size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int posix_memalign (void **__memptr, size_t __alignment, size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void *aligned_alloc (size_t __alignment, size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) __attribute__ ((__alloc_align__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void abort (void) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));
// [SYSTEM: /usr/include/stdlib.h]
extern int atexit (void (*__func) (void)) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern int at_quick_exit (void (*__func) (void)) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern int on_exit (void (*__func) (int __status, void *__arg), void *__arg) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern void exit (int __status) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
// [SYSTEM: /usr/include/stdlib.h]
extern void quick_exit (int __status) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
// [SYSTEM: /usr/include/stdlib.h]
extern void _Exit (int __status) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
// [SYSTEM: /usr/include/stdlib.h]
extern char *getenv (const char *__name) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int putenv (char *__string) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern int setenv (const char *__name, const char *__value, int __replace) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdlib.h]
extern int unsetenv (const char *__name) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern int clearenv (void) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern char *mktemp (char *__template) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdlib.h]
extern int mkstemp (char *__template) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int mkstemps (char *__template, int __suffixlen) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern char *mkdtemp (char *__template) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int system (const char *__command) ;
// [SYSTEM: /usr/include/stdlib.h]
extern char *realpath (const char *__restrict __name, char *__restrict __resolved) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdlib.h]
typedef int (*__compar_fn_t) (const void *, const void *);
// [SYSTEM: /usr/include/stdlib.h]
extern void *bsearch (const void *__key, const void *__base, size_t __nmemb, size_t __size, __compar_fn_t __compar) __attribute__ ((__nonnull__ (1, 2, 5))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern void qsort (void *__base, size_t __nmemb, size_t __size, __compar_fn_t __compar) __attribute__ ((__nonnull__ (1, 4)));
// [SYSTEM: /usr/include/stdlib.h]
extern int abs (int __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern long int labs (long int __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
// [SYSTEM: /usr/include/stdlib.h]
__extension__ extern long long int llabs (long long int __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern div_t div (int __numer, int __denom) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern ldiv_t ldiv (long int __numer, long int __denom) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
// [SYSTEM: /usr/include/stdlib.h]
__extension__ extern lldiv_t lldiv (long long int __numer, long long int __denom) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
// [SYSTEM: /usr/include/stdlib.h]
extern char *ecvt (double __value, int __ndigit, int *__restrict __decpt, int *__restrict __sign) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern char *fcvt (double __value, int __ndigit, int *__restrict __decpt, int *__restrict __sign) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern char *gcvt (double __value, int __ndigit, char *__buf) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern char *qecvt (long double __value, int __ndigit, int *__restrict __decpt, int *__restrict __sign) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern char *qfcvt (long double __value, int __ndigit, int *__restrict __decpt, int *__restrict __sign) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern char *qgcvt (long double __value, int __ndigit, char *__buf) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int ecvt_r (double __value, int __ndigit, int *__restrict __decpt, int *__restrict __sign, char *__restrict __buf, size_t __len) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4, 5)));
// [SYSTEM: /usr/include/stdlib.h]
extern int fcvt_r (double __value, int __ndigit, int *__restrict __decpt, int *__restrict __sign, char *__restrict __buf, size_t __len) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4, 5)));
// [SYSTEM: /usr/include/stdlib.h]
extern int qecvt_r (long double __value, int __ndigit, int *__restrict __decpt, int *__restrict __sign, char *__restrict __buf, size_t __len) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4, 5)));
// [SYSTEM: /usr/include/stdlib.h]
extern int qfcvt_r (long double __value, int __ndigit, int *__restrict __decpt, int *__restrict __sign, char *__restrict __buf, size_t __len) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4, 5)));
// [SYSTEM: /usr/include/stdlib.h]
extern int mblen (const char *__s, size_t __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern int mbtowc (wchar_t *__restrict __pwc, const char *__restrict __s, size_t __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern int wctomb (char *__s, wchar_t __wchar) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdlib.h]
extern size_t mbstowcs (wchar_t *__restrict __pwcs, const char *__restrict __s, size_t __n) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdlib.h]
extern size_t wcstombs (char *__restrict __s, const wchar_t *__restrict __pwcs, size_t __n) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int rpmatch (const char *__response) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int getsubopt (char **__restrict __optionp, char *const *__restrict __tokens, char **__restrict __valuep) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2, 3))) ;
// [SYSTEM: /usr/include/stdlib.h]
extern int getloadavg (double __loadavg[], int __nelem) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /home/yunzez/c2rust_testing/tools/frameworks/ptrtrans_rebuild/SVF/llvm-14.0.0.obj/lib/clang/14.0.0/include/stdarg.h]
typedef __builtin_va_list va_list;
// [SYSTEM: /home/yunzez/c2rust_testing/tools/frameworks/ptrtrans_rebuild/SVF/llvm-14.0.0.obj/lib/clang/14.0.0/include/stdarg.h]
typedef __builtin_va_list __gnuc_va_list;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/__mbstate_t.h]
typedef struct {
  int __count;
  union {
    unsigned int __wch;
    char __wchb[4];
  } __value;
} __mbstate_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/__fpos_t.h]
typedef struct _G_fpos_t {
  __off_t __pos;
  __mbstate_t __state;
} __fpos_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/__fpos64_t.h]
typedef struct _G_fpos64_t {
  __off64_t __pos;
  __mbstate_t __state;
} __fpos64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/__FILE.h]
struct _IO_FILE;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/__FILE.h]
typedef struct _IO_FILE __FILE;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/FILE.h]
struct _IO_FILE;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/FILE.h]
typedef struct _IO_FILE FILE;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h]
struct _IO_FILE;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h]
struct _IO_marker;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h]
struct _IO_codecvt;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h]
struct _IO_wide_data;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h]
typedef void _IO_lock_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h]
struct _IO_FILE {
  int _flags;
  char *_IO_read_ptr;
  char *_IO_read_end;
  char *_IO_read_base;
  char *_IO_write_base;
  char *_IO_write_ptr;
  char *_IO_write_end;
  char *_IO_buf_base;
  char *_IO_buf_end;
  char *_IO_save_base;
  char *_IO_backup_base;
  char *_IO_save_end;
  struct _IO_marker *_markers;
  struct _IO_FILE *_chain;
  int _fileno;
  int _flags2:24;
  char _short_backupbuf[1];
  __off_t _old_offset;
  unsigned short _cur_column;
  signed char _vtable_offset;
  char _shortbuf[1];
  _IO_lock_t *_lock;
  __off64_t _offset;
  struct _IO_codecvt *_codecvt;
  struct _IO_wide_data *_wide_data;
  struct _IO_FILE *_freeres_list;
  void *_freeres_buf;
  struct _IO_FILE **_prevchain;
  int _mode;
  int _unused3;
  __uint64_t _total_written;
  char _unused2[12 * sizeof (int) - 5 * sizeof (void *)];
};
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/cookie_io_functions_t.h]
typedef __ssize_t cookie_read_function_t (void *__cookie, char *__buf, size_t __nbytes);
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/cookie_io_functions_t.h]
typedef __ssize_t cookie_write_function_t (void *__cookie, const char *__buf, size_t __nbytes);
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/cookie_io_functions_t.h]
typedef int cookie_seek_function_t (void *__cookie, __off64_t *__pos, int __w);
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/cookie_io_functions_t.h]
typedef int cookie_close_function_t (void *__cookie);
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/cookie_io_functions_t.h]
typedef struct _IO_cookie_io_functions_t {
  cookie_read_function_t *read;
  cookie_write_function_t *write;
  cookie_seek_function_t *seek;
  cookie_close_function_t *close;
} cookie_io_functions_t;
// [SYSTEM: /usr/include/stdio.h]
typedef __gnuc_va_list va_list;
// [SYSTEM: /usr/include/stdio.h]
typedef __fpos_t fpos_t;
// [SYSTEM: /usr/include/stdio.h]
extern FILE *stdin;
// [SYSTEM: /usr/include/stdio.h]
extern FILE *stdout;
// [SYSTEM: /usr/include/stdio.h]
extern FILE *stderr;
// [SYSTEM: /usr/include/stdio.h]
extern int remove (const char *__filename) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdio.h]
extern int rename (const char *__old, const char *__new) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdio.h]
extern int renameat (int __oldfd, const char *__old, int __newfd, const char *__new) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdio.h]
extern int fclose (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern FILE *tmpfile (void) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdio.h]
extern char *tmpnam (char[20]) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdio.h]
extern char *tmpnam_r (char __s[20]) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdio.h]
extern char *tempnam (const char *__dir, const char *__pfx) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdio.h]
extern int fflush (FILE *__stream);
// [SYSTEM: /usr/include/stdio.h]
extern int fflush_unlocked (FILE *__stream);
// [SYSTEM: /usr/include/stdio.h]
extern FILE *fopen (const char *__restrict __filename, const char *__restrict __modes) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdio.h]
extern FILE *freopen (const char *__restrict __filename, const char *__restrict __modes, FILE *__restrict __stream) __attribute__ ((__nonnull__ (3)));
// [SYSTEM: /usr/include/stdio.h]
extern FILE *fdopen (int __fd, const char *__modes) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdio.h]
extern FILE *fopencookie (void *__restrict __magic_cookie, const char *__restrict __modes, cookie_io_functions_t __io_funcs) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdio.h]
extern FILE *fmemopen (void *__s, size_t __len, const char *__modes) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdio.h]
extern FILE *open_memstream (char **__bufloc, size_t *__sizeloc) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdio.h]
extern void setbuf (FILE *__restrict __stream, char *__restrict __buf) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int setvbuf (FILE *__restrict __stream, char *__restrict __buf, int __modes, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern void setbuffer (FILE *__restrict __stream, char *__restrict __buf, size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern void setlinebuf (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int fprintf (FILE *__restrict __stream, const char *__restrict __format, ...) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int printf (const char *__restrict __format, ...);
// [SYSTEM: /usr/include/stdio.h]
extern int sprintf (char *__restrict __s, const char *__restrict __format, ...) __attribute__ ((__nothrow__));
// [SYSTEM: /usr/include/stdio.h]
extern int vfprintf (FILE *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int vprintf (const char *__restrict __format, __gnuc_va_list __arg);
// [SYSTEM: /usr/include/stdio.h]
extern int vsprintf (char *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __attribute__ ((__nothrow__));
// [SYSTEM: /usr/include/stdio.h]
extern int snprintf (char *__restrict __s, size_t __maxlen, const char *__restrict __format, ...) __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 3, 4)));
// [SYSTEM: /usr/include/stdio.h]
extern int vsnprintf (char *__restrict __s, size_t __maxlen, const char *__restrict __format, __gnuc_va_list __arg) __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 3, 0)));
// [SYSTEM: /usr/include/stdio.h]
extern int vasprintf (char **__restrict __ptr, const char *__restrict __f, __gnuc_va_list __arg) __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 2, 0))) ;
// [SYSTEM: /usr/include/stdio.h]
extern int __asprintf (char **__restrict __ptr, const char *__restrict __fmt, ...) __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 2, 3))) ;
// [SYSTEM: /usr/include/stdio.h]
extern int asprintf (char **__restrict __ptr, const char *__restrict __fmt, ...) __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 2, 3))) ;
// [SYSTEM: /usr/include/stdio.h]
extern int vdprintf (int __fd, const char *__restrict __fmt, __gnuc_va_list __arg) __attribute__ ((__format__ (__printf__, 2, 0)));
// [SYSTEM: /usr/include/stdio.h]
extern int dprintf (int __fd, const char *__restrict __fmt, ...) __attribute__ ((__format__ (__printf__, 2, 3)));
// [SYSTEM: /usr/include/stdio.h]
extern int fscanf (FILE *__restrict __stream, const char *__restrict __format, ...) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int scanf (const char *__restrict __format, ...) ;
// [SYSTEM: /usr/include/stdio.h]
extern int sscanf (const char *__restrict __s, const char *__restrict __format, ...) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdio.h]
extern int fscanf (FILE *__restrict __stream, const char *__restrict __format, ...) __asm__ ("" "__isoc99_fscanf") __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int scanf (const char *__restrict __format, ...) __asm__ ("" "__isoc99_scanf") ;
// [SYSTEM: /usr/include/stdio.h]
extern int sscanf (const char *__restrict __s, const char *__restrict __format, ...) __asm__ ("" "__isoc99_sscanf") __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/stdio.h]
extern int vfscanf (FILE *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __attribute__ ((__format__ (__scanf__, 2, 0))) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int vscanf (const char *__restrict __format, __gnuc_va_list __arg) __attribute__ ((__format__ (__scanf__, 1, 0))) ;
// [SYSTEM: /usr/include/stdio.h]
extern int vsscanf (const char *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __attribute__ ((__nothrow__ )) __attribute__ ((__format__ (__scanf__, 2, 0)));
// [SYSTEM: /usr/include/stdio.h]
extern int vfscanf (FILE *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc99_vfscanf") __attribute__ ((__format__ (__scanf__, 2, 0))) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int vscanf (const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc99_vscanf") __attribute__ ((__format__ (__scanf__, 1, 0))) ;
// [SYSTEM: /usr/include/stdio.h]
extern int vsscanf (const char *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc99_vsscanf") __attribute__ ((__nothrow__ )) __attribute__ ((__format__ (__scanf__, 2, 0)));
// [SYSTEM: /usr/include/stdio.h]
extern int fgetc (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int getc (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int getchar (void);
// [SYSTEM: /usr/include/stdio.h]
extern int getc_unlocked (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int getchar_unlocked (void);
// [SYSTEM: /usr/include/stdio.h]
extern int fgetc_unlocked (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int fputc (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdio.h]
extern int putc (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdio.h]
extern int putchar (int __c);
// [SYSTEM: /usr/include/stdio.h]
extern int fputc_unlocked (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdio.h]
extern int putc_unlocked (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdio.h]
extern int putchar_unlocked (int __c);
// [SYSTEM: /usr/include/stdio.h]
extern int getw (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int putw (int __w, FILE *__stream) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdio.h]
extern char *fgets (char *__restrict __s, int __n, FILE *__restrict __stream) __attribute__ ((__nonnull__ (3)));
// [SYSTEM: /usr/include/stdio.h]
extern __ssize_t __getdelim (char **__restrict __lineptr, size_t *__restrict __n, int __delimiter, FILE *__restrict __stream) __attribute__ ((__nonnull__ (4)));
// [SYSTEM: /usr/include/stdio.h]
extern __ssize_t getdelim (char **__restrict __lineptr, size_t *__restrict __n, int __delimiter, FILE *__restrict __stream) __attribute__ ((__nonnull__ (4)));
// [SYSTEM: /usr/include/stdio.h]
extern __ssize_t getline (char **__restrict __lineptr, size_t *__restrict __n, FILE *__restrict __stream) __attribute__ ((__nonnull__ (3)));
// [SYSTEM: /usr/include/stdio.h]
extern int fputs (const char *__restrict __s, FILE *__restrict __stream) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdio.h]
extern int puts (const char *__s);
// [SYSTEM: /usr/include/stdio.h]
extern int ungetc (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/stdio.h]
extern size_t fread (void *__restrict __ptr, size_t __size, size_t __n, FILE *__restrict __stream) __attribute__ ((__nonnull__ (4)));
// [SYSTEM: /usr/include/stdio.h]
extern size_t fwrite (const void *__restrict __ptr, size_t __size, size_t __n, FILE *__restrict __s) __attribute__ ((__nonnull__ (4)));
// [SYSTEM: /usr/include/stdio.h]
extern size_t fread_unlocked (void *__restrict __ptr, size_t __size, size_t __n, FILE *__restrict __stream) __attribute__ ((__nonnull__ (4)));
// [SYSTEM: /usr/include/stdio.h]
extern size_t fwrite_unlocked (const void *__restrict __ptr, size_t __size, size_t __n, FILE *__restrict __stream) __attribute__ ((__nonnull__ (4)));
// [SYSTEM: /usr/include/stdio.h]
extern int fseek (FILE *__stream, long int __off, int __whence) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern long int ftell (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern void rewind (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int fseeko (FILE *__stream, __off_t __off, int __whence) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern __off_t ftello (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int fgetpos (FILE *__restrict __stream, fpos_t *__restrict __pos) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int fsetpos (FILE *__stream, const fpos_t *__pos) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern void clearerr (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int feof (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int ferror (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern void clearerr_unlocked (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int feof_unlocked (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int ferror_unlocked (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern void perror (const char *__s) __attribute__ ((__cold__));
// [SYSTEM: /usr/include/stdio.h]
extern int fileno (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int fileno_unlocked (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int pclose (FILE *__stream) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern FILE *popen (const char *__command, const char *__modes) __attribute__ ((__malloc__)) ;
// [SYSTEM: /usr/include/stdio.h]
extern char *ctermid (char *__s) __attribute__ ((__nothrow__ )) ;
// [SYSTEM: /usr/include/stdio.h]
extern void flockfile (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int ftrylockfile (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern void funlockfile (FILE *__stream) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/stdio.h]
extern int __uflow (FILE *);
// [SYSTEM: /usr/include/stdio.h]
extern int __overflow (FILE *, int);
// [SYSTEM: /usr/include/string.h]
extern void *memcpy (void *__restrict __dest, const void *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern void *memmove (void *__dest, const void *__src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern void *memccpy (void *__restrict __dest, const void *__restrict __src, int __c, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2))) ;
// [SYSTEM: /usr/include/string.h]
extern void *memset (void *__s, int __c, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern void *memset_explicit (void *__s, int __c, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/string.h]
extern int memcmp (const void *__s1, const void *__s2, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern int __memcmpeq (const void *__s1, const void *__s2, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern void *memchr (const void *__s, int __c, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern char *strcpy (char *__restrict __dest, const char *__restrict __src) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *strncpy (char *__restrict __dest, const char *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *strcat (char *__restrict __dest, const char *__restrict __src) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *strncat (char *__restrict __dest, const char *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern int strcmp (const char *__s1, const char *__s2) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern int strncmp (const char *__s1, const char *__s2, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern int strcoll (const char *__s1, const char *__s2) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern size_t strxfrm (char *__restrict __dest, const char *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2))) ;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/__locale_t.h]
struct __locale_struct {
  struct __locale_data *__locales[13];
  const unsigned short int *__ctype_b;
  const int *__ctype_tolower;
  const int *__ctype_toupper;
  const char *__names[13];
};
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/__locale_t.h]
typedef struct __locale_struct *__locale_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/types/locale_t.h]
typedef __locale_t locale_t;
// [SYSTEM: /usr/include/string.h]
extern int strcoll_l (const char *__s1, const char *__s2, locale_t __l) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 3)));
// [SYSTEM: /usr/include/string.h]
extern size_t strxfrm_l (char *__dest, const char *__src, size_t __n, locale_t __l) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2, 4))) ;
// [SYSTEM: /usr/include/string.h]
extern char *strdup (const char *__s) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern char *strndup (const char *__string, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern char *strchr (const char *__s, int __c) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern char *strrchr (const char *__s, int __c) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern char *strchrnul (const char *__s, int __c) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern size_t strcspn (const char *__s, const char *__reject) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern size_t strspn (const char *__s, const char *__accept) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *strpbrk (const char *__s, const char *__accept) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *strstr (const char *__haystack, const char *__needle) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *strtok (char *__restrict __s, const char *__restrict __delim) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/string.h]
extern char *__strtok_r (char *__restrict __s, const char *__restrict __delim, char **__restrict __save_ptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2, 3)));
// [SYSTEM: /usr/include/string.h]
extern char *strtok_r (char *__restrict __s, const char *__restrict __delim, char **__restrict __save_ptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2, 3)));
// [SYSTEM: /usr/include/string.h]
extern char *strcasestr (const char *__haystack, const char *__needle) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern void *memmem (const void *__haystack, size_t __haystacklen, const void *__needle, size_t __needlelen) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 3))) ;
// [SYSTEM: /usr/include/string.h]
extern void *__mempcpy (void *__restrict __dest, const void *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern void *mempcpy (void *__restrict __dest, const void *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern size_t strlen (const char *__s) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern size_t strnlen (const char *__string, size_t __maxlen) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/string.h]
extern char *strerror (int __errnum) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/string.h]
extern int strerror_r (int __errnum, char *__buf, size_t __buflen) __asm__ ("" "__xpg_strerror_r") __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2))) ;
// [SYSTEM: /usr/include/string.h]
extern char *strerror_l (int __errnum, locale_t __l) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/strings.h]
extern int bcmp (const void *__s1, const void *__s2, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/strings.h]
extern void bcopy (const void *__src, void *__dest, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/strings.h]
extern void bzero (void *__s, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/strings.h]
extern char *index (const char *__s, int __c) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/strings.h]
extern char *rindex (const char *__s, int __c) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
// [SYSTEM: /usr/include/strings.h]
extern int ffs (int __i) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/strings.h]
extern int ffsl (long int __l) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/strings.h]
__extension__ extern int ffsll (long long int __ll) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/strings.h]
extern int strcasecmp (const char *__s1, const char *__s2) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/strings.h]
extern int strncasecmp (const char *__s1, const char *__s2, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/strings.h]
extern int strcasecmp_l (const char *__s1, const char *__s2, locale_t __loc) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 3)));
// [SYSTEM: /usr/include/strings.h]
extern int strncasecmp_l (const char *__s1, const char *__s2, size_t __n, locale_t __loc) __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 4)));
// [SYSTEM: /usr/include/string.h]
extern void explicit_bzero (void *__s, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
// [SYSTEM: /usr/include/string.h]
extern char *strsep (char **__restrict __stringp, const char *__restrict __delim) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *strsignal (int __sig) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/string.h]
extern char *__stpcpy (char *__restrict __dest, const char *__restrict __src) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *stpcpy (char *__restrict __dest, const char *__restrict __src) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *__stpncpy (char *__restrict __dest, const char *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern char *stpncpy (char *__restrict __dest, const char *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
// [SYSTEM: /usr/include/string.h]
extern size_t strlcpy (char *__restrict __dest, const char *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2))) ;
// [SYSTEM: /usr/include/string.h]
extern size_t strlcat (char *__restrict __dest, const char *__restrict __src, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2))) ;
// [LOCAL: ./url.h]
char *URL_SCHEMES[] = {
  "aaa", "aaas", "about", "acap", "acct", "adiumxtra", "afp", "afs", "aim", "apt", "attachment", "aw", "beshare", "bitcoin", "bolo", "callto", "cap", "chrome", "crome-extension", "com-evenbrite-attendee", "cid", "coap", "coaps","content", "crid", "cvs", "data", "dav", "dict", "lna-playsingle", "dln-playcontainer", "dns", "dtn", "dvb", "ed2k", "facetime", "fax", "feed", "file", "finger", "fish","ftp", "geo", "gg","git", "gizmoproject", "go", "gopher", "gtalk", "h323", "hcp", "http", "https", "iax", "icap", "icon","im", "imap", "info", "ipn", "ipp", "irc", "irc6", "ircs", "iris", "iris.beep", "iris.xpc", "iris.xpcs","iris.lws", "itms", "jabber", "jar", "jms", "keyparc", "lastfm", "ldap", "ldaps", "magnet", "mailserver","mailto", "maps", "market", "message", "mid", "mms", "modem", "ms-help", "mssettings-power", "msnim", "msrp", "msrps", "mtqp", "mumble", "mupdate", "mvn", "news", "nfs", "ni", "nih", "nntp", "notes","oid", "paquelocktoken", "pack", "palm", "paparazzi", "pkcs11", "platform", "pop", "pres", "prospero", "proxy", "psyc","query", "reload", "res", "resource", "rmi", "rsync", "rtmp","rtsp", "secondlife", "service","session", "sftp", "sgn", "shttp", "sieve", "sip", "sips", "skype", "smb", "sms", "snews", "snmp", "soap.beep","soap.beeps", "soldat", "spotify", "ssh", "steam", "svn", "tag", "teamspeak", "tel", "telnet", "tftp", "things","thismessage", "tn3270", "tip", "tv", "udp", "unreal", "urn", "ut2004", "vemmi","ventrilo", "videotex", "view-source", "wais","webcal", "ws", "wss", "wtai", "wyciwyg", "xcon", "xcon-userid", "xfire","xmlrpc.beep", "xmlrpc.beeps", "xmpp", "xri","ymsgr", "javascript", "jdbc", "doi" };
// [LOCAL: ./url.h]
typedef struct url_data {
  char *href;
  char *protocol;
  char *host;
  char *auth;
  char *hostname;
  char *pathname;
  char *search;
  char *path;
  char *hash;
  char *query;
  char *port;
} url_data_t;
// [LOCAL: ./url.h]
url_data_t * url_parse (char *url);
// [LOCAL: ./url.h]
char * url_get_protocol (char *url);
// [LOCAL: ./url.h]
char * url_get_auth (char *url);
// [LOCAL: ./url.h]
char * url_get_hostname (char *url);
// [LOCAL: ./url.h]
char * url_get_host (char *url);
// [LOCAL: ./url.h]
char * url_get_pathname (char *url);
// [LOCAL: ./url.h]
char * url_get_path (char *url);
// [LOCAL: ./url.h]
char * url_get_search (char *url);
// [LOCAL: ./url.h]
char * url_get_query (char *url);
// [LOCAL: ./url.h]
char * url_get_hash (char *url);
// [LOCAL: ./url.h]
char * url_get_port (char *url);
// [LOCAL: ./url.h]
void url_free (url_data_t *data);
// [LOCAL: ./url.h]
_Bool url_is_protocol (char *str);
// [LOCAL: ./url.h]
_Bool url_is_ssh (char *str);
// [LOCAL: ./url.h]
void url_inspect (char *url);
// [LOCAL: ./url.h]
void url_data_inspect (url_data_t *data);
// [LOCAL: ./url.h]
char * strdup (const char *str) {
  int n = strlen(str) + 1;
  char *dup = malloc(n);
  if (dup) strcpy(dup, str);
  return dup;
}
// [LOCAL: ./url.h]
static char * strff (char *ptr, int n) {
  int y = 0;
  for (int i = 0; i < n; ++i) {
    y = *ptr++;
  }
  return strdup(ptr);
}
// [LOCAL: ./url.h]
static char * strrwd (char *ptr, int n) {
  int y = 0;
  for (int i = 0; i < n; ++i) {
    y = *ptr--;
  }
  return strdup(ptr);
}
// [LOCAL: ./url.h]
static char * get_part (char *url, const char *format, int l) {
  _Bool has = 0;
  char *tmp = malloc(sizeof(char));
  char *tmp_url = strdup(url);
  char *fmt_url = strdup(url);
  char *ret = malloc(sizeof(char));
  if (!tmp || !tmp_url || !fmt_url || !ret) return ((void*)0);
  strcpy(tmp, "");
  strcpy(fmt_url, "");
  fmt_url = strff(fmt_url, l);
  sscanf(fmt_url, format, tmp);
  if (0 != strcmp(tmp, tmp_url)) {
    has = 1;
    ret = strdup(tmp);
  }
  fmt_url = strrwd(fmt_url, l);
  free(tmp);
  free(tmp_url);
  free(fmt_url);
  return has? ret : ((void*)0);
}
// [LOCAL: ./url.h]
url_data_t * url_parse (char *url) {
  url_data_t *data = malloc(sizeof(url_data_t));
  if (!data) return ((void*)0);
  data->href = url;
  char *tmp;
  char *tmp_url = strdup(url);
  _Bool is_ssh = 0;
  char *protocol = url_get_protocol(tmp_url);
  if (!protocol) return ((void*)0);
  int protocol_len = (int) strlen(protocol) + 3;
  data->protocol = protocol;
  is_ssh = url_is_ssh(protocol);
  char *auth = malloc(sizeof(char));
  int auth_len = 0;
  if ((tmp = strstr(tmp_url, "@"))) {
    auth = get_part(tmp_url, "%[^@]", protocol_len);
    auth_len = strlen(auth);
    if (auth) auth_len++;
  }
  data->auth = auth;
  char *hostname;
  hostname = (is_ssh) ? get_part(tmp_url, "%[^:]", protocol_len + auth_len) : get_part(tmp_url, "%[^/]", protocol_len + auth_len);
  if (!hostname) return ((void*)0);
  int hostname_len = (int) strlen(hostname);
  char *tmp_hostname = strdup(hostname);
  data->hostname = hostname;
  char *host = malloc(strlen(tmp_hostname) * sizeof(char));
  sscanf(tmp_hostname, "%[^:]", host);
  if (!host) return ((void*)0);
  int host_len = (int) strlen(host);
  data->host = host;
  char *tmp_path;
  tmp_path = (is_ssh) ? get_part(tmp_url, ":%s", protocol_len + auth_len + hostname_len) : get_part(tmp_url, "/%s", protocol_len + auth_len + hostname_len);
  char *path = malloc(strlen(tmp_path) * sizeof(char));
  if (!path) return ((void*)0);
  char *fmt = (is_ssh)? "%s" : "/%s";
  sprintf(path, fmt, tmp_path);
  data->path = path;
  free(tmp_path);
  char *pathname = malloc(sizeof(char));
  if (!pathname) return ((void*)0);
  strcat(pathname, "");
  tmp_path = strdup(path);
  sscanf(tmp_path, "%[^? | ^#]", pathname);
  int pathname_len = strlen(pathname);
  data->pathname = pathname;
  char *search = malloc(sizeof(search));
  if (!search) return ((void*)0);
  tmp_path = strff(tmp_path, pathname_len);
  strcat(search, "");
  sscanf(tmp_path, "%[^#]", search);
  data->search = search;
  int search_len = strlen(search);
  free(tmp_path);
  char *query = malloc(sizeof(char));
  if (!query) return ((void*)0);
  sscanf(search, "?%s", query);
  data->query = query;
  char *hash = malloc(sizeof(char));
  if (!hash) return ((void*)0);
  tmp_path = strff(path, pathname_len + search_len);
  strcat(hash, "");
  sscanf(tmp_path, "%s", hash);
  data->hash = hash;
  free(tmp_path);
  char *port = malloc(sizeof(char));
  if (!port) return ((void*)0);
  tmp_hostname = strff(hostname, host_len + 1);
  sscanf(tmp_hostname, "%s", port);
  data->port = port;
  free(tmp_hostname);
  return data;
}
// [LOCAL: ./url.h]
_Bool url_is_protocol (char *str) {
  int count = sizeof(URL_SCHEMES) / sizeof(URL_SCHEMES[0]);
  for (int i = 0; i < count; ++i) {
    if (0 == strcmp(URL_SCHEMES[i], str)) {
      return 1;
    }
  }
  return 0;
}
// [LOCAL: ./url.h]
_Bool url_is_ssh (char *str) {
  str = strdup(str);
  if (0 == strcmp(str, "ssh") || 0 == strcmp(str, "git")) {
    free(str);
    return 1;
  }
  return 0;
}
// [LOCAL: ./url.h]
char * url_get_protocol (char *url) {
  char *protocol = malloc(16 * sizeof(char));
  if (!protocol) return ((void*)0);
  sscanf(url, "%[^://]", protocol);
  if (url_is_protocol(protocol)) return protocol;
  return ((void*)0);
}
// [LOCAL: ./url.h]
char * url_get_auth (char *url) {
  char *protocol = url_get_protocol(url);
  if (!protocol) return ((void*)0);
  int l = (int) strlen(protocol) + 3;
  return get_part(url, "%[^@]", l);
}
// [LOCAL: ./url.h]
char * url_get_hostname (char *url) {
  int l = 3;
  char *protocol = url_get_protocol(url);
  char *tmp_protocol = strdup(protocol);
  char *auth = url_get_auth(url);
  if (!protocol) return ((void*)0);
  if (auth) l += strlen(auth) + 1;
  if (auth) free(auth);
  l += (int) strlen(protocol);
  free(protocol);
  char * hostname = url_is_ssh(tmp_protocol) ? get_part(url, "%[^:]", l) : get_part(url, "%[^/]", l);
  free(tmp_protocol);
  return hostname;
}
// [LOCAL: ./url.h]
char * url_get_host (char *url) {
  char *host = malloc(sizeof(char));
  char *hostname = url_get_hostname(url);
  if (!host || !hostname) return ((void*)0);
  sscanf(hostname, "%[^:]", host);
  free(hostname);
  return host;
}
// [LOCAL: ./url.h]
char * url_get_pathname (char *url) {
  char *path = url_get_path(url);
  char *pathname = malloc(sizeof(char));
  if (!path || !pathname) return ((void*)0);
  strcat(pathname, "");
  sscanf(path, "%[^?]", pathname);
  free(path);
  return pathname;
}
// [LOCAL: ./url.h]
char * url_get_path (char *url) {
  int l = 3;
  char *tmp_path;
  char *protocol = url_get_protocol(url);
  char *auth = url_get_auth(url);
  char *hostname = url_get_hostname(url);
  if (!protocol || !hostname) return ((void*)0);
  _Bool is_ssh = url_is_ssh(protocol);
  l += (int) strlen(protocol) + (int) strlen(hostname);
  if (auth) l+= (int) strlen(auth) +1;
  tmp_path = (is_ssh) ? get_part(url, ":%s", l) : get_part(url, "/%s", l);
  char *fmt = (is_ssh)? "%s" : "/%s";
  char *path = malloc(strlen(tmp_path) * sizeof(char));
  sprintf(path, fmt, tmp_path);
  if (auth) free(auth);
  free(protocol);
  free(hostname);
  free(tmp_path);
  return path;
}
// [LOCAL: ./url.h]
char * url_get_search (char *url) {
  char *path = url_get_path(url);
  char *pathname = url_get_pathname(url);
  char *search = malloc(sizeof(char));
  if (!path || !search) return ((void*)0);
  char *tmp_path = strff(path, (int)strlen(pathname));
  strcat(search, "");
  sscanf(tmp_path, "%[^#]", search);
  tmp_path = strrwd(tmp_path, (int)strlen(pathname));
  free(path);
  free(pathname);
  return search;
}
// [LOCAL: ./url.h]
char * url_get_query (char *url) {
  char *search = url_get_search(url);
  char *query = malloc(sizeof(char));
  if (!search) return ((void*)0);
  sscanf(search, "?%s", query);
  free(search);
  return query;
}
// [LOCAL: ./url.h]
char * url_get_hash (char *url) {
  char *hash = malloc(sizeof(char));
  if (!hash) return ((void*)0);
  char *path = url_get_path(url);
  if (!path) return ((void*)0);
  char *pathname = url_get_pathname(url);
  if (!pathname) return ((void*)0);
  char *search = url_get_search(url);
  int pathname_len = (int) strlen(pathname);
  int search_len = (int) strlen(search);
  char *tmp_path = strff(path, pathname_len + search_len);
  strcat(hash, "");
  sscanf(tmp_path, "%s", hash);
  tmp_path = strrwd(tmp_path, pathname_len + search_len);
  free(tmp_path);
  free(pathname);
  free(path);
  if (search) free(search);
  return hash;
}
// [LOCAL: ./url.h]
char * url_get_port (char *url) {
  char *port = malloc(sizeof(char));
  char *hostname = url_get_hostname(url);
  char *host = url_get_host(url);
  if (!port || !hostname) return ((void*)0);
  char *tmp_hostname = strff(hostname, strlen(host) +1);
  sscanf(tmp_hostname, "%s", port);
  free(hostname);
  free(tmp_hostname);
  return port;
}
// [LOCAL: ./url.h]
void url_inspect (char *url) {
  url_data_inspect(url_parse(url));
}
// [LOCAL: ./url.h]
void url_data_inspect (url_data_t *data) {
  printf("#url =>\n");
  printf("    .href: \"%s\"\n", data->href);
  printf("    .protocol: \"%s\"\n", data->protocol);
  printf("    .host: \"%s\"\n", data->host);
  printf("    .auth: \"%s\"\n", data->auth);
  printf("    .hostname: \"%s\"\n", data->hostname);
  printf("    .pathname: \"%s\"\n", data->pathname);
  printf("    .search: \"%s\"\n", data->search);
  printf("    .path: \"%s\"\n", data->path);
  printf("    .hash: \"%s\"\n", data->hash);
  printf("    .query: \"%s\"\n", data->query);
  printf("    .port: \"%s\"\n", data->port);
}
// [LOCAL: ./url.h]
void url_free (url_data_t *data) {
  if (!data) return;
  if (data->auth) free(data->auth);
  if (data->protocol) free(data->protocol);
  if (data->hostname) free(data->hostname);
  if (data->host) free(data->host);
  if (data->pathname) free(data->pathname);
  if (data->path) free(data->path);
  if (data->hash) free(data->hash);
  if (data->search) free(data->search);
  if (data->query) free(data->query);
}
// [SYSTEM: /usr/include/assert.h]
extern void __assert_fail (const char *__assertion, const char *__file, unsigned int __line, const char *__function) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));
// [SYSTEM: /usr/include/assert.h]
extern void __assert_perror_fail (int __errnum, const char *__file, unsigned int __line, const char *__function) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));
// [SYSTEM: /usr/include/assert.h]
extern void __assert (const char *__assertion, const char *__file, int __line) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));
// [LOCAL: test.c]
int main (void) {
  char *gh_url = "git://git@github.com:jwerle/url.h.git";
  char *url = "http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash";
  url_data_t *parsed = url_parse(url);
  url_data_t *gh_parsed = url_parse(gh_url);
  ((void) sizeof ((parsed) ? 1 : 0), __extension__ ({ if (parsed) ; else __assert_fail ("parsed", "test.c", 15, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((gh_parsed) ? 1 : 0), __extension__ ({ if (gh_parsed) ; else __assert_fail ("gh_parsed", "test.c", 16, __extension__ __PRETTY_FUNCTION__); }));
  url_data_inspect(parsed);
  url_data_inspect(gh_parsed);
  ((void) sizeof ((parsed->href) ? 1 : 0), __extension__ ({ if (parsed->href) ; else __assert_fail ("parsed->href", "test.c", 21, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->auth) ? 1 : 0), __extension__ ({ if (parsed->auth) ; else __assert_fail ("parsed->auth", "test.c", 22, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->protocol) ? 1 : 0), __extension__ ({ if (parsed->protocol) ; else __assert_fail ("parsed->protocol", "test.c", 23, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->port) ? 1 : 0), __extension__ ({ if (parsed->port) ; else __assert_fail ("parsed->port", "test.c", 24, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->hostname) ? 1 : 0), __extension__ ({ if (parsed->hostname) ; else __assert_fail ("parsed->hostname", "test.c", 25, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->host) ? 1 : 0), __extension__ ({ if (parsed->host) ; else __assert_fail ("parsed->host", "test.c", 26, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->pathname) ? 1 : 0), __extension__ ({ if (parsed->pathname) ; else __assert_fail ("parsed->pathname", "test.c", 27, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->path) ? 1 : 0), __extension__ ({ if (parsed->path) ; else __assert_fail ("parsed->path", "test.c", 28, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->hash) ? 1 : 0), __extension__ ({ if (parsed->hash) ; else __assert_fail ("parsed->hash", "test.c", 29, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->search) ? 1 : 0), __extension__ ({ if (parsed->search) ; else __assert_fail ("parsed->search", "test.c", 30, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((parsed->query) ? 1 : 0), __extension__ ({ if (parsed->query) ; else __assert_fail ("parsed->query", "test.c", 31, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((gh_parsed->href) ? 1 : 0), __extension__ ({ if (gh_parsed->href) ; else __assert_fail ("gh_parsed->href", "test.c", 33, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((gh_parsed->protocol) ? 1 : 0), __extension__ ({ if (gh_parsed->protocol) ; else __assert_fail ("gh_parsed->protocol", "test.c", 34, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((gh_parsed->host) ? 1 : 0), __extension__ ({ if (gh_parsed->host) ; else __assert_fail ("gh_parsed->host", "test.c", 35, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((gh_parsed->auth) ? 1 : 0), __extension__ ({ if (gh_parsed->auth) ; else __assert_fail ("gh_parsed->auth", "test.c", 36, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((gh_parsed->hostname) ? 1 : 0), __extension__ ({ if (gh_parsed->hostname) ; else __assert_fail ("gh_parsed->hostname", "test.c", 37, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((gh_parsed->pathname) ? 1 : 0), __extension__ ({ if (gh_parsed->pathname) ; else __assert_fail ("gh_parsed->pathname", "test.c", 38, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((gh_parsed->path) ? 1 : 0), __extension__ ({ if (gh_parsed->path) ; else __assert_fail ("gh_parsed->path", "test.c", 39, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((url_is_protocol("http")) ? 1 : 0), __extension__ ({ if (url_is_protocol("http")) ; else __assert_fail ("url_is_protocol(\"http\")", "test.c", 41, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((url_is_protocol("https")) ? 1 : 0), __extension__ ({ if (url_is_protocol("https")) ; else __assert_fail ("url_is_protocol(\"https\")", "test.c", 42, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((url_is_protocol("git")) ? 1 : 0), __extension__ ({ if (url_is_protocol("git")) ; else __assert_fail ("url_is_protocol(\"git\")", "test.c", 43, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((url_is_protocol("ssh")) ? 1 : 0), __extension__ ({ if (url_is_protocol("ssh")) ; else __assert_fail ("url_is_protocol(\"ssh\")", "test.c", 44, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((url_is_protocol("sftp")) ? 1 : 0), __extension__ ({ if (url_is_protocol("sftp")) ; else __assert_fail ("url_is_protocol(\"sftp\")", "test.c", 45, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((url_is_protocol("ftp")) ? 1 : 0), __extension__ ({ if (url_is_protocol("ftp")) ; else __assert_fail ("url_is_protocol(\"ftp\")", "test.c", 46, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((url_is_protocol("javascript")) ? 1 : 0), __extension__ ({ if (url_is_protocol("javascript")) ; else __assert_fail ("url_is_protocol(\"javascript\")", "test.c", 47, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("http", url_get_protocol(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("http", url_get_protocol(url))) ; else __assert_fail ("0 == strcmp(\"http\", url_get_protocol(url))", "test.c", 49, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("user:pass", url_get_auth(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("user:pass", url_get_auth(url))) ; else __assert_fail ("0 == strcmp(\"user:pass\", url_get_auth(url))", "test.c", 50, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("subdomain.host.com:8080", url_get_hostname(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("subdomain.host.com:8080", url_get_hostname(url))) ; else __assert_fail ("0 == strcmp(\"subdomain.host.com:8080\", url_get_hostname(url))", "test.c", 51, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("subdomain.host.com", url_get_host(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("subdomain.host.com", url_get_host(url))) ; else __assert_fail ("0 == strcmp(\"subdomain.host.com\", url_get_host(url))", "test.c", 52, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("/p/a/t/h", url_get_pathname(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("/p/a/t/h", url_get_pathname(url))) ; else __assert_fail ("0 == strcmp(\"/p/a/t/h\", url_get_pathname(url))", "test.c", 53, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("/p/a/t/h?query=string#hash", url_get_path(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("/p/a/t/h?query=string#hash", url_get_path(url))) ; else __assert_fail ("0 == strcmp(\"/p/a/t/h?query=string#hash\", url_get_path(url))", "test.c", 54, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("?query=string", url_get_search(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("?query=string", url_get_search(url))) ; else __assert_fail ("0 == strcmp(\"?query=string\", url_get_search(url))", "test.c", 55, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("query=string", url_get_query(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("query=string", url_get_query(url))) ; else __assert_fail ("0 == strcmp(\"query=string\", url_get_query(url))", "test.c", 56, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("#hash", url_get_hash(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("#hash", url_get_hash(url))) ; else __assert_fail ("0 == strcmp(\"#hash\", url_get_hash(url))", "test.c", 57, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("8080", url_get_port(url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("8080", url_get_port(url))) ; else __assert_fail ("0 == strcmp(\"8080\", url_get_port(url))", "test.c", 58, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("git", url_get_protocol(gh_url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("git", url_get_protocol(gh_url))) ; else __assert_fail ("0 == strcmp(\"git\", url_get_protocol(gh_url))", "test.c", 60, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("github.com", url_get_host(gh_url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("github.com", url_get_host(gh_url))) ; else __assert_fail ("0 == strcmp(\"github.com\", url_get_host(gh_url))", "test.c", 61, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("github.com", url_get_hostname(gh_url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("github.com", url_get_hostname(gh_url))) ; else __assert_fail ("0 == strcmp(\"github.com\", url_get_hostname(gh_url))", "test.c", 62, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("git", url_get_auth(gh_url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("git", url_get_auth(gh_url))) ; else __assert_fail ("0 == strcmp(\"git\", url_get_auth(gh_url))", "test.c", 63, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("jwerle/url.h.git", url_get_pathname(gh_url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("jwerle/url.h.git", url_get_pathname(gh_url))) ; else __assert_fail ("0 == strcmp(\"jwerle/url.h.git\", url_get_pathname(gh_url))", "test.c", 64, __extension__ __PRETTY_FUNCTION__); }));
  ((void) sizeof ((0 == strcmp("jwerle/url.h.git", url_get_path(gh_url))) ? 1 : 0), __extension__ ({ if (0 == strcmp("jwerle/url.h.git", url_get_path(gh_url))) ; else __assert_fail ("0 == strcmp(\"jwerle/url.h.git\", url_get_path(gh_url))", "test.c", 65, __extension__ __PRETTY_FUNCTION__); }));
  url_free(parsed);
  return 0;
}