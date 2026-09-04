// [SYSTEM: /usr/lib/llvm-21/lib/clang/21/include/__stddef_size_t.h]
typedef long unsigned int size_t;
// [SYSTEM: /usr/lib/llvm-21/lib/clang/21/include/__stddef_wchar_t.h]
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
// [SYSTEM: /usr/lib/llvm-21/lib/clang/21/include/__stdarg___gnuc_va_list.h]
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
// [SYSTEM: /usr/include/ctype.h]
enum {
  _ISupper = ((0) < 8 ? ((1 << (0)) << 8) : ((1 << (0)) >> 8)), _ISlower = ((1) < 8 ? ((1 << (1)) << 8) : ((1 << (1)) >> 8)), _ISalpha = ((2) < 8 ? ((1 << (2)) << 8) : ((1 << (2)) >> 8)), _ISdigit = ((3) < 8 ? ((1 << (3)) << 8) : ((1 << (3)) >> 8)), _ISxdigit = ((4) < 8 ? ((1 << (4)) << 8) : ((1 << (4)) >> 8)), _ISspace = ((5) < 8 ? ((1 << (5)) << 8) : ((1 << (5)) >> 8)), _ISprint = ((6) < 8 ? ((1 << (6)) << 8) : ((1 << (6)) >> 8)), _ISgraph = ((7) < 8 ? ((1 << (7)) << 8) : ((1 << (7)) >> 8)), _ISblank = ((8) < 8 ? ((1 << (8)) << 8) : ((1 << (8)) >> 8)), _IScntrl = ((9) < 8 ? ((1 << (9)) << 8) : ((1 << (9)) >> 8)), _ISpunct = ((10) < 8 ? ((1 << (10)) << 8) : ((1 << (10)) >> 8)), _ISalnum = ((11) < 8 ? ((1 << (11)) << 8) : ((1 << (11)) >> 8)) };
// [SYSTEM: /usr/include/ctype.h]
extern const unsigned short int **__ctype_b_loc (void) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/ctype.h]
extern const __int32_t **__ctype_tolower_loc (void) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/ctype.h]
extern const __int32_t **__ctype_toupper_loc (void) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/ctype.h]
extern int isalnum (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isalpha (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int iscntrl (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isdigit (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int islower (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isgraph (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isprint (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int ispunct (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isspace (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isupper (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isxdigit (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int tolower (int __c) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int toupper (int __c) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isblank (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isascii (int __c) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int toascii (int __c) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int _toupper (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int _tolower (int) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isalnum_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isalpha_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int iscntrl_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isdigit_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int islower_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isgraph_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isprint_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int ispunct_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isspace_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isupper_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isxdigit_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int isblank_l (int, locale_t) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int __tolower_l (int __c, locale_t __l) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int tolower_l (int __c, locale_t __l) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int __toupper_l (int __c, locale_t __l) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/ctype.h]
extern int toupper_l (int __c, locale_t __l) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/math.h]
typedef float float_t;
// [SYSTEM: /usr/include/math.h]
typedef double double_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __fpclassify (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __signbit (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __isinf (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __finite (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __isnan (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __iseqsig (double __x, double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __issignaling (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double asin (double __x) __attribute__ ((__nothrow__ )); extern double __asin (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double atan (double __x) __attribute__ ((__nothrow__ )); extern double __atan (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double atan2 (double __y, double __x) __attribute__ ((__nothrow__ )); extern double __atan2 (double __y, double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double cos (double __x) __attribute__ ((__nothrow__ )); extern double __cos (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double sin (double __x) __attribute__ ((__nothrow__ )); extern double __sin (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double tan (double __x) __attribute__ ((__nothrow__ )); extern double __tan (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double cosh (double __x) __attribute__ ((__nothrow__ )); extern double __cosh (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double sinh (double __x) __attribute__ ((__nothrow__ )); extern double __sinh (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double tanh (double __x) __attribute__ ((__nothrow__ )); extern double __tanh (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double acosh (double __x) __attribute__ ((__nothrow__ )); extern double __acosh (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double asinh (double __x) __attribute__ ((__nothrow__ )); extern double __asinh (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double atanh (double __x) __attribute__ ((__nothrow__ )); extern double __atanh (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double exp (double __x) __attribute__ ((__nothrow__ )); extern double __exp (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double frexp (double __x, int *__exponent) __attribute__ ((__nothrow__ )); extern double __frexp (double __x, int *__exponent) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double ldexp (double __x, int __exponent) __attribute__ ((__nothrow__ )); extern double __ldexp (double __x, int __exponent) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double log (double __x) __attribute__ ((__nothrow__ )); extern double __log (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double log10 (double __x) __attribute__ ((__nothrow__ )); extern double __log10 (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double modf (double __x, double *__iptr) __attribute__ ((__nothrow__ )); extern double __modf (double __x, double *__iptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double expm1 (double __x) __attribute__ ((__nothrow__ )); extern double __expm1 (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double log1p (double __x) __attribute__ ((__nothrow__ )); extern double __log1p (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double logb (double __x) __attribute__ ((__nothrow__ )); extern double __logb (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double exp2 (double __x) __attribute__ ((__nothrow__ )); extern double __exp2 (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double log2 (double __x) __attribute__ ((__nothrow__ )); extern double __log2 (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double pow (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __pow (double __x, double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double sqrt (double __x) __attribute__ ((__nothrow__ )); extern double __sqrt (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double hypot (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __hypot (double __x, double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double cbrt (double __x) __attribute__ ((__nothrow__ )); extern double __cbrt (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double ceil (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double fabs (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double floor (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double fmod (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __fmod (double __x, double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int isinf (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int finite (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double drem (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __drem (double __x, double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double significand (double __x) __attribute__ ((__nothrow__ )); extern double __significand (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double copysign (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double nan (const char *__tagb) __attribute__ ((__nothrow__ )); extern double __nan (const char *__tagb) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int isnan (double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double j0 (double) __attribute__ ((__nothrow__ )); extern double __j0 (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double j1 (double) __attribute__ ((__nothrow__ )); extern double __j1 (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double jn (int, double) __attribute__ ((__nothrow__ )); extern double __jn (int, double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double y0 (double) __attribute__ ((__nothrow__ )); extern double __y0 (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double y1 (double) __attribute__ ((__nothrow__ )); extern double __y1 (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double yn (int, double) __attribute__ ((__nothrow__ )); extern double __yn (int, double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double erf (double) __attribute__ ((__nothrow__ )); extern double __erf (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern double erfc (double) __attribute__ ((__nothrow__ )); extern double __erfc (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double lgamma (double) __attribute__ ((__nothrow__ )); extern double __lgamma (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double tgamma (double) __attribute__ ((__nothrow__ )); extern double __tgamma (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double gamma (double) __attribute__ ((__nothrow__ )); extern double __gamma (double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double lgamma_r (double, int *__signgamp) __attribute__ ((__nothrow__ )); extern double __lgamma_r (double, int *__signgamp) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double rint (double __x) __attribute__ ((__nothrow__ )); extern double __rint (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double nextafter (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __nextafter (double __x, double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double nexttoward (double __x, long double __y) __attribute__ ((__nothrow__ )); extern double __nexttoward (double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double remainder (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __remainder (double __x, double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double scalbn (double __x, int __n) __attribute__ ((__nothrow__ )); extern double __scalbn (double __x, int __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int ilogb (double __x) __attribute__ ((__nothrow__ )); extern int __ilogb (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double scalbln (double __x, long int __n) __attribute__ ((__nothrow__ )); extern double __scalbln (double __x, long int __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double nearbyint (double __x) __attribute__ ((__nothrow__ )); extern double __nearbyint (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double round (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double trunc (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double remquo (double __x, double __y, int *__quo) __attribute__ ((__nothrow__ )); extern double __remquo (double __x, double __y, int *__quo) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long int lrint (double __x) __attribute__ ((__nothrow__ )); extern long int __lrint (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
__extension__ extern long long int llrint (double __x) __attribute__ ((__nothrow__ )); extern long long int __llrint (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long int lround (double __x) __attribute__ ((__nothrow__ )); extern long int __lround (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
__extension__ extern long long int llround (double __x) __attribute__ ((__nothrow__ )); extern long long int __llround (double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double fdim (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __fdim (double __x, double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double fmax (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double fmin (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double fma (double __x, double __y, double __z) __attribute__ ((__nothrow__ )); extern double __fma (double __x, double __y, double __z) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern double scalb (double __x, double __n) __attribute__ ((__nothrow__ )); extern double __scalb (double __x, double __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __fpclassifyf (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __signbitf (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __isinff (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __finitef (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __isnanf (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __iseqsigf (float __x, float __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __issignalingf (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float asinf (float __x) __attribute__ ((__nothrow__ )); extern float __asinf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float atanf (float __x) __attribute__ ((__nothrow__ )); extern float __atanf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float atan2f (float __y, float __x) __attribute__ ((__nothrow__ )); extern float __atan2f (float __y, float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float cosf (float __x) __attribute__ ((__nothrow__ )); extern float __cosf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float sinf (float __x) __attribute__ ((__nothrow__ )); extern float __sinf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float tanf (float __x) __attribute__ ((__nothrow__ )); extern float __tanf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float coshf (float __x) __attribute__ ((__nothrow__ )); extern float __coshf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float sinhf (float __x) __attribute__ ((__nothrow__ )); extern float __sinhf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float tanhf (float __x) __attribute__ ((__nothrow__ )); extern float __tanhf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float acoshf (float __x) __attribute__ ((__nothrow__ )); extern float __acoshf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float asinhf (float __x) __attribute__ ((__nothrow__ )); extern float __asinhf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float atanhf (float __x) __attribute__ ((__nothrow__ )); extern float __atanhf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float expf (float __x) __attribute__ ((__nothrow__ )); extern float __expf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float frexpf (float __x, int *__exponent) __attribute__ ((__nothrow__ )); extern float __frexpf (float __x, int *__exponent) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float ldexpf (float __x, int __exponent) __attribute__ ((__nothrow__ )); extern float __ldexpf (float __x, int __exponent) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float logf (float __x) __attribute__ ((__nothrow__ )); extern float __logf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float log10f (float __x) __attribute__ ((__nothrow__ )); extern float __log10f (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float modff (float __x, float *__iptr) __attribute__ ((__nothrow__ )); extern float __modff (float __x, float *__iptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float expm1f (float __x) __attribute__ ((__nothrow__ )); extern float __expm1f (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float log1pf (float __x) __attribute__ ((__nothrow__ )); extern float __log1pf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float logbf (float __x) __attribute__ ((__nothrow__ )); extern float __logbf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float exp2f (float __x) __attribute__ ((__nothrow__ )); extern float __exp2f (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float log2f (float __x) __attribute__ ((__nothrow__ )); extern float __log2f (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float powf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __powf (float __x, float __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float sqrtf (float __x) __attribute__ ((__nothrow__ )); extern float __sqrtf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float hypotf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __hypotf (float __x, float __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float cbrtf (float __x) __attribute__ ((__nothrow__ )); extern float __cbrtf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float ceilf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float fabsf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float floorf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float fmodf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __fmodf (float __x, float __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int isinff (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int finitef (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float dremf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __dremf (float __x, float __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float significandf (float __x) __attribute__ ((__nothrow__ )); extern float __significandf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float copysignf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float nanf (const char *__tagb) __attribute__ ((__nothrow__ )); extern float __nanf (const char *__tagb) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int isnanf (float __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float j0f (float) __attribute__ ((__nothrow__ )); extern float __j0f (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float j1f (float) __attribute__ ((__nothrow__ )); extern float __j1f (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float jnf (int, float) __attribute__ ((__nothrow__ )); extern float __jnf (int, float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float y0f (float) __attribute__ ((__nothrow__ )); extern float __y0f (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float y1f (float) __attribute__ ((__nothrow__ )); extern float __y1f (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float ynf (int, float) __attribute__ ((__nothrow__ )); extern float __ynf (int, float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float erff (float) __attribute__ ((__nothrow__ )); extern float __erff (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern float erfcf (float) __attribute__ ((__nothrow__ )); extern float __erfcf (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float lgammaf (float) __attribute__ ((__nothrow__ )); extern float __lgammaf (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float tgammaf (float) __attribute__ ((__nothrow__ )); extern float __tgammaf (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float gammaf (float) __attribute__ ((__nothrow__ )); extern float __gammaf (float) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float lgammaf_r (float, int *__signgamp) __attribute__ ((__nothrow__ )); extern float __lgammaf_r (float, int *__signgamp) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float rintf (float __x) __attribute__ ((__nothrow__ )); extern float __rintf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float nextafterf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __nextafterf (float __x, float __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float nexttowardf (float __x, long double __y) __attribute__ ((__nothrow__ )); extern float __nexttowardf (float __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float remainderf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __remainderf (float __x, float __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float scalbnf (float __x, int __n) __attribute__ ((__nothrow__ )); extern float __scalbnf (float __x, int __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int ilogbf (float __x) __attribute__ ((__nothrow__ )); extern int __ilogbf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float scalblnf (float __x, long int __n) __attribute__ ((__nothrow__ )); extern float __scalblnf (float __x, long int __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float nearbyintf (float __x) __attribute__ ((__nothrow__ )); extern float __nearbyintf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float roundf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float truncf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float remquof (float __x, float __y, int *__quo) __attribute__ ((__nothrow__ )); extern float __remquof (float __x, float __y, int *__quo) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long int lrintf (float __x) __attribute__ ((__nothrow__ )); extern long int __lrintf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
__extension__ extern long long int llrintf (float __x) __attribute__ ((__nothrow__ )); extern long long int __llrintf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long int lroundf (float __x) __attribute__ ((__nothrow__ )); extern long int __lroundf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
__extension__ extern long long int llroundf (float __x) __attribute__ ((__nothrow__ )); extern long long int __llroundf (float __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float fdimf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __fdimf (float __x, float __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float fmaxf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float fminf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float fmaf (float __x, float __y, float __z) __attribute__ ((__nothrow__ )); extern float __fmaf (float __x, float __y, float __z) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern float scalbf (float __x, float __n) __attribute__ ((__nothrow__ )); extern float __scalbf (float __x, float __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __fpclassifyl (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __signbitl (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __isinfl (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __finitel (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __isnanl (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __iseqsigl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __issignalingl (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double asinl (long double __x) __attribute__ ((__nothrow__ )); extern long double __asinl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double atanl (long double __x) __attribute__ ((__nothrow__ )); extern long double __atanl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double atan2l (long double __y, long double __x) __attribute__ ((__nothrow__ )); extern long double __atan2l (long double __y, long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double cosl (long double __x) __attribute__ ((__nothrow__ )); extern long double __cosl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double sinl (long double __x) __attribute__ ((__nothrow__ )); extern long double __sinl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double tanl (long double __x) __attribute__ ((__nothrow__ )); extern long double __tanl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double coshl (long double __x) __attribute__ ((__nothrow__ )); extern long double __coshl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double sinhl (long double __x) __attribute__ ((__nothrow__ )); extern long double __sinhl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double tanhl (long double __x) __attribute__ ((__nothrow__ )); extern long double __tanhl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double acoshl (long double __x) __attribute__ ((__nothrow__ )); extern long double __acoshl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double asinhl (long double __x) __attribute__ ((__nothrow__ )); extern long double __asinhl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double atanhl (long double __x) __attribute__ ((__nothrow__ )); extern long double __atanhl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double expl (long double __x) __attribute__ ((__nothrow__ )); extern long double __expl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double frexpl (long double __x, int *__exponent) __attribute__ ((__nothrow__ )); extern long double __frexpl (long double __x, int *__exponent) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double ldexpl (long double __x, int __exponent) __attribute__ ((__nothrow__ )); extern long double __ldexpl (long double __x, int __exponent) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double logl (long double __x) __attribute__ ((__nothrow__ )); extern long double __logl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double log10l (long double __x) __attribute__ ((__nothrow__ )); extern long double __log10l (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double modfl (long double __x, long double *__iptr) __attribute__ ((__nothrow__ )); extern long double __modfl (long double __x, long double *__iptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double expm1l (long double __x) __attribute__ ((__nothrow__ )); extern long double __expm1l (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double log1pl (long double __x) __attribute__ ((__nothrow__ )); extern long double __log1pl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double logbl (long double __x) __attribute__ ((__nothrow__ )); extern long double __logbl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double exp2l (long double __x) __attribute__ ((__nothrow__ )); extern long double __exp2l (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double log2l (long double __x) __attribute__ ((__nothrow__ )); extern long double __log2l (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double powl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __powl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double sqrtl (long double __x) __attribute__ ((__nothrow__ )); extern long double __sqrtl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double hypotl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __hypotl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double cbrtl (long double __x) __attribute__ ((__nothrow__ )); extern long double __cbrtl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double ceill (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double fabsl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double floorl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double fmodl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __fmodl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int isinfl (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int finitel (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double dreml (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __dreml (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double significandl (long double __x) __attribute__ ((__nothrow__ )); extern long double __significandl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double copysignl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double nanl (const char *__tagb) __attribute__ ((__nothrow__ )); extern long double __nanl (const char *__tagb) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int isnanl (long double __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double j0l (long double) __attribute__ ((__nothrow__ )); extern long double __j0l (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double j1l (long double) __attribute__ ((__nothrow__ )); extern long double __j1l (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double jnl (int, long double) __attribute__ ((__nothrow__ )); extern long double __jnl (int, long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double y0l (long double) __attribute__ ((__nothrow__ )); extern long double __y0l (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double y1l (long double) __attribute__ ((__nothrow__ )); extern long double __y1l (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double ynl (int, long double) __attribute__ ((__nothrow__ )); extern long double __ynl (int, long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double erfl (long double) __attribute__ ((__nothrow__ )); extern long double __erfl (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
 extern long double erfcl (long double) __attribute__ ((__nothrow__ )); extern long double __erfcl (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double lgammal (long double) __attribute__ ((__nothrow__ )); extern long double __lgammal (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double tgammal (long double) __attribute__ ((__nothrow__ )); extern long double __tgammal (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double gammal (long double) __attribute__ ((__nothrow__ )); extern long double __gammal (long double) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double lgammal_r (long double, int *__signgamp) __attribute__ ((__nothrow__ )); extern long double __lgammal_r (long double, int *__signgamp) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double rintl (long double __x) __attribute__ ((__nothrow__ )); extern long double __rintl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double nextafterl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __nextafterl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double nexttowardl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __nexttowardl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double remainderl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __remainderl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double scalbnl (long double __x, int __n) __attribute__ ((__nothrow__ )); extern long double __scalbnl (long double __x, int __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern int ilogbl (long double __x) __attribute__ ((__nothrow__ )); extern int __ilogbl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double scalblnl (long double __x, long int __n) __attribute__ ((__nothrow__ )); extern long double __scalblnl (long double __x, long int __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double nearbyintl (long double __x) __attribute__ ((__nothrow__ )); extern long double __nearbyintl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double roundl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double truncl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double remquol (long double __x, long double __y, int *__quo) __attribute__ ((__nothrow__ )); extern long double __remquol (long double __x, long double __y, int *__quo) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long int lrintl (long double __x) __attribute__ ((__nothrow__ )); extern long int __lrintl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
__extension__ extern long long int llrintl (long double __x) __attribute__ ((__nothrow__ )); extern long long int __llrintl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long int lroundl (long double __x) __attribute__ ((__nothrow__ )); extern long int __lroundl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
__extension__ extern long long int llroundl (long double __x) __attribute__ ((__nothrow__ )); extern long long int __llroundl (long double __x) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double fdiml (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __fdiml (long double __x, long double __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double fmaxl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double fminl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double fmal (long double __x, long double __y, long double __z) __attribute__ ((__nothrow__ )); extern long double __fmal (long double __x, long double __y, long double __z) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls.h]
extern long double scalbl (long double __x, long double __n) __attribute__ ((__nothrow__ )); extern long double __scalbl (long double __x, long double __n) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __fpclassifyf128 (_Float128 __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __signbitf128 (_Float128 __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __isinff128 (_Float128 __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __finitef128 (_Float128 __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __isnanf128 (_Float128 __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __iseqsigf128 (_Float128 __x, _Float128 __y) __attribute__ ((__nothrow__ ));
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/mathcalls-helper-functions.h]
extern int __issignalingf128 (_Float128 __value) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
// [SYSTEM: /usr/include/math.h]
extern int signgam;
// [SYSTEM: /usr/include/math.h]
enum {
    FP_NAN = 0, FP_INFINITE = 1, FP_ZERO = 2, FP_SUBNORMAL = 3, FP_NORMAL = 4 };
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-uintn.h]
typedef __uint8_t uint8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-uintn.h]
typedef __uint16_t uint16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-uintn.h]
typedef __uint32_t uint32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-uintn.h]
typedef __uint64_t uint64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-least.h]
typedef __int_least8_t int_least8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-least.h]
typedef __int_least16_t int_least16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-least.h]
typedef __int_least32_t int_least32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-least.h]
typedef __int_least64_t int_least64_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-least.h]
typedef __uint_least8_t uint_least8_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-least.h]
typedef __uint_least16_t uint_least16_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-least.h]
typedef __uint_least32_t uint_least32_t;
// [SYSTEM: /usr/include/x86_64-linux-gnu/bits/stdint-least.h]
typedef __uint_least64_t uint_least64_t;
// [SYSTEM: /usr/include/stdint.h]
typedef signed char int_fast8_t;
// [SYSTEM: /usr/include/stdint.h]
typedef long int int_fast16_t;
// [SYSTEM: /usr/include/stdint.h]
typedef long int int_fast32_t;
// [SYSTEM: /usr/include/stdint.h]
typedef long int int_fast64_t;
// [SYSTEM: /usr/include/stdint.h]
typedef unsigned char uint_fast8_t;
// [SYSTEM: /usr/include/stdint.h]
typedef unsigned long int uint_fast16_t;
// [SYSTEM: /usr/include/stdint.h]
typedef unsigned long int uint_fast32_t;
// [SYSTEM: /usr/include/stdint.h]
typedef unsigned long int uint_fast64_t;
// [SYSTEM: /usr/include/stdint.h]
typedef long int intptr_t;
// [SYSTEM: /usr/include/stdint.h]
typedef unsigned long int uintptr_t;
// [SYSTEM: /usr/include/stdint.h]
typedef __intmax_t intmax_t;
// [SYSTEM: /usr/include/stdint.h]
typedef __uintmax_t uintmax_t;
// [LOCAL: ./lil.h]
typedef int64_t lilint_t;
// [LOCAL: ./lil.h]
typedef struct _lil_value_t* lil_value_t;
// [LOCAL: ./lil.h]
typedef struct _lil_func_t* lil_func_t;
// [LOCAL: ./lil.h]
typedef struct _lil_var_t* lil_var_t;
// [LOCAL: ./lil.h]
typedef struct _lil_env_t* lil_env_t;
// [LOCAL: ./lil.h]
typedef struct _lil_list_t* lil_list_t;
// [LOCAL: ./lil.h]
typedef struct _lil_t* lil_t;
// [LOCAL: ./lil.h]
typedef lil_value_t (*lil_func_proc_t)(lil_t lil, size_t argc, lil_value_t* argv);
// [LOCAL: ./lil.h]
typedef void (*lil_exit_callback_proc_t)(lil_t lil, lil_value_t arg);
// [LOCAL: ./lil.h]
typedef void (*lil_write_callback_proc_t)(lil_t lil, const char* msg);
// [LOCAL: ./lil.h]
typedef char* (*lil_read_callback_proc_t)(lil_t lil, const char* name);
// [LOCAL: ./lil.h]
typedef char* (*lil_source_callback_proc_t)(lil_t lil, const char* name);
// [LOCAL: ./lil.h]
typedef void (*lil_store_callback_proc_t)(lil_t lil, const char* name, const char* data);
// [LOCAL: ./lil.h]
typedef void (*lil_error_callback_proc_t)(lil_t lil, size_t pos, const char* msg);
// [LOCAL: ./lil.h]
typedef int (*lil_setvar_callback_proc_t)(lil_t lil, const char* name, lil_value_t* value);
// [LOCAL: ./lil.h]
typedef int (*lil_getvar_callback_proc_t)(lil_t lil, const char* name, lil_value_t* value);
// [LOCAL: ./lil.h]
typedef void (*lil_callback_proc_t)(void);
// [LOCAL: ./lil.h]
       lil_t lil_new(void);
// [LOCAL: ./lil.h]
       void lil_free(lil_t lil);
// [LOCAL: ./lil.h]
       int lil_register(lil_t lil, const char* name, lil_func_proc_t proc);
// [LOCAL: ./lil.h]
       lil_value_t lil_parse(lil_t lil, const char* code, size_t codelen, int funclevel);
// [LOCAL: ./lil.h]
       lil_value_t lil_parse_value(lil_t lil, lil_value_t val, int funclevel);
// [LOCAL: ./lil.h]
       void lil_callback(lil_t lil, int cb, lil_callback_proc_t proc);
// [LOCAL: ./lil.h]
       void lil_set_error(lil_t lil, const char* msg);
// [LOCAL: ./lil.h]
       void lil_set_error_at(lil_t lil, size_t pos, const char* msg);
// [LOCAL: ./lil.h]
       int lil_error(lil_t lil, const char** msg, size_t* pos);
// [LOCAL: ./lil.h]
       const char* lil_to_string(lil_value_t val);
// [LOCAL: ./lil.h]
       double lil_to_double(lil_value_t val);
// [LOCAL: ./lil.h]
       lilint_t lil_to_integer(lil_value_t val);
// [LOCAL: ./lil.h]
       int lil_to_boolean(lil_value_t val);
// [LOCAL: ./lil.h]
       lil_value_t lil_alloc_string(const char* str);
// [LOCAL: ./lil.h]
       lil_value_t lil_alloc_double(double num);
// [LOCAL: ./lil.h]
       lil_value_t lil_alloc_integer(lilint_t num);
// [LOCAL: ./lil.h]
       void lil_free_value(lil_value_t val);
// [LOCAL: ./lil.h]
       lil_value_t lil_clone_value(lil_value_t src);
// [LOCAL: ./lil.h]
       int lil_append_char(lil_value_t val, char ch);
// [LOCAL: ./lil.h]
       int lil_append_string(lil_value_t val, const char* s);
// [LOCAL: ./lil.h]
       int lil_append_val(lil_value_t val, lil_value_t v);
// [LOCAL: ./lil.h]
       lil_list_t lil_alloc_list(void);
// [LOCAL: ./lil.h]
       void lil_free_list(lil_list_t list);
// [LOCAL: ./lil.h]
       void lil_list_append(lil_list_t list, lil_value_t val);
// [LOCAL: ./lil.h]
       size_t lil_list_size(lil_list_t list);
// [LOCAL: ./lil.h]
       lil_value_t lil_list_get(lil_list_t list, size_t index);
// [LOCAL: ./lil.h]
       lil_value_t lil_list_to_value(lil_list_t list, int do_escape);
// [LOCAL: ./lil.h]
       lil_list_t lil_subst_to_list(lil_t lil, lil_value_t code);
// [LOCAL: ./lil.h]
       lil_value_t lil_subst_to_value(lil_t lil, lil_value_t code);
// [LOCAL: ./lil.h]
       lil_env_t lil_alloc_env(lil_env_t parent);
// [LOCAL: ./lil.h]
       void lil_free_env(lil_env_t env);
// [LOCAL: ./lil.h]
       lil_env_t lil_push_env(lil_t lil);
// [LOCAL: ./lil.h]
       void lil_pop_env(lil_t lil);
// [LOCAL: ./lil.h]
       lil_var_t lil_set_var(lil_t lil, const char* name, lil_value_t val, int local);
// [LOCAL: ./lil.h]
       lil_value_t lil_get_var(lil_t lil, const char* name);
// [LOCAL: ./lil.h]
       lil_value_t lil_get_var_or(lil_t lil, const char* name, lil_value_t defvalue);
// [LOCAL: ./lil.h]
       lil_value_t lil_eval_expr(lil_t lil, lil_value_t code);
// [LOCAL: ./lil.h]
       lil_value_t lil_unused_name(lil_t lil, const char* part);
// [LOCAL: ./lil.h]
       lil_value_t lil_arg(lil_value_t* argv, size_t index);
// [LOCAL: ./lil.h]
       void lil_set_data(lil_t lil, void* data);
// [LOCAL: ./lil.h]
       void* lil_get_data(lil_t lil);
// [LOCAL: lil.c]
struct _lil_value_t {
    size_t l;
    char* d;
};
// [LOCAL: lil.c]
struct _lil_var_t {
    char* n;
    struct _lil_env_t* env;
    lil_value_t v;
};
// [LOCAL: lil.c]
struct _lil_env_t {
    struct _lil_env_t* parent;
    lil_func_t func;
    lil_value_t catcher_for;
    lil_var_t* var;
    size_t vars;
    lil_value_t retval;
    int retval_set;
    int breakrun;
};
// [LOCAL: lil.c]
struct _lil_list_t {
    lil_value_t* v;
    size_t c;
};
// [LOCAL: lil.c]
struct _lil_func_t {
    char* name;
    lil_value_t code;
    lil_list_t argnames;
    lil_func_proc_t proc;
};
// [LOCAL: lil.c]
struct _lil_t {
    const char* code;
    const char* rootcode;
    size_t clen;
    size_t head;
    int ignoreeol;
    lil_func_t* cmd;
    size_t cmds;
    size_t syscmds;
    char* catcher;
    int in_catcher;
    char* dollarprefix;
    lil_env_t env;
    lil_env_t rootenv;
    lil_env_t downenv;
    lil_value_t empty;
    int error;
    size_t err_head;
    char* err_msg;
    lil_callback_proc_t callback[8];
    size_t parse_depth;
    void* data;
};
// [LOCAL: lil.c]
typedef struct _expreval_t {
    const char* code;
    size_t len, head;
    lilint_t ival;
    double dval;
    int type;
    int error;
} expreval_t;
// [LOCAL: lil.c]
static lil_value_t next_word(lil_t lil);
// [LOCAL: lil.c]
static void register_stdcmds(lil_t lil);
// [LOCAL: lil.c]
static char* strclone(const char* s) {
    size_t len = strlen(s) + 1;
    char* ns = malloc(len);
    if (!ns) return ((void*)0);
    memcpy(ns, s, len);
    return ns;
}
// [LOCAL: lil.c]
static lil_value_t alloc_value(const char* str) {
    lil_value_t val = calloc(1, sizeof(struct _lil_value_t));
    if (!val) return ((void*)0);
    if (str) {
        val->l = strlen(str);
        val->d = malloc(val->l + 1);
        if (!val->d) {
            free(val);
            return ((void*)0);
        }
        memcpy(val->d, str, val->l + 1);
    } else {
        val->l = 0;
        val->d = ((void*)0);
    }
    return val;
}
// [LOCAL: lil.c]
lil_value_t lil_clone_value(lil_value_t src) {
    lil_value_t val;
    if (!src) return ((void*)0);
    val = calloc(1, sizeof(struct _lil_value_t));
    if (!val) return ((void*)0);
    val->l = src->l;
    if (src->l) {
        val->d = malloc(val->l + 1);
        if (!val->d) {
            free(val);
            return ((void*)0);
        }
        memcpy(val->d, src->d, val->l + 1);
    } else val->d = ((void*)0);
    return val;
}
// [LOCAL: lil.c]
int lil_append_char(lil_value_t val, char ch) {
    char* new = realloc(val->d, val->l + 2);
    if (!new) return 0;
    new[val->l++] = ch;
    new[val->l] = 0;
    val->d = new;
    return 1;
}
// [LOCAL: lil.c]
int lil_append_string(lil_value_t val, const char* s) {
    char* new;
    size_t len;
    if (!s || !s[0]) return 1;
    len = strlen(s);
    new = realloc(val->d, val->l + len + 1);
    if (!new) return 0;
    memcpy(new + val->l, s, len + 1);
    val->l += len;
    val->d = new;
    return 1;
}
// [LOCAL: lil.c]
int lil_append_val(lil_value_t val, lil_value_t v) {
    char* new;
    if (!v || !v->l) return 1;
    new = realloc(val->d, val->l + v->l + 1);
    if (!new) return 0;
    memcpy(new + val->l, v->d, v->l + 1);
    val->l += v->l;
    val->d = new;
    return 1;
}
// [LOCAL: lil.c]
void lil_free_value(lil_value_t val) {
    if (!val) return;
    free(val->d);
    free(val);
}
// [LOCAL: lil.c]
lil_list_t lil_alloc_list(void) {
    lil_list_t list = calloc(1, sizeof(struct _lil_list_t));
    list->v = ((void*)0);
    list->c = 0;
    return list;
}
// [LOCAL: lil.c]
void lil_free_list(lil_list_t list) {
    size_t i;
    if (!list) return;
    for (i=0; i<list->c; i++) lil_free_value(list->v[i]);
    free(list->v);
    free(list);
}
// [LOCAL: lil.c]
void lil_list_append(lil_list_t list, lil_value_t val) {
    lil_value_t* nv = realloc(list->v, sizeof(lil_value_t)*(list->c + 1));
    if (!nv) return;
    list->v = nv;
    nv[list->c++] = val;
}
// [LOCAL: lil.c]
size_t lil_list_size(lil_list_t list) {
    return list->c;
}
// [LOCAL: lil.c]
lil_value_t lil_list_get(lil_list_t list, size_t index) {
    return index >= list->c ? ((void*)0) : list->v[index];
}
// [LOCAL: lil.c]
static int needs_escape(const char* str) {
    size_t i;
    if (!str || !str[0]) return 1;
    for (i=0; str[i]; i++) if (((*__ctype_b_loc ())[(int) ((str[i]))] & (unsigned short int) _ISpunct) || ((*__ctype_b_loc ())[(int) ((str[i]))] & (unsigned short int) _ISspace)) return 1;
    return 0;
}
// [LOCAL: lil.c]
lil_value_t lil_list_to_value(lil_list_t list, int do_escape) {
    lil_value_t val = alloc_value(((void*)0));
    size_t i;
    for (i=0; i<list->c; i++) {
        int escape = do_escape ? needs_escape(lil_to_string(list->v[i])) : 0;
        if (i) lil_append_char(val, ' ');
        if (escape) lil_append_char(val, '{');
        lil_append_val(val, list->v[i]);
        if (escape) lil_append_char(val, '}');
    }
    return val;
}
// [LOCAL: lil.c]
lil_env_t lil_alloc_env(lil_env_t parent) {
    lil_env_t env = calloc(1, sizeof(struct _lil_env_t));
    env->parent = parent;
    return env;
}
// [LOCAL: lil.c]
void lil_free_env(lil_env_t env) {
    size_t i;
    if (!env) return;
    lil_free_value(env->retval);
    for (i=0; i<env->vars; i++) {
        free(env->var[i]->n);
        lil_free_value(env->var[i]->v);
        free(env->var[i]);
    }
    free(env->var);
    free(env);
}
// [LOCAL: lil.c]
static lil_var_t lil_find_local_var(lil_t lil, lil_env_t env, const char* name) {
    if (env->vars > 0) {
        size_t i = env->vars - 1;
        while (1) {
            if (!strcmp(env->var[i]->n, name)) return env->var[i];
            if (!i) break;
            i--;
        }
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_var_t lil_find_var(lil_t lil, lil_env_t env, const char* name) {
    lil_var_t r = lil_find_local_var(lil, env, name);
    return r ? r : (env == lil->rootenv ? ((void*)0) : lil_find_var(lil, lil->rootenv, name));
}
// [LOCAL: lil.c]
static lil_func_t find_cmd(lil_t lil, const char* name) {
    if (lil->cmds > 0) {
        size_t i = lil->cmds - 1;
        while (1) {
            if (!strcmp(lil->cmd[i]->name, name)) return lil->cmd[i];
            if (!i) break;
            i--;
        }
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_func_t add_func(lil_t lil, const char* name) {
    lil_func_t cmd;
    lil_func_t* ncmd;
    cmd = find_cmd(lil, name);
    if (cmd) return cmd;
    cmd = calloc(1, sizeof(struct _lil_func_t));
    cmd->name = strclone(name);
    ncmd = realloc(lil->cmd, sizeof(lil_func_t)*(lil->cmds + 1));
    if (!ncmd) {
        free(cmd);
        return ((void*)0);
    }
    lil->cmd = ncmd;
    ncmd[lil->cmds++] = cmd;
    return cmd;
}
// [LOCAL: lil.c]
int lil_register(lil_t lil, const char* name, lil_func_proc_t proc) {
    lil_func_t cmd = add_func(lil, name);
    if (!cmd) return 0;
    cmd->proc = proc;
    return 1;
}
// [LOCAL: lil.c]
lil_var_t lil_set_var(lil_t lil, const char* name, lil_value_t val, int local) {
    lil_var_t* nvar;
    lil_env_t env = local == 0 ? lil->rootenv : lil->env;
    int freeval = 0;
    if (!name[0]) return ((void*)0);
    if (local != 2) {
        lil_var_t var = lil_find_var(lil, env, name);
        if (local == 3 && var && var->env == lil->rootenv && var->env != env) var = ((void*)0);
        if (((!var && env == lil->rootenv) || (var && var->env == lil->rootenv)) && lil->callback[6]) {
            lil_setvar_callback_proc_t proc = (lil_setvar_callback_proc_t)lil->callback[6];
            lil_value_t newval = val;
            int r = proc(lil, name, &newval);
            if (r < 0) return ((void*)0);
            if (r) {
                val = newval;
                freeval = 1;
            }
        }
        if (var) {
            lil_free_value(var->v);
            var->v = freeval ? val : lil_clone_value(val);
            return var;
        }
    }
    nvar = realloc(env->var, sizeof(lil_var_t)*(env->vars + 1));
    if (!nvar) {
        return ((void*)0);
    }
    env->var = nvar;
    nvar[env->vars] = calloc(1, sizeof(struct _lil_var_t));
    nvar[env->vars]->n = strclone(name);
    nvar[env->vars]->env = env;
    nvar[env->vars]->v = freeval ? val : lil_clone_value(val);
    return nvar[env->vars++];
}
// [LOCAL: lil.c]
lil_value_t lil_get_var(lil_t lil, const char* name) {
    return lil_get_var_or(lil, name, lil->empty);
}
// [LOCAL: lil.c]
lil_value_t lil_get_var_or(lil_t lil, const char* name, lil_value_t defvalue) {
    lil_var_t var = lil_find_var(lil, lil->env, name);
    lil_value_t retval = var ? var->v : defvalue;
    if (lil->callback[7] && (!var || var->env == lil->rootenv)) {
        lil_getvar_callback_proc_t proc = (lil_getvar_callback_proc_t)lil->callback[7];
        lil_value_t newretval = retval;
        if (proc(lil, name, &newretval)) retval = newretval;
    }
    return retval;
}
// [LOCAL: lil.c]
lil_env_t lil_push_env(lil_t lil) {
    lil_env_t env = lil_alloc_env(lil->env);
    lil->env = env;
    return env;
}
// [LOCAL: lil.c]
void lil_pop_env(lil_t lil) {
    if (lil->env->parent) {
        lil_env_t next = lil->env->parent;
        lil_free_env(lil->env);
        lil->env = next;
    }
}
// [LOCAL: lil.c]
lil_t lil_new(void) {
    lil_t lil = calloc(1, sizeof(struct _lil_t));
    lil->rootenv = lil->env = lil_alloc_env(((void*)0));
    lil->empty = alloc_value(((void*)0));
    lil->dollarprefix = strclone("set ");
    register_stdcmds(lil);
    return lil;
}
// [LOCAL: lil.c]
static int islilspecial(char ch) {
    return ch == ';' || ch == '$' || ch == '[' || ch == ']' || ch == '{' || ch == '}' || ch == '"' || ch == '\'';
}
// [LOCAL: lil.c]
static int ateol(lil_t lil) {
    return !(lil->ignoreeol) && (lil->code[lil->head] == '\n' || lil->code[lil->head] == '\r' || lil->code[lil->head] == ';');
}
// [LOCAL: lil.c]
static void skip_spaces(lil_t lil) {
    while (lil->head < lil->clen && (lil->code[lil->head] == '\\' || lil->code[lil->head] == '#' || (((*__ctype_b_loc ())[(int) ((lil->code[lil->head]))] & (unsigned short int) _ISspace) && (lil->ignoreeol || !(lil->code[lil->head] == '\r' || lil->code[lil->head] == '\n'))))) {
        if (lil->code[lil->head] == '#') {
            while (lil->head < lil->clen && !ateol(lil)) lil->head++;
        } else if (lil->code[lil->head] == '\\' && (lil->code[lil->head + 1] == '\r' || lil->code[lil->head + 1] == '\n')) {
            lil->head++;
            while (lil->head < lil->clen && ateol(lil)) lil->head++;
        } else lil->head++;
    }
}
// [LOCAL: lil.c]
static lil_value_t get_bracketpart(lil_t lil) {
    size_t cnt = 1;
    lil_value_t val, cmd = alloc_value(((void*)0));
    lil->head++;
    while (lil->head < lil->clen) {
        if (lil->code[lil->head] == '[') {
            lil->head++;
            cnt++;
            lil_append_char(cmd, '[');
        } else if (lil->code[lil->head] == ']') {
            lil->head++;
            if (--cnt == 0) break;
            else lil_append_char(cmd, ']');
        } else {
            lil_append_char(cmd, lil->code[lil->head++]);
        }
    }
    val = lil_parse_value(lil, cmd, 0);
    lil_free_value(cmd);
    return val;
}
// [LOCAL: lil.c]
static lil_value_t get_dollarpart(lil_t lil) {
    lil_value_t val, name, tmp;
    lil->head++;
    name = next_word(lil);
    tmp = alloc_value(lil->dollarprefix);
    lil_append_val(tmp, name);
    lil_free_value(name);
    val = lil_parse_value(lil, tmp, 0);
    lil_free_value(tmp);
    return val;
}
// [LOCAL: lil.c]
static lil_value_t next_word(lil_t lil) {
    lil_value_t val;
    skip_spaces(lil);
    if (lil->code[lil->head] == '$') {
        val = get_dollarpart(lil);
    } else if (lil->code[lil->head] == '{') {
        size_t cnt = 1;
        lil->head++;
        val = alloc_value(((void*)0));
        while (lil->head < lil->clen) {
            if (lil->code[lil->head] == '{') {
                lil->head++;
                cnt++;
                lil_append_char(val, '{');
            } else if (lil->code[lil->head] == '}') {
                lil->head++;
                if (--cnt == 0) break;
                else lil_append_char(val, '}');
            } else {
                lil_append_char(val, lil->code[lil->head++]);
            }
        }
    } else if (lil->code[lil->head] == '[') {
        val = get_bracketpart(lil);
    } else if (lil->code[lil->head] == '"' || lil->code[lil->head] == '\'') {
        char sc = lil->code[lil->head++];
        val = alloc_value(((void*)0));
        while (lil->head < lil->clen) {
            if (lil->code[lil->head] == '[' || lil->code[lil->head] == '$') {
                lil_value_t tmp = lil->code[lil->head] == '$' ? get_dollarpart(lil) : get_bracketpart(lil);
                lil_append_val(val, tmp);
                lil_free_value(tmp);
                lil->head--;
            } else if (lil->code[lil->head] == '\\') {
                lil->head++;
                switch (lil->code[lil->head]) {
                    case 'b': lil_append_char(val, '\b'); break;
                    case 't': lil_append_char(val, '\t'); break;
                    case 'n': lil_append_char(val, '\n'); break;
                    case 'v': lil_append_char(val, '\v'); break;
                    case 'f': lil_append_char(val, '\f'); break;
                    case 'r': lil_append_char(val, '\r'); break;
                    case '0': lil_append_char(val, 0); break;
                    case 'a': lil_append_char(val, '\a'); break;
                    case 'c': lil_append_char(val, '}'); break;
                    case 'o': lil_append_char(val, '{'); break;
                    default: lil_append_char(val, lil->code[lil->head]);
                }
            } else if (lil->code[lil->head] == sc) {
                lil->head++;
                break;
            } else {
                lil_append_char(val, lil->code[lil->head]);
            }
            lil->head++;
        }
    } else {
        val = alloc_value(((void*)0));
        while (lil->head < lil->clen && !((*__ctype_b_loc ())[(int) ((lil->code[lil->head]))] & (unsigned short int) _ISspace) && !islilspecial(lil->code[lil->head])) {
            lil_append_char(val, lil->code[lil->head++]);
        }
    }
    return val ? val : alloc_value(((void*)0));
}
// [LOCAL: lil.c]
static lil_list_t substitute(lil_t lil) {
    lil_list_t words = lil_alloc_list();
    skip_spaces(lil);
    while (lil->head < lil->clen && !ateol(lil) && !lil->error) {
        lil_value_t w = alloc_value(((void*)0));
        do {
            size_t head = lil->head;
            lil_value_t wp = next_word(lil);
            if (head == lil->head) {
                lil_free_value(w);
                lil_free_value(wp);
                lil_free_list(words);
                return ((void*)0);
            }
            lil_append_val(w, wp);
            lil_free_value(wp);
        } while (lil->head < lil->clen && !ateol(lil) && !((*__ctype_b_loc ())[(int) ((lil->code[lil->head]))] & (unsigned short int) _ISspace) && !lil->error);
        skip_spaces(lil);
        lil_list_append(words, w);
    }
    return words;
}
// [LOCAL: lil.c]
lil_list_t lil_subst_to_list(lil_t lil, lil_value_t code) {
    const char* save_code = lil->code;
    size_t save_clen = lil->clen;
    size_t save_head = lil->head;
    int save_igeol = lil->ignoreeol;
    lil_list_t words;
    lil->code = lil_to_string(code);
    lil->clen = code->l;
    lil->head = 0;
    lil->ignoreeol = 1;
    words = substitute(lil);
    lil->code = save_code;
    lil->clen = save_clen;
    lil->head = save_head;
    lil->ignoreeol = save_igeol;
    return words;
}
// [LOCAL: lil.c]
lil_value_t lil_subst_to_value(lil_t lil, lil_value_t code) {
    lil_list_t words = lil_subst_to_list(lil, code);
    lil_value_t val;
    if (!words) return lil_clone_value(code);
    val = lil_list_to_value(words, 0);
    lil_free_list(words);
    return val;
}
// [LOCAL: lil.c]
lil_value_t lil_parse(lil_t lil, const char* code, size_t codelen, int funclevel) {
    const char* save_code = lil->code;
    size_t save_clen = lil->clen;
    size_t save_head = lil->head;
    lil_value_t val = ((void*)0);
    lil_list_t words = ((void*)0);
    if (!save_code) lil->rootcode = code;
    lil->code = code;
    lil->clen = codelen ? codelen : strlen(code);
    lil->head = 0;
    skip_spaces(lil);
    lil->parse_depth++;
    if (lil->parse_depth == 1) lil->error = 0;
    if (funclevel) lil->env->breakrun = 0;
    while (lil->head < lil->clen && !lil->error) {
        if (words) lil_free_list(words);
        if (val) lil_free_value(val);
        val = ((void*)0);
        words = substitute(lil);
        if (!words || lil->error) goto cleanup;
        if (words->c) {
            lil_func_t cmd = find_cmd(lil, lil_to_string(words->v[0]));
            if (!cmd) {
                if (words->v[0]->l) {
                    if (lil->catcher) {
                        if (lil->in_catcher < 16384) {
                            lil_value_t args;
                            lil->in_catcher++;
                            lil_push_env(lil);
                            lil->env->catcher_for = words->v[0];
                            args = lil_list_to_value(words, 1);
                            lil_set_var(lil, "args", args, 2);
                            lil_free_value(args);
                            val = lil_parse(lil, lil->catcher, 0, 1);
                            lil_pop_env(lil);
                            lil->in_catcher--;
                        } else {
                            char* msg = malloc(words->v[0]->l + 64);
                            sprintf(msg, "catcher limit reached while trying to call unknown function %s", words->v[0]->d);
                            lil_set_error_at(lil, lil->head, msg);
                            free(msg);
                            goto cleanup;
                        }
                    } else {
                        char* msg = malloc(words->v[0]->l + 32);
                        sprintf(msg, "unknown function %s", words->v[0]->d);
                        lil_set_error_at(lil, lil->head, msg);
                        free(msg);
                        goto cleanup;
                    }
                }
            }
            if (cmd) {
                if (cmd->proc) {
                    size_t shead = lil->head;
                    val = cmd->proc(lil, words->c - 1, words->v + 1);
                    if (lil->error == 2) {
                        lil->error = 1;
                        lil->err_head = shead;
                    }
                } else {
                    lil_push_env(lil);
                    lil->env->func = cmd;
                    if (cmd->argnames->c == 1 && !strcmp(lil_to_string(cmd->argnames->v[0]), "args")) {
                        lil_value_t args = lil_list_to_value(words, 1);
                        lil_set_var(lil, "args", args, 2);
                        lil_free_value(args);
                    } else {
                        size_t i;
                        for (i=0; i<cmd->argnames->c; i++) {
                            lil_set_var(lil, lil_to_string(cmd->argnames->v[i]), i < words->c - 1 ? words->v[i + 1] : lil->empty, 2);
                        }
                    }
                    val = lil_parse_value(lil, cmd->code, 1);
                    lil_pop_env(lil);
                }
            }
        }
        if (lil->env->breakrun) goto cleanup;
        skip_spaces(lil);
        while (ateol(lil)) lil->head++;
        skip_spaces(lil);
    }
cleanup: if (lil->error && lil->callback[5] && lil->parse_depth == 1) {
        lil_error_callback_proc_t proc = (lil_error_callback_proc_t)lil->callback[5];
        proc(lil, lil->err_head, lil->err_msg);
    }
    if (words) lil_free_list(words);
    lil->code = save_code;
    lil->clen = save_clen;
    lil->head = save_head;
    if (funclevel && lil->env->retval_set) {
        if (val) lil_free_value(val);
        val = lil->env->retval;
        lil->env->retval = ((void*)0);
        lil->env->retval_set = 0;
        lil->env->breakrun = 0;
    }
    lil->parse_depth--;
    return val ? val : alloc_value(((void*)0));
}
// [LOCAL: lil.c]
lil_value_t lil_parse_value(lil_t lil, lil_value_t val, int funclevel) {
    if (!val || !val->d || !val->l) return alloc_value(((void*)0));
    return lil_parse(lil, val->d, val->l, funclevel);
}
// [LOCAL: lil.c]
void lil_callback(lil_t lil, int cb, lil_callback_proc_t proc) {
    if (cb < 0 || cb > 8) return;
    lil->callback[cb] = proc;
}
// [LOCAL: lil.c]
void lil_set_error(lil_t lil, const char* msg) {
    if (lil->error) return;
    free(lil->err_msg);
    lil->error = 2;
    lil->err_head = 0;
    lil->err_msg = strclone(msg ? msg : "");
}
// [LOCAL: lil.c]
void lil_set_error_at(lil_t lil, size_t pos, const char* msg) {
    if (lil->error) return;
    free(lil->err_msg);
    lil->error = 1;
    lil->err_head = pos;
    lil->err_msg = strclone(msg ? msg : "");
}
// [LOCAL: lil.c]
int lil_error(lil_t lil, const char** msg, size_t* pos) {
    if (!lil->error) return 0;
    *msg = lil->err_msg;
    *pos = lil->err_head;
    lil->error = 0;
    return 1;
}
// [LOCAL: lil.c]
static void ee_expr(expreval_t* ee);
// [LOCAL: lil.c]
static void ee_skip_spaces(expreval_t* ee) {
    while (ee->head < ee->len && ((*__ctype_b_loc ())[(int) ((ee->code[ee->head]))] & (unsigned short int) _ISspace)) ee->head++;
}
// [LOCAL: lil.c]
static void ee_numeric_element(expreval_t* ee) {
    lilint_t fpart = 0, fpartlen = 1;
    ee->type = 0;
    ee_skip_spaces(ee);
    ee->ival = 0;
    ee->dval = 0;
    while (ee->head < ee->len) {
        if (ee->code[ee->head] == '.') {
            if (ee->type == 1) break;
            ee->type = 1;
            ee->head++;
        } else if (!((*__ctype_b_loc ())[(int) ((ee->code[ee->head]))] & (unsigned short int) _ISdigit)) break;
        if (ee->type == 0) ee->ival = ee->ival*10 + (ee->code[ee->head] - '0');
        else {
            fpart = fpart*10 + (ee->code[ee->head] - '0');
            fpartlen *= 10;
        }
        ee->head++;
    }
    if (ee->type == 1) ee->dval = ee->ival + (double)fpart/(double)fpartlen;
}
// [LOCAL: lil.c]
static void ee_element(expreval_t* ee) {
    if (((*__ctype_b_loc ())[(int) ((ee->code[ee->head]))] & (unsigned short int) _ISdigit)) {
        ee_numeric_element(ee);
        return;
    }
    ee->type = 0;
    ee->ival = 1;
    ee->error = 4;
}
// [LOCAL: lil.c]
static void ee_paren(expreval_t* ee) {
    ee_skip_spaces(ee);
    if (ee->code[ee->head] == '(') {
        ee->head++;
        ee_expr(ee);
        ee_skip_spaces(ee);
        if (ee->code[ee->head] == ')') ee->head++;
        else ee->error = 1;
    } else ee_element(ee);
}
// [LOCAL: lil.c]
static void ee_unary(expreval_t* ee) {
    ee_skip_spaces(ee);
    if (ee->head < ee->len && !ee->error && (ee->code[ee->head] == '-' || ee->code[ee->head] == '+' || ee->code[ee->head] == '~' || ee->code[ee->head] == '!')) {
        char op = ee->code[ee->head++];
        ee_unary(ee);
        if (ee->error) return;
        switch (op) {
        case '-': switch (ee->type) {
            case 1: ee->dval = -ee->dval;
                break;
            case 0: ee->ival = -ee->ival;
                break;
            default: ee->error = 2;
            }
            break;
        case '+': break;
        case '~': switch (ee->type) {
            case 1: ee->ival = ~((lilint_t)ee->dval);
                ee->type = 0;
                break;
            case 0: ee->ival = ~ee->ival;
                break;
            default: ee->error = 2;
            }
            break;
        case '!': switch (ee->type) {
            case 1: ee->dval = !ee->dval;
                break;
            case 0: ee->ival = !ee->ival;
                break;
            default: ee->error = 2;
            }
            break;
        }
    } else {
        ee_paren(ee);
    }
}
// [LOCAL: lil.c]
static void ee_muldiv(expreval_t* ee) {
    ee_unary(ee);
    if (ee->error) return;
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && !((*__ctype_b_loc ())[(int) ((ee->code[ee->head + 1]))] & (unsigned short int) _ISpunct) && (ee->code[ee->head] == '*' || ee->code[ee->head] == '/' || ee->code[ee->head] == '\\' || ee->code[ee->head] == '%')) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        switch (ee->code[ee->head]) {
        case '*': switch (ee->type) {
            case 1: ee->head++;
                ee_unary(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->dval = ee->dval*odval;
                    break;
                case 0: ee->dval = ee->ival*odval;
                    ee->type = 1;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee->head++;
                ee_unary(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->dval = ee->dval*oival;
                    ee->type = 1;
                    break;
                case 0: ee->ival = ee->ival*oival;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        case '%': switch (ee->type) {
            case 1: ee->head++;
                ee_unary(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: if (ee->dval == 0.0) {
                        ee->error = 3;
                    } else {
                        ee->dval = fmod(odval, ee->dval);
                    }
                    break;
                case 0: if (ee->ival == 0) {
                        ee->error = 3;
                    } else {
                        ee->dval = fmod(odval, ee->ival);
                    }
                    ee->type = 1;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee->head++;
                ee_unary(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: if (ee->dval == 0.0) {
                        ee->error = 3;
                    } else {
                        ee->dval = fmod(oival, ee->dval);
                    }
                    break;
                case 0: if (ee->ival == 0) {
                        ee->error = 3;
                    } else {
                        ee->ival = oival%ee->ival;
                    }
                    break;
                default: ee->error = 2;
                }
                break;
            }
            break;
        case '/': switch (ee->type) {
            case 1: ee->head++;
                ee_unary(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: if (ee->dval == 0.0) {
                        ee->error = 3;
                    } else {
                        ee->dval = odval/ee->dval;
                    }
                    break;
                case 0: if (ee->ival == 0) {
                        ee->error = 3;
                    } else {
                        ee->dval = odval/(double)ee->ival;
                    }
                    ee->type = 1;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee->head++;
                ee_unary(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: if (ee->dval == 0.0) {
                        ee->error = 3;
                    } else {
                        ee->dval = (double)oival/ee->dval;
                    }
                    break;
                case 0: if (ee->ival == 0) {
                        ee->error = 3;
                    } else {
                        ee->dval = (double)oival/(double)ee->ival;
                    }
                    ee->type = 1;
                    break;
                default: ee->error = 2;
                }
                break;
            }
            break;
        case '\\': switch (ee->type) {
            case 1: ee->head++;
                ee_unary(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: if (ee->dval == 0.0) {
                        ee->error = 3;
                    } else {
                        ee->ival = (lilint_t)(odval/ee->dval);
                    }
                    ee->type = 0;
                    break;
                case 0: if (ee->ival == 0) {
                        ee->error = 3;
                    } else {
                        ee->ival = (lilint_t)(odval/(double)ee->ival);
                    }
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee->head++;
                ee_unary(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: if (ee->dval == 0.0) {
                        ee->error = 3;
                    } else {
                        ee->ival = (lilint_t)((double)oival/ee->dval);
                    }
                    ee->type = 0;
                    break;
                case 0: if (ee->ival == 0) {
                        ee->error = 3;
                    } else {
                        ee->ival = oival/ee->ival;
                    }
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_addsub(expreval_t* ee) {
    ee_muldiv(ee);
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && !((*__ctype_b_loc ())[(int) ((ee->code[ee->head + 1]))] & (unsigned short int) _ISpunct) && (ee->code[ee->head] == '+' || ee->code[ee->head] == '-')) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        switch (ee->code[ee->head]) {
        case '+': switch (ee->type) {
            case 1: ee->head++;
                ee_muldiv(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->dval = ee->dval+odval;
                    break;
                case 0: ee->dval = ee->ival+odval;
                    ee->type = 1;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee->head++;
                ee_muldiv(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->dval = ee->dval+oival;
                    ee->type = 1;
                    break;
                case 0: ee->ival = ee->ival+oival;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        case '-': switch (ee->type) {
            case 1: ee->head++;
                ee_muldiv(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->dval = odval-ee->dval;
                    break;
                case 0: ee->dval = odval-ee->ival;
                    ee->type = 1;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee->head++;
                ee_muldiv(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->dval = (double)oival-ee->dval;
                    ee->type = 1;
                    break;
                case 0: ee->ival = oival-ee->ival;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_shift(expreval_t* ee) {
    ee_addsub(ee);
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && ((ee->code[ee->head] == '<' && ee->code[ee->head + 1] == '<') || (ee->code[ee->head] == '>' && ee->code[ee->head + 1] == '>'))) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        ee->head++;
        switch (ee->code[ee->head]) {
        case '<': switch (ee->type) {
            case 1: ee->head++;
                ee_addsub(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (lilint_t)odval << (lilint_t)ee->dval;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (lilint_t)odval << ee->ival;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee->head++;
                ee_addsub(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = oival << (lilint_t)ee->dval;
                    ee->type = 0;
                    break;
                case 0: ee->ival = oival << ee->ival;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        case '>': switch (ee->type) {
            case 1: ee->head++;
                ee_addsub(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (lilint_t)odval >> (lilint_t)ee->dval;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (lilint_t)odval >> ee->ival;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee->head++;
                ee_addsub(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = oival >> (lilint_t)ee->dval;
                    ee->type = 0;
                    break;
                case 0: ee->ival = oival >> ee->ival;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_compare(expreval_t* ee) {
    ee_shift(ee);
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && ((ee->code[ee->head] == '<' && !((*__ctype_b_loc ())[(int) ((ee->code[ee->head + 1]))] & (unsigned short int) _ISpunct)) || (ee->code[ee->head] == '>' && !((*__ctype_b_loc ())[(int) ((ee->code[ee->head + 1]))] & (unsigned short int) _ISpunct)) || (ee->code[ee->head] == '<' && ee->code[ee->head + 1] == '=') || (ee->code[ee->head] == '>' && ee->code[ee->head + 1] == '='))) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        int op = 4;
        if (ee->code[ee->head] == '<' && !((*__ctype_b_loc ())[(int) ((ee->code[ee->head + 1]))] & (unsigned short int) _ISpunct)) op = 1;
        else if (ee->code[ee->head] == '>' && !((*__ctype_b_loc ())[(int) ((ee->code[ee->head + 1]))] & (unsigned short int) _ISpunct)) op = 2;
        else if (ee->code[ee->head] == '<' && ee->code[ee->head + 1] == '=') op = 3;
        ee->head += op > 2 ? 2 : 1;
        switch (op) {
        case 1: switch (ee->type) {
            case 1: ee_shift(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (odval < ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (odval < ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee_shift(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (oival < ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (oival < ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        case 2: switch (ee->type) {
            case 1: ee_shift(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (odval > ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (odval > ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee_shift(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (oival > ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (oival > ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        case 3: switch (ee->type) {
            case 1: ee_shift(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (odval <= ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (odval <= ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee_shift(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (oival <= ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (oival <= ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        case 4: switch (ee->type) {
            case 1: ee_shift(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (odval >= ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (odval >= ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee_shift(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (oival >= ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (oival >= ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_equals(expreval_t* ee) {
    ee_compare(ee);
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && ((ee->code[ee->head] == '=' && ee->code[ee->head + 1] == '=') || (ee->code[ee->head] == '!' && ee->code[ee->head + 1] == '='))) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        int op = ee->code[ee->head] == '=' ? 1 : 2;
        ee->head += 2;
        switch (op) {
        case 1: switch (ee->type) {
            case 1: ee_compare(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (odval == ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (odval == ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee_compare(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (oival == ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (oival == ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        case 2: switch (ee->type) {
            case 1: ee_compare(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (odval != ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (odval != ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            case 0: ee_compare(ee);
                if (ee->error) return;
                switch (ee->type) {
                case 1: ee->ival = (oival != ee->dval)?1:0;
                    ee->type = 0;
                    break;
                case 0: ee->ival = (oival != ee->ival)?1:0;
                    break;
                default: ee->error = 2;
                }
                break;
            default: ee->error = 2;
            }
            break;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_bitand(expreval_t* ee) {
    ee_equals(ee);
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && (ee->code[ee->head] == '&' && !((*__ctype_b_loc ())[(int) ((ee->code[ee->head + 1]))] & (unsigned short int) _ISpunct))) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        ee->head++;
        switch (ee->type) {
        case 1: ee_equals(ee);
            if (ee->error) return;
            switch (ee->type) {
            case 1: ee->ival = (lilint_t)odval & (lilint_t)ee->dval;
                ee->type = 0;
                break;
            case 0: ee->ival = (lilint_t)odval & ee->ival;
                break;
            default: ee->error = 2;
            }
            break;
        case 0: ee_equals(ee);
            if (ee->error) return;
            switch (ee->type) {
            case 1: ee->ival = oival & (lilint_t)ee->dval;
                ee->type = 0;
                break;
            case 0: ee->ival = oival & ee->ival;
                break;
            default: ee->error = 2;
            }
            break;
        default: ee->error = 2;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_bitor(expreval_t* ee) {
    ee_bitand(ee);
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && (ee->code[ee->head] == '|' && !((*__ctype_b_loc ())[(int) ((ee->code[ee->head + 1]))] & (unsigned short int) _ISpunct))) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        ee->head++;
        switch (ee->type) {
        case 1: ee_bitand(ee);
            if (ee->error) return;
            switch (ee->type) {
            case 1: ee->ival = (lilint_t)odval | (lilint_t)ee->dval;
                ee->type = 0;
                break;
            case 0: ee->ival = (lilint_t)odval | ee->ival;
                break;
            default: ee->error = 2;
            }
            break;
        case 0: ee_bitand(ee);
            if (ee->error) return;
            switch (ee->type) {
            case 1: ee->ival = oival | (lilint_t)ee->dval;
                ee->type = 0;
                break;
            case 0: ee->ival = oival | ee->ival;
                break;
            default: ee->error = 2;
            }
            break;
        default: ee->error = 2;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_logand(expreval_t* ee) {
    ee_bitor(ee);
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && (ee->code[ee->head] == '&' && ee->code[ee->head + 1] == '&')) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        ee->head += 2;
        switch (ee->type) {
        case 1: ee_bitor(ee);
            if (ee->error) return;
            switch (ee->type) {
            case 1: ee->ival = (odval && ee->dval)?1:0;
                ee->type = 0;
                break;
            case 0: ee->ival = (odval && ee->ival)?1:0;
                break;
            default: ee->error = 2;
            }
            break;
        case 0: ee_bitor(ee);
            if (ee->error) return;
            switch (ee->type) {
            case 1: ee->ival = (oival && ee->dval)?1:0;
                ee->type = 0;
                break;
            case 0: ee->ival = (oival && ee->ival)?1:0;
                break;
            default: ee->error = 2;
            }
            break;
        default: ee->error = 2;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_logor(expreval_t* ee) {
    ee_logand(ee);
    ee_skip_spaces(ee);
    while (ee->head < ee->len && !ee->error && (ee->code[ee->head] == '|' && ee->code[ee->head + 1] == '|')) {
        double odval = ee->dval;
        lilint_t oival = ee->ival;
        ee->head += 2;
        switch (ee->type) {
        case 1: ee_logand(ee);
            if (ee->error) return;
            switch (ee->type) {
            case 1: ee->ival = (odval || ee->dval)?1:0;
                ee->type = 0;
                break;
            case 0: ee->ival = (odval || ee->ival)?1:0;
                break;
            default: ee->error = 2;
            }
            break;
        case 0: ee_logand(ee);
            if (ee->error) return;
            switch (ee->type) {
            case 1: ee->ival = (oival || ee->dval)?1:0;
                ee->type = 0;
                break;
            case 0: ee->ival = (oival || ee->ival)?1:0;
                break;
            default: ee->error = 2;
            }
            break;
        default: ee->error = 2;
        }
        ee_skip_spaces(ee);
    }
}
// [LOCAL: lil.c]
static void ee_expr(expreval_t* ee) {
    ee_logor(ee);
    if (ee->error == 4) {
        ee->error = 0;
        ee->ival = 1;
    }
}
// [LOCAL: lil.c]
lil_value_t lil_eval_expr(lil_t lil, lil_value_t code) {
    expreval_t ee;
    code = lil_subst_to_value(lil, code);
    if (lil->error) return ((void*)0);
    ee.code = lil_to_string(code);
    if (!ee.code[0]) {
        lil_free_value(code);
        return lil_alloc_integer(0);
    }
    ee.head = 0;
    ee.len = code->l;
    ee.ival = 0;
    ee.dval = 0;
    ee.type = 0;
    ee.error = 0;
    ee_expr(&ee);
    lil_free_value(code);
    if (ee.error) {
        switch (ee.error) {
        case 3: lil_set_error(lil, "division by zero in expression");
            break;
        case 2: lil_set_error(lil, "mixing invalid types in expression");
            break;
        case 1: lil_set_error(lil, "expression syntax error");
            break;
        }
        return ((void*)0);
    }
    if (ee.type == 0) return lil_alloc_integer(ee.ival);
    else return lil_alloc_double(ee.dval);
}
// [LOCAL: lil.c]
lil_value_t lil_unused_name(lil_t lil, const char* part) {
    char* name = malloc(strlen(part) + 64);
    lil_value_t val;
    size_t i;
    for (i=0; i<(size_t)-1; i++) {
        sprintf(name, "!!un!%s!%09u!nu!!", part, (unsigned int)i);
        if (find_cmd(lil, name)) continue;
        if (lil_find_var(lil, lil->env, name)) continue;
        val = lil_alloc_string(name);
        free(name);
        return val;
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
lil_value_t lil_arg(lil_value_t* argv, size_t index) {
    return argv ? argv[index] : ((void*)0);
}
// [LOCAL: lil.c]
const char* lil_to_string(lil_value_t val) {
    return (val && val->d) ? val->d : "";
}
// [LOCAL: lil.c]
double lil_to_double(lil_value_t val) {
    return atof(lil_to_string(val));
}
// [LOCAL: lil.c]
lilint_t lil_to_integer(lil_value_t val) {
    return (lilint_t)atoll(lil_to_string(val));
}
// [LOCAL: lil.c]
int lil_to_boolean(lil_value_t val) {
    const char* s = lil_to_string(val);
    size_t i, dots = 0;
    if (!s[0]) {return 0;}
    for (i=0; s[i]; i++) {
        if (s[i] != '0' && s[i] != '.') return 1;
        if (s[i] == '.') {
            if (dots) return 1;
            dots = 1;
        }
    }
    return 0;
}
// [LOCAL: lil.c]
lil_value_t lil_alloc_string(const char* str) {
    return alloc_value(str);
}
// [LOCAL: lil.c]
lil_value_t lil_alloc_double(double num) {
    char buff[128];
    sprintf(buff, "%f", num);
    return alloc_value(buff);
}
// [LOCAL: lil.c]
lil_value_t lil_alloc_integer(lilint_t num) {
    char buff[128];
    sprintf(buff, "%lli", (lilint_t)num);
    return alloc_value(buff);
}
// [LOCAL: lil.c]
void lil_free(lil_t lil) {
    size_t i;
    if (!lil) return;
    free(lil->err_msg);
    lil_free_value(lil->empty);
    while (lil->env) {
        lil_env_t next = lil->env->parent;
        lil_free_env(lil->env);
        lil->env = next;
    }
    for (i=0; i<lil->cmds; i++) {
        if (lil->cmd[i]->argnames) lil_free_list(lil->cmd[i]->argnames);
        lil_free_value(lil->cmd[i]->code);
        free(lil->cmd[i]->name);
        free(lil->cmd[i]);
    }
    free(lil->cmd);
    free(lil->dollarprefix);
    free(lil->catcher);
    free(lil);
}
// [LOCAL: lil.c]
       void lil_set_data(lil_t lil, void* data) {
    lil->data = data;
}
// [LOCAL: lil.c]
       void* lil_get_data(lil_t lil) {
    return lil->data;
}
// [LOCAL: lil.c]
static lil_value_t fnc_reflect(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_func_t func;
    const char* type;
    size_t i;
    lil_value_t r;
    if (!argc) return ((void*)0);
    type = lil_to_string(argv[0]);
    if (!strcmp(type, "version")) {
        return lil_alloc_string("0.1");
    }
    if (!strcmp(type, "args")) {
        if (argc < 2) return ((void*)0);
        func = find_cmd(lil, lil_to_string(argv[1]));
        if (!func || !func->argnames) return ((void*)0);
        return lil_list_to_value(func->argnames, 1);
    }
    if (!strcmp(type, "body")) {
        if (argc < 2) return ((void*)0);
        func = find_cmd(lil, lil_to_string(argv[1]));
        if (!func || func->proc) return ((void*)0);
        return lil_clone_value(func->code);
    }
    if (!strcmp(type, "func-count")) {
        return lil_alloc_integer(lil->cmds);
    }
    if (!strcmp(type, "funcs")) {
        lil_list_t funcs = lil_alloc_list();
        for (i=0; i<lil->cmds; i++) lil_list_append(funcs, lil_alloc_string(lil->cmd[i]->name));
        r = lil_list_to_value(funcs, 1);
        lil_free_list(funcs);
        return r;
    }
    if (!strcmp(type, "vars")) {
        lil_list_t vars = lil_alloc_list();
        lil_env_t env = lil->env;
        while (env) {
            for (i=0; i<env->vars; i++) lil_list_append(vars, lil_alloc_string(env->var[i]->n));
            env = env->parent;
        }
        r = lil_list_to_value(vars, 1);
        lil_free_list(vars);
        return r;
    }
    if (!strcmp(type, "globals")) {
        lil_list_t vars = lil_alloc_list();
        for (i=0; i<lil->rootenv->vars; i++) lil_list_append(vars, lil_alloc_string(lil->rootenv->var[i]->n));
        r = lil_list_to_value(vars, 1);
        lil_free_list(vars);
        return r;
    }
    if (!strcmp(type, "has-func")) {
        const char* target;
        if (argc == 1) return ((void*)0);
        target = lil_to_string(argv[1]);
        for (i=0; i<lil->cmds; i++) if (!strcmp(target, lil->cmd[i]->name)) return lil_alloc_string("1");
        return ((void*)0);
    }
    if (!strcmp(type, "has-var")) {
        const char* target;
        lil_env_t env = lil->env;
        if (argc == 1) return ((void*)0);
        target = lil_to_string(argv[1]);
        while (env) {
            for (i=0; i<env->vars; i++) if (!strcmp(target, env->var[i]->n)) return lil_alloc_string("1");
            env = env->parent;
        }
        return ((void*)0);
    }
    if (!strcmp(type, "has-global")) {
        const char* target;
        if (argc == 1) return ((void*)0);
        target = lil_to_string(argv[1]);
        for (i=0; i<lil->rootenv->vars; i++) if (!strcmp(target, lil->rootenv->var[i]->n)) return lil_alloc_string("1");
        return ((void*)0);
    }
    if (!strcmp(type, "error")) {
        return lil->err_msg ? lil_alloc_string(lil->err_msg) : ((void*)0);
    }
    if (!strcmp(type, "dollar-prefix")) {
        lil_value_t r;
        if (argc == 1) return lil_alloc_string(lil->dollarprefix);
        r = lil_alloc_string(lil->dollarprefix);
        free(lil->dollarprefix);
        lil->dollarprefix = strclone(lil_to_string(argv[1]));
        return r;
    }
    if (!strcmp(type, "this")) {
        lil_env_t env = lil->env;
        while (env != lil->rootenv && !env->catcher_for && !env->func) env = env->parent;
        if (env->catcher_for) return lil_alloc_string(lil->catcher);
        if (env == lil->rootenv) return lil_alloc_string(lil->rootcode);
        return env->func ? env->func->code : ((void*)0);
    }
    if (!strcmp(type, "name")) {
        lil_env_t env = lil->env;
        while (env != lil->rootenv && !env->catcher_for && !env->func) env = env->parent;
        if (env->catcher_for) return env->catcher_for;
        if (env == lil->rootenv) return ((void*)0);
        return env->func ? lil_alloc_string(env->func->name) : ((void*)0);
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_func(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t name;
    lil_func_t cmd;
    if (argc < 1) return ((void*)0);
    if (argc == 3) {
        name = lil_clone_value(argv[0]);
        cmd = add_func(lil, lil_to_string(argv[0]));
        cmd->argnames = lil_subst_to_list(lil, argv[1]);
        cmd->code = lil_clone_value(argv[2]);
    } else {
        name = lil_unused_name(lil, "anonymous-function");
        cmd = add_func(lil, lil_to_string(name));
        if (argc < 2) {
            lil_value_t tmp = lil_alloc_string("args");
            cmd->argnames = lil_subst_to_list(lil, tmp);
            lil_free_value(tmp);
            cmd->code = lil_clone_value(argv[0]);
        } else {
            cmd->argnames = lil_subst_to_list(lil, argv[0]);
            cmd->code = lil_clone_value(argv[1]);
        }
    }
    return name;
}
// [LOCAL: lil.c]
static lil_value_t fnc_rename(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t r;
    lil_func_t func;
    const char* oldname;
    const char* newname;
    if (argc < 2) return ((void*)0);
    oldname = lil_to_string(argv[0]);
    newname = lil_to_string(argv[1]);
    func = find_cmd(lil, oldname);
    if (!func) {
        char* msg = malloc(24 + strlen(oldname));
        sprintf(msg, "unknown function '%s'", oldname);
        lil_set_error_at(lil, lil->head, msg);
        free(msg);
        return ((void*)0);
    }
    r = lil_alloc_string(func->name);
    free(func->name);
    func->name = strclone(newname);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_unusedname(lil_t lil, size_t argc, lil_value_t* argv) {
    return lil_unused_name(lil, argc > 0 ? lil_to_string(argv[0]) : "unusedname");
}
// [LOCAL: lil.c]
static lil_value_t fnc_quote(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t r;
    size_t i;
    if (argc < 1) return ((void*)0);
    r = alloc_value(((void*)0));
    for (i=0; i<argc; i++) {
        if (i) lil_append_char(r, ' ');
        lil_append_val(r, argv[i]);
    }
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_set(lil_t lil, size_t argc, lil_value_t* argv) {
    size_t i = 0;
    lil_var_t var = ((void*)0);
    int access = 1;
    if (!argc) return ((void*)0);
    if (!strcmp(lil_to_string(argv[0]), "global")) {
        i = 1;
        access = 0;
    }
    while (i < argc) {
        if (argc == i + 1) return lil_clone_value(lil_get_var(lil, lil_to_string(argv[i])));
        var = lil_set_var(lil, lil_to_string(argv[i]), argv[i + 1], access);
        i += 2;
    }
    return var ? lil_clone_value(var->v) : ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_local(lil_t lil, size_t argc, lil_value_t* argv) {
    size_t i;
    for (i=0; i<argc; i++) {
        const char* varname = lil_to_string(argv[i]);
        if (!lil_find_local_var(lil, lil->env, varname)) lil_set_var(lil, varname, lil->empty, 2);
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_write(lil_t lil, size_t argc, lil_value_t* argv) {
    size_t i;
    lil_value_t msg = lil_alloc_string(((void*)0));
    for (i=0; i<argc; i++) {
        if (i) lil_append_char(msg, ' ');
        lil_append_val(msg, argv[i]);
    }
    if (lil->callback[1]) {
        lil_write_callback_proc_t proc = (lil_write_callback_proc_t)lil->callback[1];
        proc(lil, lil_to_string(msg));
    } else printf("%s", lil_to_string(msg));
    lil_free_value(msg);
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_print(lil_t lil, size_t argc, lil_value_t* argv) {
    fnc_write(lil, argc, argv);
    if (lil->callback[1]) {
        lil_write_callback_proc_t proc = (lil_write_callback_proc_t)lil->callback[1];
        proc(lil, "\n");
    } else printf("\n");
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_eval(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc == 1) return lil_parse_value(lil, argv[0], 0);
    if (argc > 1) {
        lil_value_t val = alloc_value(((void*)0)), r;
        size_t i;
        for (i=0; i<argc; i++) {
            if (i) lil_append_char(val, ' ');
            lil_append_val(val, argv[i]);
        }
        r = lil_parse_value(lil, val, 0);
        lil_free_value(val);
        return r;
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_topeval(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_env_t thisenv = lil->env;
    lil_env_t thisdownenv = lil->downenv;
    lil_value_t r;
    lil->env = lil->rootenv;
    lil->downenv = thisenv;
    r = fnc_eval(lil, argc, argv);
    lil->downenv = thisdownenv;
    lil->env = thisenv;
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_upeval(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_env_t thisenv = lil->env;
    lil_env_t thisdownenv = lil->downenv;
    lil_value_t r;
    if (lil->rootenv == thisenv) return fnc_eval(lil, argc, argv);
    lil->env = thisenv->parent;
    lil->downenv = thisenv;
    r = fnc_eval(lil, argc, argv);
    lil->env = thisenv;
    lil->downenv = thisdownenv;
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_downeval(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t r;
    lil_env_t upenv = lil->env;
    lil_env_t downenv = lil->downenv;
    if (!downenv) return fnc_eval(lil, argc, argv);
    lil->downenv = ((void*)0);
    lil->env = downenv;
    r = fnc_eval(lil, argc, argv);
    lil->downenv = downenv;
    lil->env = upenv;
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_enveval(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t r;
    lil_list_t invars = ((void*)0);
    lil_list_t outvars = ((void*)0);
    lil_value_t* varvalues = ((void*)0);
    int codeindex;
    size_t i;
    if (argc < 1) return ((void*)0);
    if (argc == 1) codeindex = 0;
    else if (argc >= 2) {
        invars = lil_subst_to_list(lil, argv[0]);
        varvalues = malloc(sizeof(lil_value_t)*lil_list_size(invars));
        for (i=0; i<lil_list_size(invars); i++) varvalues[i] = lil_clone_value(lil_get_var(lil, lil_to_string(lil_list_get(invars, i))));
        if (argc > 2) {
            codeindex = 2;
            outvars = lil_subst_to_list(lil, argv[1]);
        } else {
            codeindex = 1;
        }
    }
    lil_push_env(lil);
    if (invars) {
        for (i=0; i<lil_list_size(invars); i++) {
            lil_set_var(lil, lil_to_string(lil_list_get(invars, i)), varvalues[i], 2);
            lil_free_value(varvalues[i]);
        }
    }
    r = lil_parse_value(lil, argv[codeindex], 0);
    if (invars || outvars) {
        if (outvars) {
            varvalues = realloc(varvalues, sizeof(lil_value_t)*lil_list_size(outvars));
            for (i=0; i<lil_list_size(outvars); i++) varvalues[i] = lil_clone_value(lil_get_var(lil, lil_to_string(lil_list_get(outvars, i))));
        } else {
            for (i=0; i<lil_list_size(invars); i++) varvalues[i] = lil_clone_value(lil_get_var(lil, lil_to_string(lil_list_get(invars, i))));
        }
    }
    lil_pop_env(lil);
    if (invars) {
        if (outvars) {
            for (i=0; i<lil_list_size(outvars); i++) {
                lil_set_var(lil, lil_to_string(lil_list_get(outvars, i)), varvalues[i], 1);
                lil_free_value(varvalues[i]);
            }
        } else {
            for (i=0; i<lil_list_size(invars); i++) {
                lil_set_var(lil, lil_to_string(lil_list_get(invars, i)), varvalues[i], 1);
                lil_free_value(varvalues[i]);
            }
        }
        lil_free_list(invars);
        if (outvars) lil_free_list(outvars);
        free(varvalues);
    }
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_jaileval(lil_t lil, size_t argc, lil_value_t* argv) {
    size_t i;
    lil_t sublil;
    lil_value_t r;
    size_t base = 0;
    if (!argc) return ((void*)0);
    if (!strcmp(lil_to_string(argv[0]), "clean")) {
        base = 1;
        if (argc == 1) return ((void*)0);
    }
    sublil = lil_new();
    if (base != 1) {
        for (i=lil->syscmds; i<lil->cmds; i++) {
            lil_func_t fnc = lil->cmd[i];
            if (!fnc->proc) continue;
            lil_register(sublil, fnc->name, fnc->proc);
        }
    }
    r = lil_parse_value(sublil, argv[base], 1);
    lil_free(sublil);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_count(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list;
    char buff[64];
    if (!argc) return alloc_value("0");
    list = lil_subst_to_list(lil, argv[0]);
    sprintf(buff, "%u", (unsigned int)list->c);
    lil_free_list(list);
    return alloc_value(buff);
}
// [LOCAL: lil.c]
static lil_value_t fnc_index(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list;
    size_t index;
    lil_value_t r;
    if (argc < 2) return ((void*)0);
    list = lil_subst_to_list(lil, argv[0]);
    index = (size_t)lil_to_integer(argv[1]);
    if (index >= list->c) r = ((void*)0);
    else r = lil_clone_value(list->v[index]);
    lil_free_list(list);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_indexof(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list;
    size_t index;
    lil_value_t r = ((void*)0);
    if (argc < 2) return ((void*)0);
    list = lil_subst_to_list(lil, argv[0]);
    for (index = 0; index < list->c; index++) if (!strcmp(lil_to_string(list->v[index]), lil_to_string(argv[1]))) {
            r = lil_alloc_integer(index);
            break;
        }
    lil_free_list(list);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_append(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list;
    lil_value_t r;
    size_t i, base = 1;
    int access = 1;
    const char* varname;
    if (argc < 2) return ((void*)0);
    varname = lil_to_string(argv[0]);
    if (!strcmp(varname, "global")) {
        if (argc < 3) return ((void*)0);
        varname = lil_to_string(argv[1]);
        base = 2;
        access = 0;
    }
    list = lil_subst_to_list(lil, lil_get_var(lil, varname));
    for (i=base; i<argc; i++) lil_list_append(list, lil_clone_value(argv[i]));
    r = lil_list_to_value(list, 1);
    lil_free_list(list);
    lil_set_var(lil, varname, r, access);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_slice(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list, slice;
    size_t i;
    lilint_t from, to;
    lil_value_t r;
    if (argc < 1) return ((void*)0);
    if (argc < 2) return lil_clone_value(argv[0]);
    from = lil_to_integer(argv[1]);
    if (from < 0) from = 0;
    list = lil_subst_to_list(lil, argv[0]);
    to = argc > 2 ? lil_to_integer(argv[2]) : (lilint_t)list->c;
    if (to > (lilint_t)list->c) to = list->c;
    if (to < from) to = from;
    slice = lil_alloc_list();
    for (i=(size_t)from; i<(size_t)to; i++) lil_list_append(slice, lil_clone_value(list->v[i]));
    lil_free_list(list);
    r = lil_list_to_value(slice, 1);
    lil_free_list(slice);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_filter(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list, filtered;
    size_t i;
    lil_value_t r;
    const char* varname = "x";
    int base = 0;
    if (argc < 1) return ((void*)0);
    if (argc < 2) return lil_clone_value(argv[0]);
    if (argc > 2) {
        base = 1;
        varname = lil_to_string(argv[0]);
    }
    list = lil_subst_to_list(lil, argv[base]);
    filtered = lil_alloc_list();
    for (i=0; i<list->c && !lil->env->breakrun; i++) {
        lil_set_var(lil, varname, list->v[i], 3);
        r = lil_eval_expr(lil, argv[base + 1]);
        if (lil_to_boolean(r)) lil_list_append(filtered, lil_clone_value(list->v[i]));
        lil_free_value(r);
    }
    lil_free_list(list);
    r = lil_list_to_value(filtered, 1);
    lil_free_list(filtered);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_list(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list = lil_alloc_list();
    lil_value_t r;
    size_t i;
    for (i=0; i<argc; i++) lil_list_append(list, lil_clone_value(argv[i]));
    r = lil_list_to_value(list, 1);
    lil_free_list(list);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_subst(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc < 1) return ((void*)0);
    return lil_subst_to_value(lil, argv[0]);
}
// [LOCAL: lil.c]
static lil_value_t fnc_concat(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list;
    lil_value_t r, tmp;
    size_t i;
    if (argc < 1) return ((void*)0);
    r = lil_alloc_string("");
    for (i=0; i<argc; i++) {
        list = lil_subst_to_list(lil, argv[i]);
        tmp = lil_list_to_value(list, 1);
        lil_free_list(list);
        lil_append_val(r, tmp);
        lil_free_value(tmp);
    }
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_foreach(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list, rlist;
    lil_value_t r;
    size_t i, listidx = 0, codeidx = 1;
    const char* varname = "i";
    if (argc < 2) return ((void*)0);
    if (argc >= 3) {
        varname = lil_to_string(argv[0]);
        listidx = 1;
        codeidx = 2;
    }
    rlist = lil_alloc_list();
    list = lil_subst_to_list(lil, argv[listidx]);
    for (i=0; i<list->c; i++) {
        lil_value_t rv;
        lil_set_var(lil, varname, list->v[i], 3);
        rv = lil_parse_value(lil, argv[codeidx], 0);
        if (rv->l) lil_list_append(rlist, rv);
        else lil_free_value(rv);
        if (lil->env->breakrun || lil->error) break;
    }
    r = lil_list_to_value(rlist, 1);
    lil_free_list(list);
    lil_free_list(rlist);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_return(lil_t lil, size_t argc, lil_value_t* argv) {
    lil->env->breakrun = 1;
    lil_free_value(lil->env->retval);
    lil->env->retval = argc < 1 ? ((void*)0) : lil_clone_value(argv[0]);
    lil->env->retval_set = 1;
    return argc < 1 ? ((void*)0) : lil_clone_value(argv[0]);
}
// [LOCAL: lil.c]
static lil_value_t fnc_result(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc > 0) {
        lil_free_value(lil->env->retval);
        lil->env->retval = lil_clone_value(argv[0]);
        lil->env->retval_set = 1;
    }
    return lil->env->retval_set ? lil_clone_value(lil->env->retval) : ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_expr(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc == 1) return lil_eval_expr(lil, argv[0]);
    if (argc > 1) {
        lil_value_t val = alloc_value(((void*)0)), r;
        size_t i;
        for (i=0; i<argc; i++) {
            if (i) lil_append_char(val, ' ');
            lil_append_val(val, argv[i]);
        }
        r = lil_eval_expr(lil, val);
        lil_free_value(val);
        return r;
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t real_inc(lil_t lil, const char* varname, float v) {
    lil_value_t pv = lil_get_var(lil, varname);
    double dv = lil_to_double(pv) + v;
    if (fmod(dv, 1)) pv = lil_alloc_double(dv);
    else pv = lil_alloc_integer(lil_to_integer(pv) + v);
    lil_set_var(lil, varname, pv, 1);
    return pv;
}
// [LOCAL: lil.c]
static lil_value_t fnc_inc(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc < 1) return ((void*)0);
    return real_inc(lil, lil_to_string(argv[0]), argc > 1 ? lil_to_double(argv[1]) : 1);
}
// [LOCAL: lil.c]
static lil_value_t fnc_dec(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc < 1) return ((void*)0);
    return real_inc(lil, lil_to_string(argv[0]), -(argc > 1 ? lil_to_double(argv[1]) : 1));
}
// [LOCAL: lil.c]
static lil_value_t fnc_read(lil_t lil, size_t argc, lil_value_t* argv) {
    FILE* f;
    size_t size;
    char* buffer;
    lil_value_t r;
    if (argc < 1) return ((void*)0);
    if (lil->callback[2]) {
        lil_read_callback_proc_t proc = (lil_read_callback_proc_t) lil->callback[2];
        buffer = proc(lil, lil_to_string(argv[0]));
    } else {
        f = fopen(lil_to_string(argv[0]), "rb");
        if (!f) return ((void*)0);
        fseek(f, 0, 2);
        size = ftell(f);
        fseek(f, 0, 0);
        buffer = malloc(size + 1);
        fread(buffer, 1, size, f);
        buffer[size] = 0;
        fclose(f);
    }
    r = lil_alloc_string(buffer);
    free(buffer);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_store(lil_t lil, size_t argc, lil_value_t* argv) {
    FILE* f;
    const char* buffer;
    if (argc < 2) return ((void*)0);
    if (lil->callback[3]) {
        lil_store_callback_proc_t proc = (lil_store_callback_proc_t)lil->callback[3];
        proc(lil, lil_to_string(argv[0]), lil_to_string(argv[1]));
    } else {
        f = fopen(lil_to_string(argv[0]), "wb");
        if (!f) return ((void*)0);
        buffer = lil_to_string(argv[1]);
        fwrite(buffer, 1, strlen(buffer), f);
        fclose(f);
    }
    return lil_clone_value(argv[1]);
}
// [LOCAL: lil.c]
static lil_value_t fnc_if(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t val, r = ((void*)0);
    int base = 0, not = 0, v;
    if (argc < 1) return ((void*)0);
    if (!strcmp(lil_to_string(argv[0]), "not")) base = not = 1;
    if (argc < (size_t)base + 2) return ((void*)0);
    val = lil_eval_expr(lil, argv[base]);
    if (!val || lil->error) return ((void*)0);
    v = lil_to_boolean(val);
    if (not) v = !v;
    if (v) {
        r = lil_parse_value(lil, argv[base + 1], 0);
    } else if (argc > (size_t)base + 2) {
        r = lil_parse_value(lil, argv[base + 2], 0);
    }
    lil_free_value(val);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_while(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t val, r = ((void*)0);
    int base = 0, not = 0, v;
    if (argc < 1) return ((void*)0);
    if (!strcmp(lil_to_string(argv[0]), "not")) base = not = 1;
    if (argc < (size_t)base + 2) return ((void*)0);
    while (!lil->error && !lil->env->breakrun) {
        val = lil_eval_expr(lil, argv[base]);
        if (!val || lil->error) return ((void*)0);
        v = lil_to_boolean(val);
        if (not) v = !v;
        if (!v) {
            lil_free_value(val);
            break;
        }
        if (r) lil_free_value(r);
        r = lil_parse_value(lil, argv[base + 1], 0);
        lil_free_value(val);
    }
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_for(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t val, r = ((void*)0);
    if (argc < 4) return ((void*)0);
    lil_free_value(lil_parse_value(lil, argv[0], 0));
    while (!lil->error && !lil->env->breakrun) {
        val = lil_eval_expr(lil, argv[1]);
        if (!val || lil->error) return ((void*)0);
        if (!lil_to_boolean(val)) {
            lil_free_value(val);
            break;
        }
        if (r) lil_free_value(r);
        r = lil_parse_value(lil, argv[3], 0);
        lil_free_value(val);
        lil_free_value(lil_parse_value(lil, argv[2], 0));
    }
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_char(lil_t lil, size_t argc, lil_value_t* argv) {
    char s[2];
    if (!argc) return ((void*)0);
    s[0] = (char)lil_to_integer(argv[0]);
    s[1] = 0;
    return lil_alloc_string(s);
}
// [LOCAL: lil.c]
static lil_value_t fnc_charat(lil_t lil, size_t argc, lil_value_t* argv) {
    size_t index;
    char chstr[2];
    const char* str;
    if (argc < 2) return ((void*)0);
    str = lil_to_string(argv[0]);
    index = (size_t)lil_to_integer(argv[1]);
    if (index >= strlen(str)) return ((void*)0);
    chstr[0] = str[index];
    chstr[1] = 0;
    return lil_alloc_string(chstr);
}
// [LOCAL: lil.c]
static lil_value_t fnc_codeat(lil_t lil, size_t argc, lil_value_t* argv) {
    size_t index;
    const char* str;
    if (argc < 2) return ((void*)0);
    str = lil_to_string(argv[0]);
    index = (size_t)lil_to_integer(argv[1]);
    if (index >= strlen(str)) return ((void*)0);
    return lil_alloc_integer(str[index]);
}
// [LOCAL: lil.c]
static lil_value_t fnc_substr(lil_t lil, size_t argc, lil_value_t* argv) {
    const char* str;
    lil_value_t r;
    size_t start, end, i, slen;
    if (argc < 2) return ((void*)0);
    str = lil_to_string(argv[0]);
    if (!str[0]) return ((void*)0);
    slen = strlen(str);
    start = (size_t)atoll(lil_to_string(argv[1]));
    end = argc > 2 ? (size_t)atoll(lil_to_string(argv[2])) : slen;
    if (end > slen) end = slen;
    if (start >= end) return ((void*)0);
    r = lil_alloc_string("");
    for (i=start; i<end; i++) lil_append_char(r, str[i]);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_strpos(lil_t lil, size_t argc, lil_value_t* argv) {
    const char* hay;
    const char* str;
    size_t min = 0;
    if (argc < 2) return lil_alloc_integer(-1);
    hay = lil_to_string(argv[0]);
    if (argc > 2) {
        min = (size_t)atoll(lil_to_string(argv[2]));
        if (min >= strlen(hay)) return lil_alloc_integer(-1);
    }
    str = strstr(hay + min, lil_to_string(argv[1]));
    if (!str) return lil_alloc_integer(-1);
    return lil_alloc_integer(str - hay);
}
// [LOCAL: lil.c]
static lil_value_t fnc_length(lil_t lil, size_t argc, lil_value_t* argv) {
    size_t i, total = 0;
    for (i=0; i<argc; i++) {
        if (i) total++;
        total += strlen(lil_to_string(argv[i]));
    }
    return lil_alloc_integer((lilint_t)total);
}
// [LOCAL: lil.c]
static lil_value_t real_trim(const char* str, const char* chars, int left, int right) {
    int base = 0;
    lil_value_t r = ((void*)0);
    if (left) {
        while (str[base] && strchr(chars, str[base])) base++;
        if (!right) r = lil_alloc_string(str[base] ? str + base : ((void*)0));
    }
    if (right) {
        size_t len;
        char* s;
        s = strclone(str + base);
        len = strlen(s);
        while (len && strchr(chars, s[len - 1])) len--;
        s[len] = 0;
        r = lil_alloc_string(s);
        free(s);
    }
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_trim(lil_t lil, size_t argc, lil_value_t* argv) {
    if (!argc) return ((void*)0);
    return real_trim(lil_to_string(argv[0]), argc < 2 ? " \f\n\r\t\v" : lil_to_string(argv[1]), 1, 1);
}
// [LOCAL: lil.c]
static lil_value_t fnc_ltrim(lil_t lil, size_t argc, lil_value_t* argv) {
    if (!argc) return ((void*)0);
    return real_trim(lil_to_string(argv[0]), argc < 2 ? " \f\n\r\t\v" : lil_to_string(argv[1]), 1, 0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_rtrim(lil_t lil, size_t argc, lil_value_t* argv) {
    if (!argc) return ((void*)0);
    return real_trim(lil_to_string(argv[0]), argc < 2 ? " \f\n\r\t\v" : lil_to_string(argv[1]), 0, 1);
}
// [LOCAL: lil.c]
static lil_value_t fnc_strcmp(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc < 2) return ((void*)0);
    return lil_alloc_integer(strcmp(lil_to_string(argv[0]), lil_to_string(argv[1])));
}
// [LOCAL: lil.c]
static lil_value_t fnc_streq(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc < 2) return ((void*)0);
    return lil_alloc_integer(strcmp(lil_to_string(argv[0]), lil_to_string(argv[1]))?0:1);
}
// [LOCAL: lil.c]
static lil_value_t fnc_repstr(lil_t lil, size_t argc, lil_value_t* argv) {
    const char* from;
    const char* to;
    char* src;
    const char* sub;
    size_t idx;
    size_t fromlen;
    size_t tolen;
    size_t srclen;
    lil_value_t r;
    if (argc < 1) return ((void*)0);
    if (argc < 3) return lil_clone_value(argv[0]);
    from = lil_to_string(argv[1]);
    to = lil_to_string(argv[2]);
    if (!from[0]) return ((void*)0);
    src = strclone(lil_to_string(argv[0]));
    srclen = strlen(src);
    fromlen = strlen(from);
    tolen = strlen(to);
    while ((sub = strstr(src, from))) {
        char* newsrc = malloc(srclen - fromlen + tolen + 1);
        idx = sub - src;
        if (idx) memcpy(newsrc, src, idx);
        memcpy(newsrc + idx, to, tolen);
        memcpy(newsrc + idx + tolen, src + idx + fromlen, srclen - idx - fromlen);
        srclen = srclen - fromlen + tolen;
        free(src);
        src = newsrc;
        src[srclen] = 0;
    }
    r = lil_alloc_string(src);
    free(src);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_split(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list;
    const char* sep = " ";
    size_t i;
    lil_value_t val;
    const char* str;
    if (argc == 0) return ((void*)0);
    if (argc > 1) {
        sep = lil_to_string(argv[1]);
        if (!sep || !sep[0]) return lil_clone_value(argv[0]);
    }
    val = lil_alloc_string("");
    str = lil_to_string(argv[0]);
    list = lil_alloc_list();
    for (i=0; str[i]; i++) {
        if (strchr(sep, str[i])) {
            lil_list_append(list, val);
            val = lil_alloc_string("");
        } else {
            lil_append_char(val, str[i]);
        }
    }
    lil_list_append(list, val);
    val = lil_list_to_value(list, 1);
    lil_free_list(list);
    return val;
}
// [LOCAL: lil.c]
static lil_value_t fnc_try(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_value_t r;
    if (argc < 1) return ((void*)0);
    if (lil->error) return ((void*)0);
    r = lil_parse_value(lil, argv[0], 0);
    if (lil->error) {
        lil->error = 0;
        lil_free_value(r);
        if (argc > 1) r = lil_parse_value(lil, argv[1], 0);
        else r = 0;
    }
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_error(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_set_error(lil, argc > 0 ? lil_to_string(argv[0]) : ((void*)0));
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_exit(lil_t lil, size_t argc, lil_value_t* argv) {
    if (lil->callback[0]) {
        lil_exit_callback_proc_t proc = (lil_exit_callback_proc_t)lil->callback[0];
        proc(lil, argc > 0 ? argv[0] : ((void*)0));
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_source(lil_t lil, size_t argc, lil_value_t* argv) {
    FILE* f;
    size_t size;
    char* buffer;
    lil_value_t r;
    if (argc < 1) return ((void*)0);
    if (lil->callback[4]) {
        lil_source_callback_proc_t proc = (lil_source_callback_proc_t)lil->callback[4];
        buffer = proc(lil, lil_to_string(argv[0]));
    } else if (lil->callback[2]) {
        lil_read_callback_proc_t proc = (lil_read_callback_proc_t)lil->callback[2];
        buffer = proc(lil, lil_to_string(argv[0]));
    } else {
        f = fopen(lil_to_string(argv[0]), "rb");
        if (!f) return ((void*)0);
        fseek(f, 0, 2);
        size = ftell(f);
        fseek(f, 0, 0);
        buffer = malloc(size + 1);
        fread(buffer, 1, size, f);
        buffer[size] = 0;
        fclose(f);
    }
    r = lil_parse(lil, buffer, 0, 0);
    free(buffer);
    return r;
}
// [LOCAL: lil.c]
static lil_value_t fnc_lmap(lil_t lil, size_t argc, lil_value_t* argv) {
    lil_list_t list;
    size_t i;
    if (argc < 2) return ((void*)0);
    list = lil_subst_to_list(lil, argv[0]);
    for (i=1; i<argc; i++) lil_set_var(lil, lil_to_string(argv[i]), lil_list_get(list, i - 1), 1);
    lil_free_list(list);
    return ((void*)0);
}
// [LOCAL: lil.c]
static lil_value_t fnc_rand(lil_t lil, size_t argc, lil_value_t* argv) {
    return lil_alloc_double(rand()/(double)2147483647);
}
// [LOCAL: lil.c]
static lil_value_t fnc_catcher(lil_t lil, size_t argc, lil_value_t* argv) {
    if (argc == 0) {
        return lil_alloc_string(lil->catcher);
    } else {
        const char* catcher = lil_to_string(argv[0]);
        free(lil->catcher);
        lil->catcher = catcher[0] ? strclone(catcher) : ((void*)0);
    }
    return ((void*)0);
}
// [LOCAL: lil.c]
static void register_stdcmds(lil_t lil) {
    lil_register(lil, "reflect", fnc_reflect);
    lil_register(lil, "func", fnc_func);
    lil_register(lil, "rename", fnc_rename);
    lil_register(lil, "unusedname", fnc_unusedname);
    lil_register(lil, "quote", fnc_quote);
    lil_register(lil, "set", fnc_set);
    lil_register(lil, "local", fnc_local);
    lil_register(lil, "write", fnc_write);
    lil_register(lil, "print", fnc_print);
    lil_register(lil, "eval", fnc_eval);
    lil_register(lil, "topeval", fnc_topeval);
    lil_register(lil, "upeval", fnc_upeval);
    lil_register(lil, "downeval", fnc_downeval);
    lil_register(lil, "enveval", fnc_enveval);
    lil_register(lil, "jaileval", fnc_jaileval);
    lil_register(lil, "count", fnc_count);
    lil_register(lil, "index", fnc_index);
    lil_register(lil, "indexof", fnc_indexof);
    lil_register(lil, "filter", fnc_filter);
    lil_register(lil, "list", fnc_list);
    lil_register(lil, "append", fnc_append);
    lil_register(lil, "slice", fnc_slice);
    lil_register(lil, "subst", fnc_subst);
    lil_register(lil, "concat", fnc_concat);
    lil_register(lil, "foreach", fnc_foreach);
    lil_register(lil, "return", fnc_return);
    lil_register(lil, "result", fnc_result);
    lil_register(lil, "expr", fnc_expr);
    lil_register(lil, "inc", fnc_inc);
    lil_register(lil, "dec", fnc_dec);
    lil_register(lil, "read", fnc_read);
    lil_register(lil, "store", fnc_store);
    lil_register(lil, "if", fnc_if);
    lil_register(lil, "while", fnc_while);
    lil_register(lil, "for", fnc_for);
    lil_register(lil, "char", fnc_char);
    lil_register(lil, "charat", fnc_charat);
    lil_register(lil, "codeat", fnc_codeat);
    lil_register(lil, "substr", fnc_substr);
    lil_register(lil, "strpos", fnc_strpos);
    lil_register(lil, "length", fnc_length);
    lil_register(lil, "trim", fnc_trim);
    lil_register(lil, "ltrim", fnc_ltrim);
    lil_register(lil, "rtrim", fnc_rtrim);
    lil_register(lil, "strcmp", fnc_strcmp);
    lil_register(lil, "streq", fnc_streq);
    lil_register(lil, "repstr", fnc_repstr);
    lil_register(lil, "split", fnc_split);
    lil_register(lil, "try", fnc_try);
    lil_register(lil, "error", fnc_error);
    lil_register(lil, "exit", fnc_exit);
    lil_register(lil, "source", fnc_source);
    lil_register(lil, "lmap", fnc_lmap);
    lil_register(lil, "rand", fnc_rand);
    lil_register(lil, "catcher", fnc_catcher);
    lil->syscmds = lil->cmds;
}