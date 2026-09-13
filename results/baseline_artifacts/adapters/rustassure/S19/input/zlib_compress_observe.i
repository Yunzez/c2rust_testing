extern void __assert_fail (const char *__assertion, const char *__file,
      unsigned int __line, const char *__function)
     __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern void __assert_perror_fail (int __errnum, const char *__file,
      unsigned int __line, const char *__function)
     __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern void __assert (const char *__assertion, const char *__file, int __line)
     __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern int *__errno_location (void) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
typedef unsigned char __u_char;
typedef unsigned short int __u_short;
typedef unsigned int __u_int;
typedef unsigned long int __u_long;
typedef signed char __int8_t;
typedef unsigned char __uint8_t;
typedef signed short int __int16_t;
typedef unsigned short int __uint16_t;
typedef signed int __int32_t;
typedef unsigned int __uint32_t;
typedef signed long int __int64_t;
typedef unsigned long int __uint64_t;
typedef __int8_t __int_least8_t;
typedef __uint8_t __uint_least8_t;
typedef __int16_t __int_least16_t;
typedef __uint16_t __uint_least16_t;
typedef __int32_t __int_least32_t;
typedef __uint32_t __uint_least32_t;
typedef __int64_t __int_least64_t;
typedef __uint64_t __uint_least64_t;
typedef long int __quad_t;
typedef unsigned long int __u_quad_t;
typedef long int __intmax_t;
typedef unsigned long int __uintmax_t;
typedef unsigned long int __dev_t;
typedef unsigned int __uid_t;
typedef unsigned int __gid_t;
typedef unsigned long int __ino_t;
typedef unsigned long int __ino64_t;
typedef unsigned int __mode_t;
typedef unsigned long int __nlink_t;
typedef long int __off_t;
typedef long int __off64_t;
typedef int __pid_t;
typedef struct { int __val[2]; } __fsid_t;
typedef long int __clock_t;
typedef unsigned long int __rlim_t;
typedef unsigned long int __rlim64_t;
typedef unsigned int __id_t;
typedef long int __time_t;
typedef unsigned int __useconds_t;
typedef long int __suseconds_t;
typedef long int __suseconds64_t;
typedef int __daddr_t;
typedef int __key_t;
typedef int __clockid_t;
typedef void * __timer_t;
typedef long int __blksize_t;
typedef long int __blkcnt_t;
typedef long int __blkcnt64_t;
typedef unsigned long int __fsblkcnt_t;
typedef unsigned long int __fsblkcnt64_t;
typedef unsigned long int __fsfilcnt_t;
typedef unsigned long int __fsfilcnt64_t;
typedef long int __fsword_t;
typedef long int __ssize_t;
typedef long int __syscall_slong_t;
typedef unsigned long int __syscall_ulong_t;
typedef __off64_t __loff_t;
typedef char *__caddr_t;
typedef long int __intptr_t;
typedef unsigned int __socklen_t;
typedef int __sig_atomic_t;
typedef __int8_t int8_t;
typedef __int16_t int16_t;
typedef __int32_t int32_t;
typedef __int64_t int64_t;
typedef __uint8_t uint8_t;
typedef __uint16_t uint16_t;
typedef __uint32_t uint32_t;
typedef __uint64_t uint64_t;
typedef __int_least8_t int_least8_t;
typedef __int_least16_t int_least16_t;
typedef __int_least32_t int_least32_t;
typedef __int_least64_t int_least64_t;
typedef __uint_least8_t uint_least8_t;
typedef __uint_least16_t uint_least16_t;
typedef __uint_least32_t uint_least32_t;
typedef __uint_least64_t uint_least64_t;
typedef signed char int_fast8_t;
typedef long int int_fast16_t;
typedef long int int_fast32_t;
typedef long int int_fast64_t;
typedef unsigned char uint_fast8_t;
typedef unsigned long int uint_fast16_t;
typedef unsigned long int uint_fast32_t;
typedef unsigned long int uint_fast64_t;
typedef long int intptr_t;
typedef unsigned long int uintptr_t;
typedef __intmax_t intmax_t;
typedef __uintmax_t uintmax_t;
typedef long int ptrdiff_t;
typedef long unsigned int size_t;
typedef int wchar_t;
typedef struct {
  long long __clang_max_align_nonce1
      __attribute__((__aligned__(__alignof__(long long))));
  long double __clang_max_align_nonce2
      __attribute__((__aligned__(__alignof__(long double))));
} max_align_t;
typedef __builtin_va_list va_list;
typedef __builtin_va_list __gnuc_va_list;
typedef struct
{
  int __count;
  union
  {
    unsigned int __wch;
    char __wchb[4];
  } __value;
} __mbstate_t;
typedef struct _G_fpos_t
{
  __off_t __pos;
  __mbstate_t __state;
} __fpos_t;
typedef struct _G_fpos64_t
{
  __off64_t __pos;
  __mbstate_t __state;
} __fpos64_t;
struct _IO_FILE;
typedef struct _IO_FILE __FILE;
struct _IO_FILE;
typedef struct _IO_FILE FILE;
struct _IO_FILE;
struct _IO_marker;
struct _IO_codecvt;
struct _IO_wide_data;
typedef void _IO_lock_t;
struct _IO_FILE
{
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
  int _flags2;
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
  size_t __pad5;
  int _mode;
  char _unused2[15 * sizeof (int) - 4 * sizeof (void *) - sizeof (size_t)];
};
typedef __gnuc_va_list va_list;
typedef __off_t off_t;
typedef __ssize_t ssize_t;
typedef __fpos_t fpos_t;
extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;
extern int remove (const char *__filename) __attribute__ ((__nothrow__ ));
extern int rename (const char *__old, const char *__new) __attribute__ ((__nothrow__ ));
extern int renameat (int __oldfd, const char *__old, int __newfd,
       const char *__new) __attribute__ ((__nothrow__ ));
extern int fclose (FILE *__stream);
extern FILE *tmpfile (void)
  __attribute__ ((__malloc__)) ;
extern char *tmpnam (char[20]) __attribute__ ((__nothrow__ )) ;
extern char *tmpnam_r (char __s[20]) __attribute__ ((__nothrow__ )) ;
extern char *tempnam (const char *__dir, const char *__pfx)
   __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
extern int fflush (FILE *__stream);
extern int fflush_unlocked (FILE *__stream);
extern FILE *fopen (const char *__restrict __filename,
      const char *__restrict __modes)
  __attribute__ ((__malloc__)) ;
extern FILE *freopen (const char *__restrict __filename,
        const char *__restrict __modes,
        FILE *__restrict __stream) ;
extern FILE *fdopen (int __fd, const char *__modes) __attribute__ ((__nothrow__ ))
  __attribute__ ((__malloc__)) ;
extern FILE *fmemopen (void *__s, size_t __len, const char *__modes)
  __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
extern FILE *open_memstream (char **__bufloc, size_t *__sizeloc) __attribute__ ((__nothrow__ ))
  __attribute__ ((__malloc__)) ;
extern void setbuf (FILE *__restrict __stream, char *__restrict __buf) __attribute__ ((__nothrow__ ));
extern int setvbuf (FILE *__restrict __stream, char *__restrict __buf,
      int __modes, size_t __n) __attribute__ ((__nothrow__ ));
extern void setbuffer (FILE *__restrict __stream, char *__restrict __buf,
         size_t __size) __attribute__ ((__nothrow__ ));
extern void setlinebuf (FILE *__stream) __attribute__ ((__nothrow__ ));
extern int fprintf (FILE *__restrict __stream,
      const char *__restrict __format, ...);
extern int printf (const char *__restrict __format, ...);
extern int sprintf (char *__restrict __s,
      const char *__restrict __format, ...) __attribute__ ((__nothrow__));
extern int vfprintf (FILE *__restrict __s, const char *__restrict __format,
       __gnuc_va_list __arg);
extern int vprintf (const char *__restrict __format, __gnuc_va_list __arg);
extern int vsprintf (char *__restrict __s, const char *__restrict __format,
       __gnuc_va_list __arg) __attribute__ ((__nothrow__));
extern int snprintf (char *__restrict __s, size_t __maxlen,
       const char *__restrict __format, ...)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 3, 4)));
extern int vsnprintf (char *__restrict __s, size_t __maxlen,
        const char *__restrict __format, __gnuc_va_list __arg)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 3, 0)));
extern int vdprintf (int __fd, const char *__restrict __fmt,
       __gnuc_va_list __arg)
     __attribute__ ((__format__ (__printf__, 2, 0)));
extern int dprintf (int __fd, const char *__restrict __fmt, ...)
     __attribute__ ((__format__ (__printf__, 2, 3)));
extern int fscanf (FILE *__restrict __stream,
     const char *__restrict __format, ...) ;
extern int scanf (const char *__restrict __format, ...) ;
extern int sscanf (const char *__restrict __s,
     const char *__restrict __format, ...) __attribute__ ((__nothrow__ ));
typedef float _Float32;
typedef double _Float64;
typedef double _Float32x;
typedef long double _Float64x;
extern int fscanf (FILE *__restrict __stream, const char *__restrict __format, ...) __asm__ ("" "__isoc99_fscanf") ;
extern int scanf (const char *__restrict __format, ...) __asm__ ("" "__isoc99_scanf") ;
extern int sscanf (const char *__restrict __s, const char *__restrict __format, ...) __asm__ ("" "__isoc99_sscanf") __attribute__ ((__nothrow__ ));
extern int vfscanf (FILE *__restrict __s, const char *__restrict __format,
      __gnuc_va_list __arg)
     __attribute__ ((__format__ (__scanf__, 2, 0))) ;
extern int vscanf (const char *__restrict __format, __gnuc_va_list __arg)
     __attribute__ ((__format__ (__scanf__, 1, 0))) ;
extern int vsscanf (const char *__restrict __s,
      const char *__restrict __format, __gnuc_va_list __arg)
     __attribute__ ((__nothrow__ )) __attribute__ ((__format__ (__scanf__, 2, 0)));
extern int vfscanf (FILE *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc99_vfscanf")
     __attribute__ ((__format__ (__scanf__, 2, 0))) ;
extern int vscanf (const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc99_vscanf")
     __attribute__ ((__format__ (__scanf__, 1, 0))) ;
extern int vsscanf (const char *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc99_vsscanf") __attribute__ ((__nothrow__ ))
     __attribute__ ((__format__ (__scanf__, 2, 0)));
extern int fgetc (FILE *__stream);
extern int getc (FILE *__stream);
extern int getchar (void);
extern int getc_unlocked (FILE *__stream);
extern int getchar_unlocked (void);
extern int fgetc_unlocked (FILE *__stream);
extern int fputc (int __c, FILE *__stream);
extern int putc (int __c, FILE *__stream);
extern int putchar (int __c);
extern int fputc_unlocked (int __c, FILE *__stream);
extern int putc_unlocked (int __c, FILE *__stream);
extern int putchar_unlocked (int __c);
extern int getw (FILE *__stream);
extern int putw (int __w, FILE *__stream);
extern char *fgets (char *__restrict __s, int __n, FILE *__restrict __stream)
                                                         ;
extern __ssize_t __getdelim (char **__restrict __lineptr,
                             size_t *__restrict __n, int __delimiter,
                             FILE *__restrict __stream) ;
extern __ssize_t getdelim (char **__restrict __lineptr,
                           size_t *__restrict __n, int __delimiter,
                           FILE *__restrict __stream) ;
extern __ssize_t getline (char **__restrict __lineptr,
                          size_t *__restrict __n,
                          FILE *__restrict __stream) ;
extern int fputs (const char *__restrict __s, FILE *__restrict __stream);
extern int puts (const char *__s);
extern int ungetc (int __c, FILE *__stream);
extern size_t fread (void *__restrict __ptr, size_t __size,
       size_t __n, FILE *__restrict __stream) ;
extern size_t fwrite (const void *__restrict __ptr, size_t __size,
        size_t __n, FILE *__restrict __s);
extern size_t fread_unlocked (void *__restrict __ptr, size_t __size,
         size_t __n, FILE *__restrict __stream) ;
extern size_t fwrite_unlocked (const void *__restrict __ptr, size_t __size,
          size_t __n, FILE *__restrict __stream);
extern int fseek (FILE *__stream, long int __off, int __whence);
extern long int ftell (FILE *__stream) ;
extern void rewind (FILE *__stream);
extern int fseeko (FILE *__stream, __off_t __off, int __whence);
extern __off_t ftello (FILE *__stream) ;
extern int fgetpos (FILE *__restrict __stream, fpos_t *__restrict __pos);
extern int fsetpos (FILE *__stream, const fpos_t *__pos);
extern void clearerr (FILE *__stream) __attribute__ ((__nothrow__ ));
extern int feof (FILE *__stream) __attribute__ ((__nothrow__ )) ;
extern int ferror (FILE *__stream) __attribute__ ((__nothrow__ )) ;
extern void clearerr_unlocked (FILE *__stream) __attribute__ ((__nothrow__ ));
extern int feof_unlocked (FILE *__stream) __attribute__ ((__nothrow__ )) ;
extern int ferror_unlocked (FILE *__stream) __attribute__ ((__nothrow__ )) ;
extern void perror (const char *__s);
extern int fileno (FILE *__stream) __attribute__ ((__nothrow__ )) ;
extern int fileno_unlocked (FILE *__stream) __attribute__ ((__nothrow__ )) ;
extern int pclose (FILE *__stream);
extern FILE *popen (const char *__command, const char *__modes)
  __attribute__ ((__malloc__)) ;
extern char *ctermid (char *__s) __attribute__ ((__nothrow__ ))
                                     ;
extern void flockfile (FILE *__stream) __attribute__ ((__nothrow__ ));
extern int ftrylockfile (FILE *__stream) __attribute__ ((__nothrow__ )) ;
extern void funlockfile (FILE *__stream) __attribute__ ((__nothrow__ ));
extern int __uflow (FILE *);
extern int __overflow (FILE *, int);
typedef struct
  {
    int quot;
    int rem;
  } div_t;
typedef struct
  {
    long int quot;
    long int rem;
  } ldiv_t;
__extension__ typedef struct
  {
    long long int quot;
    long long int rem;
  } lldiv_t;
extern size_t __ctype_get_mb_cur_max (void) __attribute__ ((__nothrow__ )) ;
extern double atof (const char *__nptr)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
extern int atoi (const char *__nptr)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
extern long int atol (const char *__nptr)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
__extension__ extern long long int atoll (const char *__nptr)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
extern double strtod (const char *__restrict __nptr,
        char **__restrict __endptr)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern float strtof (const char *__restrict __nptr,
       char **__restrict __endptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern long double strtold (const char *__restrict __nptr,
       char **__restrict __endptr)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern long int strtol (const char *__restrict __nptr,
   char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern unsigned long int strtoul (const char *__restrict __nptr,
      char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
__extension__
extern long long int strtoq (const char *__restrict __nptr,
        char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
__extension__
extern unsigned long long int strtouq (const char *__restrict __nptr,
           char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
__extension__
extern long long int strtoll (const char *__restrict __nptr,
         char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
__extension__
extern unsigned long long int strtoull (const char *__restrict __nptr,
     char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern char *l64a (long int __n) __attribute__ ((__nothrow__ )) ;
extern long int a64l (const char *__s)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;
typedef __u_char u_char;
typedef __u_short u_short;
typedef __u_int u_int;
typedef __u_long u_long;
typedef __quad_t quad_t;
typedef __u_quad_t u_quad_t;
typedef __fsid_t fsid_t;
typedef __loff_t loff_t;
typedef __ino_t ino_t;
typedef __dev_t dev_t;
typedef __gid_t gid_t;
typedef __mode_t mode_t;
typedef __nlink_t nlink_t;
typedef __uid_t uid_t;
typedef __pid_t pid_t;
typedef __id_t id_t;
typedef __daddr_t daddr_t;
typedef __caddr_t caddr_t;
typedef __key_t key_t;
typedef __clock_t clock_t;

typedef __clockid_t clockid_t;
typedef __time_t time_t;
typedef __timer_t timer_t;
typedef unsigned long int ulong;
typedef unsigned short int ushort;
typedef unsigned int uint;
typedef __uint8_t u_int8_t;
typedef __uint16_t u_int16_t;
typedef __uint32_t u_int32_t;
typedef __uint64_t u_int64_t;
typedef int register_t __attribute__ ((__mode__ (__word__)));
static __inline __uint16_t
__bswap_16 (__uint16_t __bsx)
{
  return ((__uint16_t) ((((__bsx) >> 8) & 0xff) | (((__bsx) & 0xff) << 8)));
}
static __inline __uint32_t
__bswap_32 (__uint32_t __bsx)
{
  return ((((__bsx) & 0xff000000u) >> 24) | (((__bsx) & 0x00ff0000u) >> 8) | (((__bsx) & 0x0000ff00u) << 8) | (((__bsx) & 0x000000ffu) << 24));
}
__extension__ static __inline __uint64_t
__bswap_64 (__uint64_t __bsx)
{
  return ((((__bsx) & 0xff00000000000000ull) >> 56) | (((__bsx) & 0x00ff000000000000ull) >> 40) | (((__bsx) & 0x0000ff0000000000ull) >> 24) | (((__bsx) & 0x000000ff00000000ull) >> 8) | (((__bsx) & 0x00000000ff000000ull) << 8) | (((__bsx) & 0x0000000000ff0000ull) << 24) | (((__bsx) & 0x000000000000ff00ull) << 40) | (((__bsx) & 0x00000000000000ffull) << 56));
}
static __inline __uint16_t
__uint16_identity (__uint16_t __x)
{
  return __x;
}
static __inline __uint32_t
__uint32_identity (__uint32_t __x)
{
  return __x;
}
static __inline __uint64_t
__uint64_identity (__uint64_t __x)
{
  return __x;
}
typedef struct
{
  unsigned long int __val[(1024 / (8 * sizeof (unsigned long int)))];
} __sigset_t;
typedef __sigset_t sigset_t;
struct timeval
{
  __time_t tv_sec;
  __suseconds_t tv_usec;
};

struct timespec
{
  __time_t tv_sec;
  __syscall_slong_t tv_nsec;
};
typedef __suseconds_t suseconds_t;
typedef long int __fd_mask;
typedef struct
  {
    __fd_mask __fds_bits[1024 / (8 * (int) sizeof (__fd_mask))];
  } fd_set;
typedef __fd_mask fd_mask;
extern int select (int __nfds, fd_set *__restrict __readfds,
     fd_set *__restrict __writefds,
     fd_set *__restrict __exceptfds,
     struct timeval *__restrict __timeout);
extern int pselect (int __nfds, fd_set *__restrict __readfds,
      fd_set *__restrict __writefds,
      fd_set *__restrict __exceptfds,
      const struct timespec *__restrict __timeout,
      const __sigset_t *__restrict __sigmask);
typedef __blksize_t blksize_t;
typedef __blkcnt_t blkcnt_t;
typedef __fsblkcnt_t fsblkcnt_t;
typedef __fsfilcnt_t fsfilcnt_t;

typedef union
{
  __extension__ unsigned long long int __value64;
  struct
  {
    unsigned int __low;
    unsigned int __high;
  } __value32;
} __atomic_wide_counter;
typedef struct __pthread_internal_list
{
  struct __pthread_internal_list *__prev;
  struct __pthread_internal_list *__next;
} __pthread_list_t;
typedef struct __pthread_internal_slist
{
  struct __pthread_internal_slist *__next;
} __pthread_slist_t;
struct __pthread_mutex_s
{
  int __lock;
  unsigned int __count;
  int __owner;
  unsigned int __nusers;
  int __kind;
  short __spins;
  short __elision;
  __pthread_list_t __list;
};
struct __pthread_rwlock_arch_t
{
  unsigned int __readers;
  unsigned int __writers;
  unsigned int __wrphase_futex;
  unsigned int __writers_futex;
  unsigned int __pad3;
  unsigned int __pad4;
  int __cur_writer;
  int __shared;
  signed char __rwelision;
  unsigned char __pad1[7];
  unsigned long int __pad2;
  unsigned int __flags;
};
struct __pthread_cond_s
{
  __atomic_wide_counter __wseq;
  __atomic_wide_counter __g1_start;
  unsigned int __g_refs[2] ;
  unsigned int __g_size[2];
  unsigned int __g1_orig_size;
  unsigned int __wrefs;
  unsigned int __g_signals[2];
};
typedef unsigned int __tss_t;
typedef unsigned long int __thrd_t;
typedef struct
{
  int __data ;
} __once_flag;
typedef unsigned long int pthread_t;
typedef union
{
  char __size[4];
  int __align;
} pthread_mutexattr_t;
typedef union
{
  char __size[4];
  int __align;
} pthread_condattr_t;
typedef unsigned int pthread_key_t;
typedef int pthread_once_t;
union pthread_attr_t
{
  char __size[56];
  long int __align;
};
typedef union pthread_attr_t pthread_attr_t;
typedef union
{
  struct __pthread_mutex_s __data;
  char __size[40];
  long int __align;
} pthread_mutex_t;
typedef union
{
  struct __pthread_cond_s __data;
  char __size[48];
  __extension__ long long int __align;
} pthread_cond_t;
typedef union
{
  struct __pthread_rwlock_arch_t __data;
  char __size[56];
  long int __align;
} pthread_rwlock_t;
typedef union
{
  char __size[8];
  long int __align;
} pthread_rwlockattr_t;
typedef volatile int pthread_spinlock_t;
typedef union
{
  char __size[32];
  long int __align;
} pthread_barrier_t;
typedef union
{
  char __size[4];
  int __align;
} pthread_barrierattr_t;
extern long int random (void) __attribute__ ((__nothrow__ ));
extern void srandom (unsigned int __seed) __attribute__ ((__nothrow__ ));
extern char *initstate (unsigned int __seed, char *__statebuf,
   size_t __statelen) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
extern char *setstate (char *__statebuf) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
struct random_data
  {
    int32_t *fptr;
    int32_t *rptr;
    int32_t *state;
    int rand_type;
    int rand_deg;
    int rand_sep;
    int32_t *end_ptr;
  };
extern int random_r (struct random_data *__restrict __buf,
       int32_t *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int srandom_r (unsigned int __seed, struct random_data *__buf)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
extern int initstate_r (unsigned int __seed, char *__restrict __statebuf,
   size_t __statelen,
   struct random_data *__restrict __buf)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2, 4)));
extern int setstate_r (char *__restrict __statebuf,
         struct random_data *__restrict __buf)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int rand (void) __attribute__ ((__nothrow__ ));
extern void srand (unsigned int __seed) __attribute__ ((__nothrow__ ));
extern int rand_r (unsigned int *__seed) __attribute__ ((__nothrow__ ));
extern double drand48 (void) __attribute__ ((__nothrow__ ));
extern double erand48 (unsigned short int __xsubi[3]) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern long int lrand48 (void) __attribute__ ((__nothrow__ ));
extern long int nrand48 (unsigned short int __xsubi[3])
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern long int mrand48 (void) __attribute__ ((__nothrow__ ));
extern long int jrand48 (unsigned short int __xsubi[3])
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern void srand48 (long int __seedval) __attribute__ ((__nothrow__ ));
extern unsigned short int *seed48 (unsigned short int __seed16v[3])
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern void lcong48 (unsigned short int __param[7]) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
struct drand48_data
  {
    unsigned short int __x[3];
    unsigned short int __old_x[3];
    unsigned short int __c;
    unsigned short int __init;
    __extension__ unsigned long long int __a;
  };
extern int drand48_r (struct drand48_data *__restrict __buffer,
        double *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int erand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        double *__restrict __result) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int lrand48_r (struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int nrand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int mrand48_r (struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int jrand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int srand48_r (long int __seedval, struct drand48_data *__buffer)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
extern int seed48_r (unsigned short int __seed16v[3],
       struct drand48_data *__buffer) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int lcong48_r (unsigned short int __param[7],
        struct drand48_data *__buffer)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern void *malloc (size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__))
                                         ;
extern void *calloc (size_t __nmemb, size_t __size)
     __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) ;
extern void *realloc (void *__ptr, size_t __size)
     __attribute__ ((__nothrow__ )) __attribute__ ((__warn_unused_result__)) ;
extern void free (void *__ptr) __attribute__ ((__nothrow__ ));
extern void *reallocarray (void *__ptr, size_t __nmemb, size_t __size)
     __attribute__ ((__nothrow__ )) __attribute__ ((__warn_unused_result__))
                       ;
extern void *reallocarray (void *__ptr, size_t __nmemb, size_t __size)
     __attribute__ ((__nothrow__ )) ;
extern void *alloca (size_t __size) __attribute__ ((__nothrow__ ));
extern void *valloc (size_t __size) __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__))
                                         ;
extern int posix_memalign (void **__memptr, size_t __alignment, size_t __size)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
extern void *aligned_alloc (size_t __alignment, size_t __size)
     __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) __attribute__ ((__alloc_align__ (1)))
                                         ;
extern void abort (void) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern int atexit (void (*__func) (void)) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern int at_quick_exit (void (*__func) (void)) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern int on_exit (void (*__func) (int __status, void *__arg), void *__arg)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern void exit (int __status) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern void quick_exit (int __status) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern void _Exit (int __status) __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern char *getenv (const char *__name) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
extern int putenv (char *__string) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern int setenv (const char *__name, const char *__value, int __replace)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
extern int unsetenv (const char *__name) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern int clearenv (void) __attribute__ ((__nothrow__ ));
extern char *mktemp (char *__template) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern int mkstemp (char *__template) __attribute__ ((__nonnull__ (1))) ;
extern int mkstemps (char *__template, int __suffixlen) __attribute__ ((__nonnull__ (1))) ;
extern char *mkdtemp (char *__template) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
extern int system (const char *__command) ;
extern char *realpath (const char *__restrict __name,
         char *__restrict __resolved) __attribute__ ((__nothrow__ )) ;
typedef int (*__compar_fn_t) (const void *, const void *);
extern void *bsearch (const void *__key, const void *__base,
        size_t __nmemb, size_t __size, __compar_fn_t __compar)
     __attribute__ ((__nonnull__ (1, 2, 5))) ;
extern void qsort (void *__base, size_t __nmemb, size_t __size,
     __compar_fn_t __compar) __attribute__ ((__nonnull__ (1, 4)));
extern int abs (int __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
extern long int labs (long int __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
__extension__ extern long long int llabs (long long int __x)
     __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
extern div_t div (int __numer, int __denom)
     __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
extern ldiv_t ldiv (long int __numer, long int __denom)
     __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
__extension__ extern lldiv_t lldiv (long long int __numer,
        long long int __denom)
     __attribute__ ((__nothrow__ )) __attribute__ ((__const__)) ;
extern char *ecvt (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4))) ;
extern char *fcvt (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4))) ;
extern char *gcvt (double __value, int __ndigit, char *__buf)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3))) ;
extern char *qecvt (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4))) ;
extern char *qfcvt (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4))) ;
extern char *qgcvt (long double __value, int __ndigit, char *__buf)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3))) ;
extern int ecvt_r (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign, char *__restrict __buf,
     size_t __len) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4, 5)));
extern int fcvt_r (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign, char *__restrict __buf,
     size_t __len) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4, 5)));
extern int qecvt_r (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign,
      char *__restrict __buf, size_t __len)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4, 5)));
extern int qfcvt_r (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign,
      char *__restrict __buf, size_t __len)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (3, 4, 5)));
extern int mblen (const char *__s, size_t __n) __attribute__ ((__nothrow__ ));
extern int mbtowc (wchar_t *__restrict __pwc,
     const char *__restrict __s, size_t __n) __attribute__ ((__nothrow__ ));
extern int wctomb (char *__s, wchar_t __wchar) __attribute__ ((__nothrow__ ));
extern size_t mbstowcs (wchar_t *__restrict __pwcs,
   const char *__restrict __s, size_t __n) __attribute__ ((__nothrow__ ))
                                      ;
extern size_t wcstombs (char *__restrict __s,
   const wchar_t *__restrict __pwcs, size_t __n)
     __attribute__ ((__nothrow__ ))
                                    ;
extern int rpmatch (const char *__response) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1))) ;
extern int getsubopt (char **__restrict __optionp,
        char *const *__restrict __tokens,
        char **__restrict __valuep)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2, 3))) ;
extern int getloadavg (double __loadavg[], int __nelem)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern void *memcpy (void *__restrict __dest, const void *__restrict __src,
       size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern void *memmove (void *__dest, const void *__src, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern void *memccpy (void *__restrict __dest, const void *__restrict __src,
        int __c, size_t __n)
    __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2))) ;
extern void *memset (void *__s, int __c, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern int memcmp (const void *__s1, const void *__s2, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern int __memcmpeq (const void *__s1, const void *__s2, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern void *memchr (const void *__s, int __c, size_t __n)
      __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern char *strcpy (char *__restrict __dest, const char *__restrict __src)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern char *strncpy (char *__restrict __dest,
        const char *__restrict __src, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern char *strcat (char *__restrict __dest, const char *__restrict __src)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern char *strncat (char *__restrict __dest, const char *__restrict __src,
        size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern int strcmp (const char *__s1, const char *__s2)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern int strncmp (const char *__s1, const char *__s2, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern int strcoll (const char *__s1, const char *__s2)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern size_t strxfrm (char *__restrict __dest,
         const char *__restrict __src, size_t __n)
    __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2))) ;
struct __locale_struct
{
  struct __locale_data *__locales[13];
  const unsigned short int *__ctype_b;
  const int *__ctype_tolower;
  const int *__ctype_toupper;
  const char *__names[13];
};
typedef struct __locale_struct *__locale_t;

typedef __locale_t locale_t;
extern int strcoll_l (const char *__s1, const char *__s2, locale_t __l)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 3)));
extern size_t strxfrm_l (char *__dest, const char *__src, size_t __n,
    locale_t __l) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2, 4)))
                                           ;
extern char *strdup (const char *__s)
     __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) __attribute__ ((__nonnull__ (1)));
extern char *strndup (const char *__string, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__malloc__)) __attribute__ ((__nonnull__ (1)));
extern char *strchr (const char *__s, int __c)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern char *strrchr (const char *__s, int __c)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern size_t strcspn (const char *__s, const char *__reject)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern size_t strspn (const char *__s, const char *__accept)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern char *strpbrk (const char *__s, const char *__accept)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern char *strstr (const char *__haystack, const char *__needle)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern char *strtok (char *__restrict __s, const char *__restrict __delim)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
extern char *__strtok_r (char *__restrict __s,
    const char *__restrict __delim,
    char **__restrict __save_ptr)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2, 3)));
extern char *strtok_r (char *__restrict __s, const char *__restrict __delim,
         char **__restrict __save_ptr)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2, 3)));
extern size_t strlen (const char *__s)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern size_t strnlen (const char *__string, size_t __maxlen)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern char *strerror (int __errnum) __attribute__ ((__nothrow__ ));
extern int strerror_r (int __errnum, char *__buf, size_t __buflen) __asm__ ("" "__xpg_strerror_r") __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)))
                                          ;
extern char *strerror_l (int __errnum, locale_t __l) __attribute__ ((__nothrow__ ));
extern int bcmp (const void *__s1, const void *__s2, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern void bcopy (const void *__src, void *__dest, size_t __n)
  __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern void bzero (void *__s, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)));
extern char *index (const char *__s, int __c)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern char *rindex (const char *__s, int __c)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern int ffs (int __i) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern int ffsl (long int __l) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
__extension__ extern int ffsll (long long int __ll)
     __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern int strcasecmp (const char *__s1, const char *__s2)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern int strncasecmp (const char *__s1, const char *__s2, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern int strcasecmp_l (const char *__s1, const char *__s2, locale_t __loc)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 3)));
extern int strncasecmp_l (const char *__s1, const char *__s2,
     size_t __n, locale_t __loc)
     __attribute__ ((__nothrow__ )) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 4)));
extern void explicit_bzero (void *__s, size_t __n) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1)))
                                                  ;
extern char *strsep (char **__restrict __stringp,
       const char *__restrict __delim)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern char *strsignal (int __sig) __attribute__ ((__nothrow__ ));
extern char *__stpcpy (char *__restrict __dest, const char *__restrict __src)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern char *stpcpy (char *__restrict __dest, const char *__restrict __src)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern char *__stpncpy (char *__restrict __dest,
   const char *__restrict __src, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
extern char *stpncpy (char *__restrict __dest,
        const char *__restrict __src, size_t __n)
     __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (1, 2)));
int zlib_compress_observe(int8_t packet[512]);
     typedef size_t z_size_t;
typedef unsigned char Byte;
typedef unsigned int uInt;
typedef unsigned long uLong;
   typedef Byte Bytef;
typedef char charf;
typedef int intf;
typedef uInt uIntf;
typedef uLong uLongf;
   typedef void const *voidpc;
   typedef void *voidpf;
   typedef void *voidp;
   typedef unsigned z_crc_t;
typedef voidpf (*alloc_func) (voidpf opaque, uInt items, uInt size);
typedef void (*free_func) (voidpf opaque, voidpf address);
struct internal_state;
typedef struct z_stream_s {
            Bytef *next_in;
    uInt avail_in;
    uLong total_in;
    Bytef *next_out;
    uInt avail_out;
    uLong total_out;
            char *msg;
    struct internal_state *state;
    alloc_func zalloc;
    free_func zfree;
    voidpf opaque;
    int data_type;
    uLong adler;
    uLong reserved;
} z_stream;
typedef z_stream *z_streamp;
typedef struct gz_header_s {
    int text;
    uLong time;
    int xflags;
    int os;
    Bytef *extra;
    uInt extra_len;
    uInt extra_max;
    Bytef *name;
    uInt name_max;
    Bytef *comment;
    uInt comm_max;
    int hcrc;
    int done;
} gz_header;
typedef gz_header *gz_headerp;
extern const char * zlibVersion (void);
extern int deflate (z_streamp strm, int flush);
extern int deflateEnd (z_streamp strm);
extern int inflate (z_streamp strm, int flush);
extern int inflateEnd (z_streamp strm);
extern int deflateSetDictionary (z_streamp strm, const Bytef *dictionary, uInt dictLength);
extern int deflateGetDictionary (z_streamp strm, Bytef *dictionary, uInt *dictLength);
extern int deflateCopy (z_streamp dest, z_streamp source);
extern int deflateReset (z_streamp strm);
extern int deflateParams (z_streamp strm, int level, int strategy);
extern int deflateTune (z_streamp strm, int good_length, int max_lazy, int nice_length, int max_chain);
extern uLong deflateBound (z_streamp strm, uLong sourceLen);
extern int deflatePending (z_streamp strm, unsigned *pending, int *bits);
extern int deflatePrime (z_streamp strm, int bits, int value);
extern int deflateSetHeader (z_streamp strm, gz_headerp head);
extern int inflateSetDictionary (z_streamp strm, const Bytef *dictionary, uInt dictLength);
extern int inflateGetDictionary (z_streamp strm, Bytef *dictionary, uInt *dictLength);
extern int inflateSync (z_streamp strm);
extern int inflateCopy (z_streamp dest, z_streamp source);
extern int inflateReset (z_streamp strm);
extern int inflateReset2 (z_streamp strm, int windowBits);
extern int inflatePrime (z_streamp strm, int bits, int value);
extern long inflateMark (z_streamp strm);
extern int inflateGetHeader (z_streamp strm, gz_headerp head);
typedef unsigned (*in_func) (void *, unsigned char * *);
typedef int (*out_func) (void *, unsigned char *, unsigned);
extern int inflateBack (z_streamp strm, in_func in, void *in_desc, out_func out, void *out_desc);
extern int inflateBackEnd (z_streamp strm);
extern uLong zlibCompileFlags (void);
extern int compress (Bytef *dest, uLongf *destLen, const Bytef *source, uLong sourceLen);
extern int compress2 (Bytef *dest, uLongf *destLen, const Bytef *source, uLong sourceLen, int level);
extern uLong compressBound (uLong sourceLen);
extern int uncompress (Bytef *dest, uLongf *destLen, const Bytef *source, uLong sourceLen);
extern int uncompress2 (Bytef *dest, uLongf *destLen, const Bytef *source, uLong *sourceLen);
typedef struct gzFile_s *gzFile;
extern gzFile gzdopen (int fd, const char *mode);
extern int gzbuffer (gzFile file, unsigned size);
extern int gzsetparams (gzFile file, int level, int strategy);
extern int gzread (gzFile file, voidp buf, unsigned len);
extern z_size_t gzfread (voidp buf, z_size_t size, z_size_t nitems, gzFile file);
extern int gzwrite (gzFile file, voidpc buf, unsigned len);
extern z_size_t gzfwrite (voidpc buf, z_size_t size, z_size_t nitems, gzFile file);
extern int gzprintf (gzFile file, const char *format, ...);
extern int gzputs (gzFile file, const char *s);
extern char * gzgets (gzFile file, char *buf, int len);
extern int gzputc (gzFile file, int c);
extern int gzgetc (gzFile file);
extern int gzungetc (int c, gzFile file);
extern int gzflush (gzFile file, int flush);
extern int gzrewind (gzFile file);
extern int gzeof (gzFile file);
extern int gzdirect (gzFile file);
extern int gzclose (gzFile file);
extern int gzclose_r (gzFile file);
extern int gzclose_w (gzFile file);
extern const char * gzerror (gzFile file, int *errnum);
extern void gzclearerr (gzFile file);
extern uLong adler32 (uLong adler, const Bytef *buf, uInt len);
extern uLong adler32_z (uLong adler, const Bytef *buf, z_size_t len);
extern uLong crc32 (uLong crc, const Bytef *buf, uInt len);
extern uLong crc32_z (uLong adler, const Bytef *buf, z_size_t len);
extern int deflateInit_ (z_streamp strm, int level, const char *version, int stream_size);
extern int inflateInit_ (z_streamp strm, const char *version, int stream_size);
extern int deflateInit2_ (z_streamp strm, int level, int method, int windowBits, int memLevel, int strategy, const char *version, int stream_size);
extern int inflateInit2_ (z_streamp strm, int windowBits, const char *version, int stream_size);
extern int inflateBackInit_ (z_streamp strm, int windowBits, unsigned char *window, const char *version, int stream_size);
struct gzFile_s {
    unsigned have;
    unsigned char *next;
    long pos;
};
extern int gzgetc_ (gzFile file);
   extern gzFile gzopen (const char *, const char *);
   extern long gzseek (gzFile, long, int);
   extern long gztell (gzFile);
   extern long gzoffset (gzFile);
   extern uLong adler32_combine (uLong, uLong, long);
   extern uLong crc32_combine (uLong, uLong, long);
extern const char * zError (int);
extern int inflateSyncPoint (z_streamp);
extern const z_crc_t * get_crc_table (void);
extern int inflateUndermine (z_streamp, int);
extern int inflateValidate (z_streamp, int);
extern unsigned long inflateCodesUsed (z_streamp);
extern int inflateResetKeep (z_streamp);
extern int deflateResetKeep (z_streamp);
extern int gzvprintf (gzFile file, const char *format, va_list va);
typedef unsigned char uch;
typedef uch uchf;
typedef unsigned short ush;
typedef ush ushf;
typedef unsigned long ulg;
extern char * const z_errmsg[10];
    extern uLong adler32_combine64 (uLong, uLong, long);
    extern uLong crc32_combine64 (uLong, uLong, long);
   voidpf zcalloc (voidpf opaque, unsigned items, unsigned size);
   void zcfree (voidpf opaque, voidpf ptr);
typedef struct ct_data_s {
    union {
        ush freq;
        ush code;
    } fc;
    union {
        ush dad;
        ush len;
    } dl;
} ct_data;
typedef struct static_tree_desc_s static_tree_desc;
typedef struct tree_desc_s {
    ct_data *dyn_tree;
    int max_code;
    const static_tree_desc *stat_desc;
} tree_desc;
typedef ush Pos;
typedef Pos Posf;
typedef unsigned IPos;
typedef struct internal_state {
    z_streamp strm;
    int status;
    Bytef *pending_buf;
    ulg pending_buf_size;
    Bytef *pending_out;
    ulg pending;
    int wrap;
    gz_headerp gzhead;
    ulg gzindex;
    Byte method;
    int last_flush;
    uInt w_size;
    uInt w_bits;
    uInt w_mask;
    Bytef *window;
    ulg window_size;
    Posf *prev;
    Posf *head;
    uInt ins_h;
    uInt hash_size;
    uInt hash_bits;
    uInt hash_mask;
    uInt hash_shift;
    long block_start;
    uInt match_length;
    IPos prev_match;
    int match_available;
    uInt strstart;
    uInt match_start;
    uInt lookahead;
    uInt prev_length;
    uInt max_chain_length;
    uInt max_lazy_match;
    int level;
    int strategy;
    uInt good_match;
    int nice_match;
    struct ct_data_s dyn_ltree[(2*(256 +1+29)+1)];
    struct ct_data_s dyn_dtree[2*30 +1];
    struct ct_data_s bl_tree[2*19 +1];
    struct tree_desc_s l_desc;
    struct tree_desc_s d_desc;
    struct tree_desc_s bl_desc;
    ush bl_count[15 +1];
    int heap[2*(256 +1+29)+1];
    int heap_len;
    int heap_max;
    uch depth[2*(256 +1+29)+1];
    uchf *l_buf;
    uInt lit_bufsize;
    uInt last_lit;
    ushf *d_buf;
    ulg opt_len;
    ulg static_len;
    uInt matches;
    uInt insert;
    ush bi_buf;
    int bi_valid;
    ulg high_water;
} deflate_state;
void _tr_init (deflate_state *s);
int _tr_tally (deflate_state *s, unsigned dist, unsigned lc);
void _tr_flush_block (deflate_state *s, charf *buf, ulg stored_len, int last);
void _tr_flush_bits (deflate_state *s);
void _tr_align (deflate_state *s);
void _tr_stored_block (deflate_state *s, charf *buf, ulg stored_len, int last);
  extern const uch _length_code[];
  extern const uch _dist_code[];
static uLong adler32_combine_ (uLong adler1, uLong adler2, long len2);
uLong adler32_z(adler, buf, len)
    uLong adler;
    const Bytef *buf;
    z_size_t len;
{
    unsigned long sum2;
    unsigned n;
    sum2 = (adler >> 16) & 0xffff;
    adler &= 0xffff;
    if (len == 1) {
        adler += buf[0];
        if (adler >= 65521U)
            adler -= 65521U;
        sum2 += adler;
        if (sum2 >= 65521U)
            sum2 -= 65521U;
        return adler | (sum2 << 16);
    }
    if (buf == 0)
        return 1L;
    if (len < 16) {
        while (len--) {
            adler += *buf++;
            sum2 += adler;
        }
        if (adler >= 65521U)
            adler -= 65521U;
        sum2 %= 65521U;
        return adler | (sum2 << 16);
    }
    while (len >= 5552) {
        len -= 5552;
        n = 5552 / 16;
        do {
            {adler += (buf)[0]; sum2 += adler;}; {adler += (buf)[0 +1]; sum2 += adler;};; {adler += (buf)[0 +2]; sum2 += adler;}; {adler += (buf)[0 +2 +1]; sum2 += adler;};;; {adler += (buf)[0 +4]; sum2 += adler;}; {adler += (buf)[0 +4 +1]; sum2 += adler;};; {adler += (buf)[0 +4 +2]; sum2 += adler;}; {adler += (buf)[0 +4 +2 +1]; sum2 += adler;};;;; {adler += (buf)[8]; sum2 += adler;}; {adler += (buf)[8 +1]; sum2 += adler;};; {adler += (buf)[8 +2]; sum2 += adler;}; {adler += (buf)[8 +2 +1]; sum2 += adler;};;; {adler += (buf)[8 +4]; sum2 += adler;}; {adler += (buf)[8 +4 +1]; sum2 += adler;};; {adler += (buf)[8 +4 +2]; sum2 += adler;}; {adler += (buf)[8 +4 +2 +1]; sum2 += adler;};;;;;
            buf += 16;
        } while (--n);
        adler %= 65521U;
        sum2 %= 65521U;
    }
    if (len) {
        while (len >= 16) {
            len -= 16;
            {adler += (buf)[0]; sum2 += adler;}; {adler += (buf)[0 +1]; sum2 += adler;};; {adler += (buf)[0 +2]; sum2 += adler;}; {adler += (buf)[0 +2 +1]; sum2 += adler;};;; {adler += (buf)[0 +4]; sum2 += adler;}; {adler += (buf)[0 +4 +1]; sum2 += adler;};; {adler += (buf)[0 +4 +2]; sum2 += adler;}; {adler += (buf)[0 +4 +2 +1]; sum2 += adler;};;;; {adler += (buf)[8]; sum2 += adler;}; {adler += (buf)[8 +1]; sum2 += adler;};; {adler += (buf)[8 +2]; sum2 += adler;}; {adler += (buf)[8 +2 +1]; sum2 += adler;};;; {adler += (buf)[8 +4]; sum2 += adler;}; {adler += (buf)[8 +4 +1]; sum2 += adler;};; {adler += (buf)[8 +4 +2]; sum2 += adler;}; {adler += (buf)[8 +4 +2 +1]; sum2 += adler;};;;;;
            buf += 16;
        }
        while (len--) {
            adler += *buf++;
            sum2 += adler;
        }
        adler %= 65521U;
        sum2 %= 65521U;
    }
    return adler | (sum2 << 16);
}
uLong adler32(adler, buf, len)
    uLong adler;
    const Bytef *buf;
    uInt len;
{
    return adler32_z(adler, buf, len);
}
static uLong adler32_combine_(adler1, adler2, len2)
    uLong adler1;
    uLong adler2;
    long len2;
{
    unsigned long sum1;
    unsigned long sum2;
    unsigned rem;
    if (len2 < 0)
        return 0xffffffffUL;
    len2 %= 65521U;
    rem = (unsigned)len2;
    sum1 = adler1 & 0xffff;
    sum2 = rem * sum1;
    sum2 %= 65521U;
    sum1 += (adler2 & 0xffff) + 65521U - 1;
    sum2 += ((adler1 >> 16) & 0xffff) + ((adler2 >> 16) & 0xffff) + 65521U - rem;
    if (sum1 >= 65521U) sum1 -= 65521U;
    if (sum1 >= 65521U) sum1 -= 65521U;
    if (sum2 >= ((unsigned long)65521U << 1)) sum2 -= ((unsigned long)65521U << 1);
    if (sum2 >= 65521U) sum2 -= 65521U;
    return sum1 | (sum2 << 16);
}
uLong adler32_combine(adler1, adler2, len2)
    uLong adler1;
    uLong adler2;
    long len2;
{
    return adler32_combine_(adler1, adler2, len2);
}
uLong adler32_combine64(adler1, adler2, len2)
    uLong adler1;
    uLong adler2;
    long len2;
{
    return adler32_combine_(adler1, adler2, len2);
}
   static unsigned long crc32_little (unsigned long, const unsigned char *, z_size_t);
   static unsigned long crc32_big (unsigned long, const unsigned char *, z_size_t);
static unsigned long gf2_matrix_times (unsigned long *mat, unsigned long vec);
static void gf2_matrix_square (unsigned long *square, unsigned long *mat);
static uLong crc32_combine_ (uLong crc1, uLong crc2, long len2);
static const z_crc_t crc_table[8][256] =
{
  {
    0x00000000UL, 0x77073096UL, 0xee0e612cUL, 0x990951baUL, 0x076dc419UL,
    0x706af48fUL, 0xe963a535UL, 0x9e6495a3UL, 0x0edb8832UL, 0x79dcb8a4UL,
    0xe0d5e91eUL, 0x97d2d988UL, 0x09b64c2bUL, 0x7eb17cbdUL, 0xe7b82d07UL,
    0x90bf1d91UL, 0x1db71064UL, 0x6ab020f2UL, 0xf3b97148UL, 0x84be41deUL,
    0x1adad47dUL, 0x6ddde4ebUL, 0xf4d4b551UL, 0x83d385c7UL, 0x136c9856UL,
    0x646ba8c0UL, 0xfd62f97aUL, 0x8a65c9ecUL, 0x14015c4fUL, 0x63066cd9UL,
    0xfa0f3d63UL, 0x8d080df5UL, 0x3b6e20c8UL, 0x4c69105eUL, 0xd56041e4UL,
    0xa2677172UL, 0x3c03e4d1UL, 0x4b04d447UL, 0xd20d85fdUL, 0xa50ab56bUL,
    0x35b5a8faUL, 0x42b2986cUL, 0xdbbbc9d6UL, 0xacbcf940UL, 0x32d86ce3UL,
    0x45df5c75UL, 0xdcd60dcfUL, 0xabd13d59UL, 0x26d930acUL, 0x51de003aUL,
    0xc8d75180UL, 0xbfd06116UL, 0x21b4f4b5UL, 0x56b3c423UL, 0xcfba9599UL,
    0xb8bda50fUL, 0x2802b89eUL, 0x5f058808UL, 0xc60cd9b2UL, 0xb10be924UL,
    0x2f6f7c87UL, 0x58684c11UL, 0xc1611dabUL, 0xb6662d3dUL, 0x76dc4190UL,
    0x01db7106UL, 0x98d220bcUL, 0xefd5102aUL, 0x71b18589UL, 0x06b6b51fUL,
    0x9fbfe4a5UL, 0xe8b8d433UL, 0x7807c9a2UL, 0x0f00f934UL, 0x9609a88eUL,
    0xe10e9818UL, 0x7f6a0dbbUL, 0x086d3d2dUL, 0x91646c97UL, 0xe6635c01UL,
    0x6b6b51f4UL, 0x1c6c6162UL, 0x856530d8UL, 0xf262004eUL, 0x6c0695edUL,
    0x1b01a57bUL, 0x8208f4c1UL, 0xf50fc457UL, 0x65b0d9c6UL, 0x12b7e950UL,
    0x8bbeb8eaUL, 0xfcb9887cUL, 0x62dd1ddfUL, 0x15da2d49UL, 0x8cd37cf3UL,
    0xfbd44c65UL, 0x4db26158UL, 0x3ab551ceUL, 0xa3bc0074UL, 0xd4bb30e2UL,
    0x4adfa541UL, 0x3dd895d7UL, 0xa4d1c46dUL, 0xd3d6f4fbUL, 0x4369e96aUL,
    0x346ed9fcUL, 0xad678846UL, 0xda60b8d0UL, 0x44042d73UL, 0x33031de5UL,
    0xaa0a4c5fUL, 0xdd0d7cc9UL, 0x5005713cUL, 0x270241aaUL, 0xbe0b1010UL,
    0xc90c2086UL, 0x5768b525UL, 0x206f85b3UL, 0xb966d409UL, 0xce61e49fUL,
    0x5edef90eUL, 0x29d9c998UL, 0xb0d09822UL, 0xc7d7a8b4UL, 0x59b33d17UL,
    0x2eb40d81UL, 0xb7bd5c3bUL, 0xc0ba6cadUL, 0xedb88320UL, 0x9abfb3b6UL,
    0x03b6e20cUL, 0x74b1d29aUL, 0xead54739UL, 0x9dd277afUL, 0x04db2615UL,
    0x73dc1683UL, 0xe3630b12UL, 0x94643b84UL, 0x0d6d6a3eUL, 0x7a6a5aa8UL,
    0xe40ecf0bUL, 0x9309ff9dUL, 0x0a00ae27UL, 0x7d079eb1UL, 0xf00f9344UL,
    0x8708a3d2UL, 0x1e01f268UL, 0x6906c2feUL, 0xf762575dUL, 0x806567cbUL,
    0x196c3671UL, 0x6e6b06e7UL, 0xfed41b76UL, 0x89d32be0UL, 0x10da7a5aUL,
    0x67dd4accUL, 0xf9b9df6fUL, 0x8ebeeff9UL, 0x17b7be43UL, 0x60b08ed5UL,
    0xd6d6a3e8UL, 0xa1d1937eUL, 0x38d8c2c4UL, 0x4fdff252UL, 0xd1bb67f1UL,
    0xa6bc5767UL, 0x3fb506ddUL, 0x48b2364bUL, 0xd80d2bdaUL, 0xaf0a1b4cUL,
    0x36034af6UL, 0x41047a60UL, 0xdf60efc3UL, 0xa867df55UL, 0x316e8eefUL,
    0x4669be79UL, 0xcb61b38cUL, 0xbc66831aUL, 0x256fd2a0UL, 0x5268e236UL,
    0xcc0c7795UL, 0xbb0b4703UL, 0x220216b9UL, 0x5505262fUL, 0xc5ba3bbeUL,
    0xb2bd0b28UL, 0x2bb45a92UL, 0x5cb36a04UL, 0xc2d7ffa7UL, 0xb5d0cf31UL,
    0x2cd99e8bUL, 0x5bdeae1dUL, 0x9b64c2b0UL, 0xec63f226UL, 0x756aa39cUL,
    0x026d930aUL, 0x9c0906a9UL, 0xeb0e363fUL, 0x72076785UL, 0x05005713UL,
    0x95bf4a82UL, 0xe2b87a14UL, 0x7bb12baeUL, 0x0cb61b38UL, 0x92d28e9bUL,
    0xe5d5be0dUL, 0x7cdcefb7UL, 0x0bdbdf21UL, 0x86d3d2d4UL, 0xf1d4e242UL,
    0x68ddb3f8UL, 0x1fda836eUL, 0x81be16cdUL, 0xf6b9265bUL, 0x6fb077e1UL,
    0x18b74777UL, 0x88085ae6UL, 0xff0f6a70UL, 0x66063bcaUL, 0x11010b5cUL,
    0x8f659effUL, 0xf862ae69UL, 0x616bffd3UL, 0x166ccf45UL, 0xa00ae278UL,
    0xd70dd2eeUL, 0x4e048354UL, 0x3903b3c2UL, 0xa7672661UL, 0xd06016f7UL,
    0x4969474dUL, 0x3e6e77dbUL, 0xaed16a4aUL, 0xd9d65adcUL, 0x40df0b66UL,
    0x37d83bf0UL, 0xa9bcae53UL, 0xdebb9ec5UL, 0x47b2cf7fUL, 0x30b5ffe9UL,
    0xbdbdf21cUL, 0xcabac28aUL, 0x53b39330UL, 0x24b4a3a6UL, 0xbad03605UL,
    0xcdd70693UL, 0x54de5729UL, 0x23d967bfUL, 0xb3667a2eUL, 0xc4614ab8UL,
    0x5d681b02UL, 0x2a6f2b94UL, 0xb40bbe37UL, 0xc30c8ea1UL, 0x5a05df1bUL,
    0x2d02ef8dUL
  },
  {
    0x00000000UL, 0x191b3141UL, 0x32366282UL, 0x2b2d53c3UL, 0x646cc504UL,
    0x7d77f445UL, 0x565aa786UL, 0x4f4196c7UL, 0xc8d98a08UL, 0xd1c2bb49UL,
    0xfaefe88aUL, 0xe3f4d9cbUL, 0xacb54f0cUL, 0xb5ae7e4dUL, 0x9e832d8eUL,
    0x87981ccfUL, 0x4ac21251UL, 0x53d92310UL, 0x78f470d3UL, 0x61ef4192UL,
    0x2eaed755UL, 0x37b5e614UL, 0x1c98b5d7UL, 0x05838496UL, 0x821b9859UL,
    0x9b00a918UL, 0xb02dfadbUL, 0xa936cb9aUL, 0xe6775d5dUL, 0xff6c6c1cUL,
    0xd4413fdfUL, 0xcd5a0e9eUL, 0x958424a2UL, 0x8c9f15e3UL, 0xa7b24620UL,
    0xbea97761UL, 0xf1e8e1a6UL, 0xe8f3d0e7UL, 0xc3de8324UL, 0xdac5b265UL,
    0x5d5daeaaUL, 0x44469febUL, 0x6f6bcc28UL, 0x7670fd69UL, 0x39316baeUL,
    0x202a5aefUL, 0x0b07092cUL, 0x121c386dUL, 0xdf4636f3UL, 0xc65d07b2UL,
    0xed705471UL, 0xf46b6530UL, 0xbb2af3f7UL, 0xa231c2b6UL, 0x891c9175UL,
    0x9007a034UL, 0x179fbcfbUL, 0x0e848dbaUL, 0x25a9de79UL, 0x3cb2ef38UL,
    0x73f379ffUL, 0x6ae848beUL, 0x41c51b7dUL, 0x58de2a3cUL, 0xf0794f05UL,
    0xe9627e44UL, 0xc24f2d87UL, 0xdb541cc6UL, 0x94158a01UL, 0x8d0ebb40UL,
    0xa623e883UL, 0xbf38d9c2UL, 0x38a0c50dUL, 0x21bbf44cUL, 0x0a96a78fUL,
    0x138d96ceUL, 0x5ccc0009UL, 0x45d73148UL, 0x6efa628bUL, 0x77e153caUL,
    0xbabb5d54UL, 0xa3a06c15UL, 0x888d3fd6UL, 0x91960e97UL, 0xded79850UL,
    0xc7cca911UL, 0xece1fad2UL, 0xf5facb93UL, 0x7262d75cUL, 0x6b79e61dUL,
    0x4054b5deUL, 0x594f849fUL, 0x160e1258UL, 0x0f152319UL, 0x243870daUL,
    0x3d23419bUL, 0x65fd6ba7UL, 0x7ce65ae6UL, 0x57cb0925UL, 0x4ed03864UL,
    0x0191aea3UL, 0x188a9fe2UL, 0x33a7cc21UL, 0x2abcfd60UL, 0xad24e1afUL,
    0xb43fd0eeUL, 0x9f12832dUL, 0x8609b26cUL, 0xc94824abUL, 0xd05315eaUL,
    0xfb7e4629UL, 0xe2657768UL, 0x2f3f79f6UL, 0x362448b7UL, 0x1d091b74UL,
    0x04122a35UL, 0x4b53bcf2UL, 0x52488db3UL, 0x7965de70UL, 0x607eef31UL,
    0xe7e6f3feUL, 0xfefdc2bfUL, 0xd5d0917cUL, 0xcccba03dUL, 0x838a36faUL,
    0x9a9107bbUL, 0xb1bc5478UL, 0xa8a76539UL, 0x3b83984bUL, 0x2298a90aUL,
    0x09b5fac9UL, 0x10aecb88UL, 0x5fef5d4fUL, 0x46f46c0eUL, 0x6dd93fcdUL,
    0x74c20e8cUL, 0xf35a1243UL, 0xea412302UL, 0xc16c70c1UL, 0xd8774180UL,
    0x9736d747UL, 0x8e2de606UL, 0xa500b5c5UL, 0xbc1b8484UL, 0x71418a1aUL,
    0x685abb5bUL, 0x4377e898UL, 0x5a6cd9d9UL, 0x152d4f1eUL, 0x0c367e5fUL,
    0x271b2d9cUL, 0x3e001cddUL, 0xb9980012UL, 0xa0833153UL, 0x8bae6290UL,
    0x92b553d1UL, 0xddf4c516UL, 0xc4eff457UL, 0xefc2a794UL, 0xf6d996d5UL,
    0xae07bce9UL, 0xb71c8da8UL, 0x9c31de6bUL, 0x852aef2aUL, 0xca6b79edUL,
    0xd37048acUL, 0xf85d1b6fUL, 0xe1462a2eUL, 0x66de36e1UL, 0x7fc507a0UL,
    0x54e85463UL, 0x4df36522UL, 0x02b2f3e5UL, 0x1ba9c2a4UL, 0x30849167UL,
    0x299fa026UL, 0xe4c5aeb8UL, 0xfdde9ff9UL, 0xd6f3cc3aUL, 0xcfe8fd7bUL,
    0x80a96bbcUL, 0x99b25afdUL, 0xb29f093eUL, 0xab84387fUL, 0x2c1c24b0UL,
    0x350715f1UL, 0x1e2a4632UL, 0x07317773UL, 0x4870e1b4UL, 0x516bd0f5UL,
    0x7a468336UL, 0x635db277UL, 0xcbfad74eUL, 0xd2e1e60fUL, 0xf9ccb5ccUL,
    0xe0d7848dUL, 0xaf96124aUL, 0xb68d230bUL, 0x9da070c8UL, 0x84bb4189UL,
    0x03235d46UL, 0x1a386c07UL, 0x31153fc4UL, 0x280e0e85UL, 0x674f9842UL,
    0x7e54a903UL, 0x5579fac0UL, 0x4c62cb81UL, 0x8138c51fUL, 0x9823f45eUL,
    0xb30ea79dUL, 0xaa1596dcUL, 0xe554001bUL, 0xfc4f315aUL, 0xd7626299UL,
    0xce7953d8UL, 0x49e14f17UL, 0x50fa7e56UL, 0x7bd72d95UL, 0x62cc1cd4UL,
    0x2d8d8a13UL, 0x3496bb52UL, 0x1fbbe891UL, 0x06a0d9d0UL, 0x5e7ef3ecUL,
    0x4765c2adUL, 0x6c48916eUL, 0x7553a02fUL, 0x3a1236e8UL, 0x230907a9UL,
    0x0824546aUL, 0x113f652bUL, 0x96a779e4UL, 0x8fbc48a5UL, 0xa4911b66UL,
    0xbd8a2a27UL, 0xf2cbbce0UL, 0xebd08da1UL, 0xc0fdde62UL, 0xd9e6ef23UL,
    0x14bce1bdUL, 0x0da7d0fcUL, 0x268a833fUL, 0x3f91b27eUL, 0x70d024b9UL,
    0x69cb15f8UL, 0x42e6463bUL, 0x5bfd777aUL, 0xdc656bb5UL, 0xc57e5af4UL,
    0xee530937UL, 0xf7483876UL, 0xb809aeb1UL, 0xa1129ff0UL, 0x8a3fcc33UL,
    0x9324fd72UL
  },
  {
    0x00000000UL, 0x01c26a37UL, 0x0384d46eUL, 0x0246be59UL, 0x0709a8dcUL,
    0x06cbc2ebUL, 0x048d7cb2UL, 0x054f1685UL, 0x0e1351b8UL, 0x0fd13b8fUL,
    0x0d9785d6UL, 0x0c55efe1UL, 0x091af964UL, 0x08d89353UL, 0x0a9e2d0aUL,
    0x0b5c473dUL, 0x1c26a370UL, 0x1de4c947UL, 0x1fa2771eUL, 0x1e601d29UL,
    0x1b2f0bacUL, 0x1aed619bUL, 0x18abdfc2UL, 0x1969b5f5UL, 0x1235f2c8UL,
    0x13f798ffUL, 0x11b126a6UL, 0x10734c91UL, 0x153c5a14UL, 0x14fe3023UL,
    0x16b88e7aUL, 0x177ae44dUL, 0x384d46e0UL, 0x398f2cd7UL, 0x3bc9928eUL,
    0x3a0bf8b9UL, 0x3f44ee3cUL, 0x3e86840bUL, 0x3cc03a52UL, 0x3d025065UL,
    0x365e1758UL, 0x379c7d6fUL, 0x35dac336UL, 0x3418a901UL, 0x3157bf84UL,
    0x3095d5b3UL, 0x32d36beaUL, 0x331101ddUL, 0x246be590UL, 0x25a98fa7UL,
    0x27ef31feUL, 0x262d5bc9UL, 0x23624d4cUL, 0x22a0277bUL, 0x20e69922UL,
    0x2124f315UL, 0x2a78b428UL, 0x2bbade1fUL, 0x29fc6046UL, 0x283e0a71UL,
    0x2d711cf4UL, 0x2cb376c3UL, 0x2ef5c89aUL, 0x2f37a2adUL, 0x709a8dc0UL,
    0x7158e7f7UL, 0x731e59aeUL, 0x72dc3399UL, 0x7793251cUL, 0x76514f2bUL,
    0x7417f172UL, 0x75d59b45UL, 0x7e89dc78UL, 0x7f4bb64fUL, 0x7d0d0816UL,
    0x7ccf6221UL, 0x798074a4UL, 0x78421e93UL, 0x7a04a0caUL, 0x7bc6cafdUL,
    0x6cbc2eb0UL, 0x6d7e4487UL, 0x6f38fadeUL, 0x6efa90e9UL, 0x6bb5866cUL,
    0x6a77ec5bUL, 0x68315202UL, 0x69f33835UL, 0x62af7f08UL, 0x636d153fUL,
    0x612bab66UL, 0x60e9c151UL, 0x65a6d7d4UL, 0x6464bde3UL, 0x662203baUL,
    0x67e0698dUL, 0x48d7cb20UL, 0x4915a117UL, 0x4b531f4eUL, 0x4a917579UL,
    0x4fde63fcUL, 0x4e1c09cbUL, 0x4c5ab792UL, 0x4d98dda5UL, 0x46c49a98UL,
    0x4706f0afUL, 0x45404ef6UL, 0x448224c1UL, 0x41cd3244UL, 0x400f5873UL,
    0x4249e62aUL, 0x438b8c1dUL, 0x54f16850UL, 0x55330267UL, 0x5775bc3eUL,
    0x56b7d609UL, 0x53f8c08cUL, 0x523aaabbUL, 0x507c14e2UL, 0x51be7ed5UL,
    0x5ae239e8UL, 0x5b2053dfUL, 0x5966ed86UL, 0x58a487b1UL, 0x5deb9134UL,
    0x5c29fb03UL, 0x5e6f455aUL, 0x5fad2f6dUL, 0xe1351b80UL, 0xe0f771b7UL,
    0xe2b1cfeeUL, 0xe373a5d9UL, 0xe63cb35cUL, 0xe7fed96bUL, 0xe5b86732UL,
    0xe47a0d05UL, 0xef264a38UL, 0xeee4200fUL, 0xeca29e56UL, 0xed60f461UL,
    0xe82fe2e4UL, 0xe9ed88d3UL, 0xebab368aUL, 0xea695cbdUL, 0xfd13b8f0UL,
    0xfcd1d2c7UL, 0xfe976c9eUL, 0xff5506a9UL, 0xfa1a102cUL, 0xfbd87a1bUL,
    0xf99ec442UL, 0xf85cae75UL, 0xf300e948UL, 0xf2c2837fUL, 0xf0843d26UL,
    0xf1465711UL, 0xf4094194UL, 0xf5cb2ba3UL, 0xf78d95faUL, 0xf64fffcdUL,
    0xd9785d60UL, 0xd8ba3757UL, 0xdafc890eUL, 0xdb3ee339UL, 0xde71f5bcUL,
    0xdfb39f8bUL, 0xddf521d2UL, 0xdc374be5UL, 0xd76b0cd8UL, 0xd6a966efUL,
    0xd4efd8b6UL, 0xd52db281UL, 0xd062a404UL, 0xd1a0ce33UL, 0xd3e6706aUL,
    0xd2241a5dUL, 0xc55efe10UL, 0xc49c9427UL, 0xc6da2a7eUL, 0xc7184049UL,
    0xc25756ccUL, 0xc3953cfbUL, 0xc1d382a2UL, 0xc011e895UL, 0xcb4dafa8UL,
    0xca8fc59fUL, 0xc8c97bc6UL, 0xc90b11f1UL, 0xcc440774UL, 0xcd866d43UL,
    0xcfc0d31aUL, 0xce02b92dUL, 0x91af9640UL, 0x906dfc77UL, 0x922b422eUL,
    0x93e92819UL, 0x96a63e9cUL, 0x976454abUL, 0x9522eaf2UL, 0x94e080c5UL,
    0x9fbcc7f8UL, 0x9e7eadcfUL, 0x9c381396UL, 0x9dfa79a1UL, 0x98b56f24UL,
    0x99770513UL, 0x9b31bb4aUL, 0x9af3d17dUL, 0x8d893530UL, 0x8c4b5f07UL,
    0x8e0de15eUL, 0x8fcf8b69UL, 0x8a809decUL, 0x8b42f7dbUL, 0x89044982UL,
    0x88c623b5UL, 0x839a6488UL, 0x82580ebfUL, 0x801eb0e6UL, 0x81dcdad1UL,
    0x8493cc54UL, 0x8551a663UL, 0x8717183aUL, 0x86d5720dUL, 0xa9e2d0a0UL,
    0xa820ba97UL, 0xaa6604ceUL, 0xaba46ef9UL, 0xaeeb787cUL, 0xaf29124bUL,
    0xad6fac12UL, 0xacadc625UL, 0xa7f18118UL, 0xa633eb2fUL, 0xa4755576UL,
    0xa5b73f41UL, 0xa0f829c4UL, 0xa13a43f3UL, 0xa37cfdaaUL, 0xa2be979dUL,
    0xb5c473d0UL, 0xb40619e7UL, 0xb640a7beUL, 0xb782cd89UL, 0xb2cddb0cUL,
    0xb30fb13bUL, 0xb1490f62UL, 0xb08b6555UL, 0xbbd72268UL, 0xba15485fUL,
    0xb853f606UL, 0xb9919c31UL, 0xbcde8ab4UL, 0xbd1ce083UL, 0xbf5a5edaUL,
    0xbe9834edUL
  },
  {
    0x00000000UL, 0xb8bc6765UL, 0xaa09c88bUL, 0x12b5afeeUL, 0x8f629757UL,
    0x37def032UL, 0x256b5fdcUL, 0x9dd738b9UL, 0xc5b428efUL, 0x7d084f8aUL,
    0x6fbde064UL, 0xd7018701UL, 0x4ad6bfb8UL, 0xf26ad8ddUL, 0xe0df7733UL,
    0x58631056UL, 0x5019579fUL, 0xe8a530faUL, 0xfa109f14UL, 0x42acf871UL,
    0xdf7bc0c8UL, 0x67c7a7adUL, 0x75720843UL, 0xcdce6f26UL, 0x95ad7f70UL,
    0x2d111815UL, 0x3fa4b7fbUL, 0x8718d09eUL, 0x1acfe827UL, 0xa2738f42UL,
    0xb0c620acUL, 0x087a47c9UL, 0xa032af3eUL, 0x188ec85bUL, 0x0a3b67b5UL,
    0xb28700d0UL, 0x2f503869UL, 0x97ec5f0cUL, 0x8559f0e2UL, 0x3de59787UL,
    0x658687d1UL, 0xdd3ae0b4UL, 0xcf8f4f5aUL, 0x7733283fUL, 0xeae41086UL,
    0x525877e3UL, 0x40edd80dUL, 0xf851bf68UL, 0xf02bf8a1UL, 0x48979fc4UL,
    0x5a22302aUL, 0xe29e574fUL, 0x7f496ff6UL, 0xc7f50893UL, 0xd540a77dUL,
    0x6dfcc018UL, 0x359fd04eUL, 0x8d23b72bUL, 0x9f9618c5UL, 0x272a7fa0UL,
    0xbafd4719UL, 0x0241207cUL, 0x10f48f92UL, 0xa848e8f7UL, 0x9b14583dUL,
    0x23a83f58UL, 0x311d90b6UL, 0x89a1f7d3UL, 0x1476cf6aUL, 0xaccaa80fUL,
    0xbe7f07e1UL, 0x06c36084UL, 0x5ea070d2UL, 0xe61c17b7UL, 0xf4a9b859UL,
    0x4c15df3cUL, 0xd1c2e785UL, 0x697e80e0UL, 0x7bcb2f0eUL, 0xc377486bUL,
    0xcb0d0fa2UL, 0x73b168c7UL, 0x6104c729UL, 0xd9b8a04cUL, 0x446f98f5UL,
    0xfcd3ff90UL, 0xee66507eUL, 0x56da371bUL, 0x0eb9274dUL, 0xb6054028UL,
    0xa4b0efc6UL, 0x1c0c88a3UL, 0x81dbb01aUL, 0x3967d77fUL, 0x2bd27891UL,
    0x936e1ff4UL, 0x3b26f703UL, 0x839a9066UL, 0x912f3f88UL, 0x299358edUL,
    0xb4446054UL, 0x0cf80731UL, 0x1e4da8dfUL, 0xa6f1cfbaUL, 0xfe92dfecUL,
    0x462eb889UL, 0x549b1767UL, 0xec277002UL, 0x71f048bbUL, 0xc94c2fdeUL,
    0xdbf98030UL, 0x6345e755UL, 0x6b3fa09cUL, 0xd383c7f9UL, 0xc1366817UL,
    0x798a0f72UL, 0xe45d37cbUL, 0x5ce150aeUL, 0x4e54ff40UL, 0xf6e89825UL,
    0xae8b8873UL, 0x1637ef16UL, 0x048240f8UL, 0xbc3e279dUL, 0x21e91f24UL,
    0x99557841UL, 0x8be0d7afUL, 0x335cb0caUL, 0xed59b63bUL, 0x55e5d15eUL,
    0x47507eb0UL, 0xffec19d5UL, 0x623b216cUL, 0xda874609UL, 0xc832e9e7UL,
    0x708e8e82UL, 0x28ed9ed4UL, 0x9051f9b1UL, 0x82e4565fUL, 0x3a58313aUL,
    0xa78f0983UL, 0x1f336ee6UL, 0x0d86c108UL, 0xb53aa66dUL, 0xbd40e1a4UL,
    0x05fc86c1UL, 0x1749292fUL, 0xaff54e4aUL, 0x322276f3UL, 0x8a9e1196UL,
    0x982bbe78UL, 0x2097d91dUL, 0x78f4c94bUL, 0xc048ae2eUL, 0xd2fd01c0UL,
    0x6a4166a5UL, 0xf7965e1cUL, 0x4f2a3979UL, 0x5d9f9697UL, 0xe523f1f2UL,
    0x4d6b1905UL, 0xf5d77e60UL, 0xe762d18eUL, 0x5fdeb6ebUL, 0xc2098e52UL,
    0x7ab5e937UL, 0x680046d9UL, 0xd0bc21bcUL, 0x88df31eaUL, 0x3063568fUL,
    0x22d6f961UL, 0x9a6a9e04UL, 0x07bda6bdUL, 0xbf01c1d8UL, 0xadb46e36UL,
    0x15080953UL, 0x1d724e9aUL, 0xa5ce29ffUL, 0xb77b8611UL, 0x0fc7e174UL,
    0x9210d9cdUL, 0x2aacbea8UL, 0x38191146UL, 0x80a57623UL, 0xd8c66675UL,
    0x607a0110UL, 0x72cfaefeUL, 0xca73c99bUL, 0x57a4f122UL, 0xef189647UL,
    0xfdad39a9UL, 0x45115eccUL, 0x764dee06UL, 0xcef18963UL, 0xdc44268dUL,
    0x64f841e8UL, 0xf92f7951UL, 0x41931e34UL, 0x5326b1daUL, 0xeb9ad6bfUL,
    0xb3f9c6e9UL, 0x0b45a18cUL, 0x19f00e62UL, 0xa14c6907UL, 0x3c9b51beUL,
    0x842736dbUL, 0x96929935UL, 0x2e2efe50UL, 0x2654b999UL, 0x9ee8defcUL,
    0x8c5d7112UL, 0x34e11677UL, 0xa9362eceUL, 0x118a49abUL, 0x033fe645UL,
    0xbb838120UL, 0xe3e09176UL, 0x5b5cf613UL, 0x49e959fdUL, 0xf1553e98UL,
    0x6c820621UL, 0xd43e6144UL, 0xc68bceaaUL, 0x7e37a9cfUL, 0xd67f4138UL,
    0x6ec3265dUL, 0x7c7689b3UL, 0xc4caeed6UL, 0x591dd66fUL, 0xe1a1b10aUL,
    0xf3141ee4UL, 0x4ba87981UL, 0x13cb69d7UL, 0xab770eb2UL, 0xb9c2a15cUL,
    0x017ec639UL, 0x9ca9fe80UL, 0x241599e5UL, 0x36a0360bUL, 0x8e1c516eUL,
    0x866616a7UL, 0x3eda71c2UL, 0x2c6fde2cUL, 0x94d3b949UL, 0x090481f0UL,
    0xb1b8e695UL, 0xa30d497bUL, 0x1bb12e1eUL, 0x43d23e48UL, 0xfb6e592dUL,
    0xe9dbf6c3UL, 0x516791a6UL, 0xccb0a91fUL, 0x740cce7aUL, 0x66b96194UL,
    0xde0506f1UL
  },
  {
    0x00000000UL, 0x96300777UL, 0x2c610eeeUL, 0xba510999UL, 0x19c46d07UL,
    0x8ff46a70UL, 0x35a563e9UL, 0xa395649eUL, 0x3288db0eUL, 0xa4b8dc79UL,
    0x1ee9d5e0UL, 0x88d9d297UL, 0x2b4cb609UL, 0xbd7cb17eUL, 0x072db8e7UL,
    0x911dbf90UL, 0x6410b71dUL, 0xf220b06aUL, 0x4871b9f3UL, 0xde41be84UL,
    0x7dd4da1aUL, 0xebe4dd6dUL, 0x51b5d4f4UL, 0xc785d383UL, 0x56986c13UL,
    0xc0a86b64UL, 0x7af962fdUL, 0xecc9658aUL, 0x4f5c0114UL, 0xd96c0663UL,
    0x633d0ffaUL, 0xf50d088dUL, 0xc8206e3bUL, 0x5e10694cUL, 0xe44160d5UL,
    0x727167a2UL, 0xd1e4033cUL, 0x47d4044bUL, 0xfd850dd2UL, 0x6bb50aa5UL,
    0xfaa8b535UL, 0x6c98b242UL, 0xd6c9bbdbUL, 0x40f9bcacUL, 0xe36cd832UL,
    0x755cdf45UL, 0xcf0dd6dcUL, 0x593dd1abUL, 0xac30d926UL, 0x3a00de51UL,
    0x8051d7c8UL, 0x1661d0bfUL, 0xb5f4b421UL, 0x23c4b356UL, 0x9995bacfUL,
    0x0fa5bdb8UL, 0x9eb80228UL, 0x0888055fUL, 0xb2d90cc6UL, 0x24e90bb1UL,
    0x877c6f2fUL, 0x114c6858UL, 0xab1d61c1UL, 0x3d2d66b6UL, 0x9041dc76UL,
    0x0671db01UL, 0xbc20d298UL, 0x2a10d5efUL, 0x8985b171UL, 0x1fb5b606UL,
    0xa5e4bf9fUL, 0x33d4b8e8UL, 0xa2c90778UL, 0x34f9000fUL, 0x8ea80996UL,
    0x18980ee1UL, 0xbb0d6a7fUL, 0x2d3d6d08UL, 0x976c6491UL, 0x015c63e6UL,
    0xf4516b6bUL, 0x62616c1cUL, 0xd8306585UL, 0x4e0062f2UL, 0xed95066cUL,
    0x7ba5011bUL, 0xc1f40882UL, 0x57c40ff5UL, 0xc6d9b065UL, 0x50e9b712UL,
    0xeab8be8bUL, 0x7c88b9fcUL, 0xdf1ddd62UL, 0x492dda15UL, 0xf37cd38cUL,
    0x654cd4fbUL, 0x5861b24dUL, 0xce51b53aUL, 0x7400bca3UL, 0xe230bbd4UL,
    0x41a5df4aUL, 0xd795d83dUL, 0x6dc4d1a4UL, 0xfbf4d6d3UL, 0x6ae96943UL,
    0xfcd96e34UL, 0x468867adUL, 0xd0b860daUL, 0x732d0444UL, 0xe51d0333UL,
    0x5f4c0aaaUL, 0xc97c0dddUL, 0x3c710550UL, 0xaa410227UL, 0x10100bbeUL,
    0x86200cc9UL, 0x25b56857UL, 0xb3856f20UL, 0x09d466b9UL, 0x9fe461ceUL,
    0x0ef9de5eUL, 0x98c9d929UL, 0x2298d0b0UL, 0xb4a8d7c7UL, 0x173db359UL,
    0x810db42eUL, 0x3b5cbdb7UL, 0xad6cbac0UL, 0x2083b8edUL, 0xb6b3bf9aUL,
    0x0ce2b603UL, 0x9ad2b174UL, 0x3947d5eaUL, 0xaf77d29dUL, 0x1526db04UL,
    0x8316dc73UL, 0x120b63e3UL, 0x843b6494UL, 0x3e6a6d0dUL, 0xa85a6a7aUL,
    0x0bcf0ee4UL, 0x9dff0993UL, 0x27ae000aUL, 0xb19e077dUL, 0x44930ff0UL,
    0xd2a30887UL, 0x68f2011eUL, 0xfec20669UL, 0x5d5762f7UL, 0xcb676580UL,
    0x71366c19UL, 0xe7066b6eUL, 0x761bd4feUL, 0xe02bd389UL, 0x5a7ada10UL,
    0xcc4add67UL, 0x6fdfb9f9UL, 0xf9efbe8eUL, 0x43beb717UL, 0xd58eb060UL,
    0xe8a3d6d6UL, 0x7e93d1a1UL, 0xc4c2d838UL, 0x52f2df4fUL, 0xf167bbd1UL,
    0x6757bca6UL, 0xdd06b53fUL, 0x4b36b248UL, 0xda2b0dd8UL, 0x4c1b0aafUL,
    0xf64a0336UL, 0x607a0441UL, 0xc3ef60dfUL, 0x55df67a8UL, 0xef8e6e31UL,
    0x79be6946UL, 0x8cb361cbUL, 0x1a8366bcUL, 0xa0d26f25UL, 0x36e26852UL,
    0x95770cccUL, 0x03470bbbUL, 0xb9160222UL, 0x2f260555UL, 0xbe3bbac5UL,
    0x280bbdb2UL, 0x925ab42bUL, 0x046ab35cUL, 0xa7ffd7c2UL, 0x31cfd0b5UL,
    0x8b9ed92cUL, 0x1daede5bUL, 0xb0c2649bUL, 0x26f263ecUL, 0x9ca36a75UL,
    0x0a936d02UL, 0xa906099cUL, 0x3f360eebUL, 0x85670772UL, 0x13570005UL,
    0x824abf95UL, 0x147ab8e2UL, 0xae2bb17bUL, 0x381bb60cUL, 0x9b8ed292UL,
    0x0dbed5e5UL, 0xb7efdc7cUL, 0x21dfdb0bUL, 0xd4d2d386UL, 0x42e2d4f1UL,
    0xf8b3dd68UL, 0x6e83da1fUL, 0xcd16be81UL, 0x5b26b9f6UL, 0xe177b06fUL,
    0x7747b718UL, 0xe65a0888UL, 0x706a0fffUL, 0xca3b0666UL, 0x5c0b0111UL,
    0xff9e658fUL, 0x69ae62f8UL, 0xd3ff6b61UL, 0x45cf6c16UL, 0x78e20aa0UL,
    0xeed20dd7UL, 0x5483044eUL, 0xc2b30339UL, 0x612667a7UL, 0xf71660d0UL,
    0x4d476949UL, 0xdb776e3eUL, 0x4a6ad1aeUL, 0xdc5ad6d9UL, 0x660bdf40UL,
    0xf03bd837UL, 0x53aebca9UL, 0xc59ebbdeUL, 0x7fcfb247UL, 0xe9ffb530UL,
    0x1cf2bdbdUL, 0x8ac2bacaUL, 0x3093b353UL, 0xa6a3b424UL, 0x0536d0baUL,
    0x9306d7cdUL, 0x2957de54UL, 0xbf67d923UL, 0x2e7a66b3UL, 0xb84a61c4UL,
    0x021b685dUL, 0x942b6f2aUL, 0x37be0bb4UL, 0xa18e0cc3UL, 0x1bdf055aUL,
    0x8def022dUL
  },
  {
    0x00000000UL, 0x41311b19UL, 0x82623632UL, 0xc3532d2bUL, 0x04c56c64UL,
    0x45f4777dUL, 0x86a75a56UL, 0xc796414fUL, 0x088ad9c8UL, 0x49bbc2d1UL,
    0x8ae8effaUL, 0xcbd9f4e3UL, 0x0c4fb5acUL, 0x4d7eaeb5UL, 0x8e2d839eUL,
    0xcf1c9887UL, 0x5112c24aUL, 0x1023d953UL, 0xd370f478UL, 0x9241ef61UL,
    0x55d7ae2eUL, 0x14e6b537UL, 0xd7b5981cUL, 0x96848305UL, 0x59981b82UL,
    0x18a9009bUL, 0xdbfa2db0UL, 0x9acb36a9UL, 0x5d5d77e6UL, 0x1c6c6cffUL,
    0xdf3f41d4UL, 0x9e0e5acdUL, 0xa2248495UL, 0xe3159f8cUL, 0x2046b2a7UL,
    0x6177a9beUL, 0xa6e1e8f1UL, 0xe7d0f3e8UL, 0x2483dec3UL, 0x65b2c5daUL,
    0xaaae5d5dUL, 0xeb9f4644UL, 0x28cc6b6fUL, 0x69fd7076UL, 0xae6b3139UL,
    0xef5a2a20UL, 0x2c09070bUL, 0x6d381c12UL, 0xf33646dfUL, 0xb2075dc6UL,
    0x715470edUL, 0x30656bf4UL, 0xf7f32abbUL, 0xb6c231a2UL, 0x75911c89UL,
    0x34a00790UL, 0xfbbc9f17UL, 0xba8d840eUL, 0x79dea925UL, 0x38efb23cUL,
    0xff79f373UL, 0xbe48e86aUL, 0x7d1bc541UL, 0x3c2ade58UL, 0x054f79f0UL,
    0x447e62e9UL, 0x872d4fc2UL, 0xc61c54dbUL, 0x018a1594UL, 0x40bb0e8dUL,
    0x83e823a6UL, 0xc2d938bfUL, 0x0dc5a038UL, 0x4cf4bb21UL, 0x8fa7960aUL,
    0xce968d13UL, 0x0900cc5cUL, 0x4831d745UL, 0x8b62fa6eUL, 0xca53e177UL,
    0x545dbbbaUL, 0x156ca0a3UL, 0xd63f8d88UL, 0x970e9691UL, 0x5098d7deUL,
    0x11a9ccc7UL, 0xd2fae1ecUL, 0x93cbfaf5UL, 0x5cd76272UL, 0x1de6796bUL,
    0xdeb55440UL, 0x9f844f59UL, 0x58120e16UL, 0x1923150fUL, 0xda703824UL,
    0x9b41233dUL, 0xa76bfd65UL, 0xe65ae67cUL, 0x2509cb57UL, 0x6438d04eUL,
    0xa3ae9101UL, 0xe29f8a18UL, 0x21cca733UL, 0x60fdbc2aUL, 0xafe124adUL,
    0xeed03fb4UL, 0x2d83129fUL, 0x6cb20986UL, 0xab2448c9UL, 0xea1553d0UL,
    0x29467efbUL, 0x687765e2UL, 0xf6793f2fUL, 0xb7482436UL, 0x741b091dUL,
    0x352a1204UL, 0xf2bc534bUL, 0xb38d4852UL, 0x70de6579UL, 0x31ef7e60UL,
    0xfef3e6e7UL, 0xbfc2fdfeUL, 0x7c91d0d5UL, 0x3da0cbccUL, 0xfa368a83UL,
    0xbb07919aUL, 0x7854bcb1UL, 0x3965a7a8UL, 0x4b98833bUL, 0x0aa99822UL,
    0xc9fab509UL, 0x88cbae10UL, 0x4f5def5fUL, 0x0e6cf446UL, 0xcd3fd96dUL,
    0x8c0ec274UL, 0x43125af3UL, 0x022341eaUL, 0xc1706cc1UL, 0x804177d8UL,
    0x47d73697UL, 0x06e62d8eUL, 0xc5b500a5UL, 0x84841bbcUL, 0x1a8a4171UL,
    0x5bbb5a68UL, 0x98e87743UL, 0xd9d96c5aUL, 0x1e4f2d15UL, 0x5f7e360cUL,
    0x9c2d1b27UL, 0xdd1c003eUL, 0x120098b9UL, 0x533183a0UL, 0x9062ae8bUL,
    0xd153b592UL, 0x16c5f4ddUL, 0x57f4efc4UL, 0x94a7c2efUL, 0xd596d9f6UL,
    0xe9bc07aeUL, 0xa88d1cb7UL, 0x6bde319cUL, 0x2aef2a85UL, 0xed796bcaUL,
    0xac4870d3UL, 0x6f1b5df8UL, 0x2e2a46e1UL, 0xe136de66UL, 0xa007c57fUL,
    0x6354e854UL, 0x2265f34dUL, 0xe5f3b202UL, 0xa4c2a91bUL, 0x67918430UL,
    0x26a09f29UL, 0xb8aec5e4UL, 0xf99fdefdUL, 0x3accf3d6UL, 0x7bfde8cfUL,
    0xbc6ba980UL, 0xfd5ab299UL, 0x3e099fb2UL, 0x7f3884abUL, 0xb0241c2cUL,
    0xf1150735UL, 0x32462a1eUL, 0x73773107UL, 0xb4e17048UL, 0xf5d06b51UL,
    0x3683467aUL, 0x77b25d63UL, 0x4ed7facbUL, 0x0fe6e1d2UL, 0xccb5ccf9UL,
    0x8d84d7e0UL, 0x4a1296afUL, 0x0b238db6UL, 0xc870a09dUL, 0x8941bb84UL,
    0x465d2303UL, 0x076c381aUL, 0xc43f1531UL, 0x850e0e28UL, 0x42984f67UL,
    0x03a9547eUL, 0xc0fa7955UL, 0x81cb624cUL, 0x1fc53881UL, 0x5ef42398UL,
    0x9da70eb3UL, 0xdc9615aaUL, 0x1b0054e5UL, 0x5a314ffcUL, 0x996262d7UL,
    0xd85379ceUL, 0x174fe149UL, 0x567efa50UL, 0x952dd77bUL, 0xd41ccc62UL,
    0x138a8d2dUL, 0x52bb9634UL, 0x91e8bb1fUL, 0xd0d9a006UL, 0xecf37e5eUL,
    0xadc26547UL, 0x6e91486cUL, 0x2fa05375UL, 0xe836123aUL, 0xa9070923UL,
    0x6a542408UL, 0x2b653f11UL, 0xe479a796UL, 0xa548bc8fUL, 0x661b91a4UL,
    0x272a8abdUL, 0xe0bccbf2UL, 0xa18dd0ebUL, 0x62defdc0UL, 0x23efe6d9UL,
    0xbde1bc14UL, 0xfcd0a70dUL, 0x3f838a26UL, 0x7eb2913fUL, 0xb924d070UL,
    0xf815cb69UL, 0x3b46e642UL, 0x7a77fd5bUL, 0xb56b65dcUL, 0xf45a7ec5UL,
    0x370953eeUL, 0x763848f7UL, 0xb1ae09b8UL, 0xf09f12a1UL, 0x33cc3f8aUL,
    0x72fd2493UL
  },
  {
    0x00000000UL, 0x376ac201UL, 0x6ed48403UL, 0x59be4602UL, 0xdca80907UL,
    0xebc2cb06UL, 0xb27c8d04UL, 0x85164f05UL, 0xb851130eUL, 0x8f3bd10fUL,
    0xd685970dUL, 0xe1ef550cUL, 0x64f91a09UL, 0x5393d808UL, 0x0a2d9e0aUL,
    0x3d475c0bUL, 0x70a3261cUL, 0x47c9e41dUL, 0x1e77a21fUL, 0x291d601eUL,
    0xac0b2f1bUL, 0x9b61ed1aUL, 0xc2dfab18UL, 0xf5b56919UL, 0xc8f23512UL,
    0xff98f713UL, 0xa626b111UL, 0x914c7310UL, 0x145a3c15UL, 0x2330fe14UL,
    0x7a8eb816UL, 0x4de47a17UL, 0xe0464d38UL, 0xd72c8f39UL, 0x8e92c93bUL,
    0xb9f80b3aUL, 0x3cee443fUL, 0x0b84863eUL, 0x523ac03cUL, 0x6550023dUL,
    0x58175e36UL, 0x6f7d9c37UL, 0x36c3da35UL, 0x01a91834UL, 0x84bf5731UL,
    0xb3d59530UL, 0xea6bd332UL, 0xdd011133UL, 0x90e56b24UL, 0xa78fa925UL,
    0xfe31ef27UL, 0xc95b2d26UL, 0x4c4d6223UL, 0x7b27a022UL, 0x2299e620UL,
    0x15f32421UL, 0x28b4782aUL, 0x1fdeba2bUL, 0x4660fc29UL, 0x710a3e28UL,
    0xf41c712dUL, 0xc376b32cUL, 0x9ac8f52eUL, 0xada2372fUL, 0xc08d9a70UL,
    0xf7e75871UL, 0xae591e73UL, 0x9933dc72UL, 0x1c259377UL, 0x2b4f5176UL,
    0x72f11774UL, 0x459bd575UL, 0x78dc897eUL, 0x4fb64b7fUL, 0x16080d7dUL,
    0x2162cf7cUL, 0xa4748079UL, 0x931e4278UL, 0xcaa0047aUL, 0xfdcac67bUL,
    0xb02ebc6cUL, 0x87447e6dUL, 0xdefa386fUL, 0xe990fa6eUL, 0x6c86b56bUL,
    0x5bec776aUL, 0x02523168UL, 0x3538f369UL, 0x087faf62UL, 0x3f156d63UL,
    0x66ab2b61UL, 0x51c1e960UL, 0xd4d7a665UL, 0xe3bd6464UL, 0xba032266UL,
    0x8d69e067UL, 0x20cbd748UL, 0x17a11549UL, 0x4e1f534bUL, 0x7975914aUL,
    0xfc63de4fUL, 0xcb091c4eUL, 0x92b75a4cUL, 0xa5dd984dUL, 0x989ac446UL,
    0xaff00647UL, 0xf64e4045UL, 0xc1248244UL, 0x4432cd41UL, 0x73580f40UL,
    0x2ae64942UL, 0x1d8c8b43UL, 0x5068f154UL, 0x67023355UL, 0x3ebc7557UL,
    0x09d6b756UL, 0x8cc0f853UL, 0xbbaa3a52UL, 0xe2147c50UL, 0xd57ebe51UL,
    0xe839e25aUL, 0xdf53205bUL, 0x86ed6659UL, 0xb187a458UL, 0x3491eb5dUL,
    0x03fb295cUL, 0x5a456f5eUL, 0x6d2fad5fUL, 0x801b35e1UL, 0xb771f7e0UL,
    0xeecfb1e2UL, 0xd9a573e3UL, 0x5cb33ce6UL, 0x6bd9fee7UL, 0x3267b8e5UL,
    0x050d7ae4UL, 0x384a26efUL, 0x0f20e4eeUL, 0x569ea2ecUL, 0x61f460edUL,
    0xe4e22fe8UL, 0xd388ede9UL, 0x8a36abebUL, 0xbd5c69eaUL, 0xf0b813fdUL,
    0xc7d2d1fcUL, 0x9e6c97feUL, 0xa90655ffUL, 0x2c101afaUL, 0x1b7ad8fbUL,
    0x42c49ef9UL, 0x75ae5cf8UL, 0x48e900f3UL, 0x7f83c2f2UL, 0x263d84f0UL,
    0x115746f1UL, 0x944109f4UL, 0xa32bcbf5UL, 0xfa958df7UL, 0xcdff4ff6UL,
    0x605d78d9UL, 0x5737bad8UL, 0x0e89fcdaUL, 0x39e33edbUL, 0xbcf571deUL,
    0x8b9fb3dfUL, 0xd221f5ddUL, 0xe54b37dcUL, 0xd80c6bd7UL, 0xef66a9d6UL,
    0xb6d8efd4UL, 0x81b22dd5UL, 0x04a462d0UL, 0x33cea0d1UL, 0x6a70e6d3UL,
    0x5d1a24d2UL, 0x10fe5ec5UL, 0x27949cc4UL, 0x7e2adac6UL, 0x494018c7UL,
    0xcc5657c2UL, 0xfb3c95c3UL, 0xa282d3c1UL, 0x95e811c0UL, 0xa8af4dcbUL,
    0x9fc58fcaUL, 0xc67bc9c8UL, 0xf1110bc9UL, 0x740744ccUL, 0x436d86cdUL,
    0x1ad3c0cfUL, 0x2db902ceUL, 0x4096af91UL, 0x77fc6d90UL, 0x2e422b92UL,
    0x1928e993UL, 0x9c3ea696UL, 0xab546497UL, 0xf2ea2295UL, 0xc580e094UL,
    0xf8c7bc9fUL, 0xcfad7e9eUL, 0x9613389cUL, 0xa179fa9dUL, 0x246fb598UL,
    0x13057799UL, 0x4abb319bUL, 0x7dd1f39aUL, 0x3035898dUL, 0x075f4b8cUL,
    0x5ee10d8eUL, 0x698bcf8fUL, 0xec9d808aUL, 0xdbf7428bUL, 0x82490489UL,
    0xb523c688UL, 0x88649a83UL, 0xbf0e5882UL, 0xe6b01e80UL, 0xd1dadc81UL,
    0x54cc9384UL, 0x63a65185UL, 0x3a181787UL, 0x0d72d586UL, 0xa0d0e2a9UL,
    0x97ba20a8UL, 0xce0466aaUL, 0xf96ea4abUL, 0x7c78ebaeUL, 0x4b1229afUL,
    0x12ac6fadUL, 0x25c6adacUL, 0x1881f1a7UL, 0x2feb33a6UL, 0x765575a4UL,
    0x413fb7a5UL, 0xc429f8a0UL, 0xf3433aa1UL, 0xaafd7ca3UL, 0x9d97bea2UL,
    0xd073c4b5UL, 0xe71906b4UL, 0xbea740b6UL, 0x89cd82b7UL, 0x0cdbcdb2UL,
    0x3bb10fb3UL, 0x620f49b1UL, 0x55658bb0UL, 0x6822d7bbUL, 0x5f4815baUL,
    0x06f653b8UL, 0x319c91b9UL, 0xb48adebcUL, 0x83e01cbdUL, 0xda5e5abfUL,
    0xed3498beUL
  },
  {
    0x00000000UL, 0x6567bcb8UL, 0x8bc809aaUL, 0xeeafb512UL, 0x5797628fUL,
    0x32f0de37UL, 0xdc5f6b25UL, 0xb938d79dUL, 0xef28b4c5UL, 0x8a4f087dUL,
    0x64e0bd6fUL, 0x018701d7UL, 0xb8bfd64aUL, 0xddd86af2UL, 0x3377dfe0UL,
    0x56106358UL, 0x9f571950UL, 0xfa30a5e8UL, 0x149f10faUL, 0x71f8ac42UL,
    0xc8c07bdfUL, 0xada7c767UL, 0x43087275UL, 0x266fcecdUL, 0x707fad95UL,
    0x1518112dUL, 0xfbb7a43fUL, 0x9ed01887UL, 0x27e8cf1aUL, 0x428f73a2UL,
    0xac20c6b0UL, 0xc9477a08UL, 0x3eaf32a0UL, 0x5bc88e18UL, 0xb5673b0aUL,
    0xd00087b2UL, 0x6938502fUL, 0x0c5fec97UL, 0xe2f05985UL, 0x8797e53dUL,
    0xd1878665UL, 0xb4e03addUL, 0x5a4f8fcfUL, 0x3f283377UL, 0x8610e4eaUL,
    0xe3775852UL, 0x0dd8ed40UL, 0x68bf51f8UL, 0xa1f82bf0UL, 0xc49f9748UL,
    0x2a30225aUL, 0x4f579ee2UL, 0xf66f497fUL, 0x9308f5c7UL, 0x7da740d5UL,
    0x18c0fc6dUL, 0x4ed09f35UL, 0x2bb7238dUL, 0xc518969fUL, 0xa07f2a27UL,
    0x1947fdbaUL, 0x7c204102UL, 0x928ff410UL, 0xf7e848a8UL, 0x3d58149bUL,
    0x583fa823UL, 0xb6901d31UL, 0xd3f7a189UL, 0x6acf7614UL, 0x0fa8caacUL,
    0xe1077fbeUL, 0x8460c306UL, 0xd270a05eUL, 0xb7171ce6UL, 0x59b8a9f4UL,
    0x3cdf154cUL, 0x85e7c2d1UL, 0xe0807e69UL, 0x0e2fcb7bUL, 0x6b4877c3UL,
    0xa20f0dcbUL, 0xc768b173UL, 0x29c70461UL, 0x4ca0b8d9UL, 0xf5986f44UL,
    0x90ffd3fcUL, 0x7e5066eeUL, 0x1b37da56UL, 0x4d27b90eUL, 0x284005b6UL,
    0xc6efb0a4UL, 0xa3880c1cUL, 0x1ab0db81UL, 0x7fd76739UL, 0x9178d22bUL,
    0xf41f6e93UL, 0x03f7263bUL, 0x66909a83UL, 0x883f2f91UL, 0xed589329UL,
    0x546044b4UL, 0x3107f80cUL, 0xdfa84d1eUL, 0xbacff1a6UL, 0xecdf92feUL,
    0x89b82e46UL, 0x67179b54UL, 0x027027ecUL, 0xbb48f071UL, 0xde2f4cc9UL,
    0x3080f9dbUL, 0x55e74563UL, 0x9ca03f6bUL, 0xf9c783d3UL, 0x176836c1UL,
    0x720f8a79UL, 0xcb375de4UL, 0xae50e15cUL, 0x40ff544eUL, 0x2598e8f6UL,
    0x73888baeUL, 0x16ef3716UL, 0xf8408204UL, 0x9d273ebcUL, 0x241fe921UL,
    0x41785599UL, 0xafd7e08bUL, 0xcab05c33UL, 0x3bb659edUL, 0x5ed1e555UL,
    0xb07e5047UL, 0xd519ecffUL, 0x6c213b62UL, 0x094687daUL, 0xe7e932c8UL,
    0x828e8e70UL, 0xd49eed28UL, 0xb1f95190UL, 0x5f56e482UL, 0x3a31583aUL,
    0x83098fa7UL, 0xe66e331fUL, 0x08c1860dUL, 0x6da63ab5UL, 0xa4e140bdUL,
    0xc186fc05UL, 0x2f294917UL, 0x4a4ef5afUL, 0xf3762232UL, 0x96119e8aUL,
    0x78be2b98UL, 0x1dd99720UL, 0x4bc9f478UL, 0x2eae48c0UL, 0xc001fdd2UL,
    0xa566416aUL, 0x1c5e96f7UL, 0x79392a4fUL, 0x97969f5dUL, 0xf2f123e5UL,
    0x05196b4dUL, 0x607ed7f5UL, 0x8ed162e7UL, 0xebb6de5fUL, 0x528e09c2UL,
    0x37e9b57aUL, 0xd9460068UL, 0xbc21bcd0UL, 0xea31df88UL, 0x8f566330UL,
    0x61f9d622UL, 0x049e6a9aUL, 0xbda6bd07UL, 0xd8c101bfUL, 0x366eb4adUL,
    0x53090815UL, 0x9a4e721dUL, 0xff29cea5UL, 0x11867bb7UL, 0x74e1c70fUL,
    0xcdd91092UL, 0xa8beac2aUL, 0x46111938UL, 0x2376a580UL, 0x7566c6d8UL,
    0x10017a60UL, 0xfeaecf72UL, 0x9bc973caUL, 0x22f1a457UL, 0x479618efUL,
    0xa939adfdUL, 0xcc5e1145UL, 0x06ee4d76UL, 0x6389f1ceUL, 0x8d2644dcUL,
    0xe841f864UL, 0x51792ff9UL, 0x341e9341UL, 0xdab12653UL, 0xbfd69aebUL,
    0xe9c6f9b3UL, 0x8ca1450bUL, 0x620ef019UL, 0x07694ca1UL, 0xbe519b3cUL,
    0xdb362784UL, 0x35999296UL, 0x50fe2e2eUL, 0x99b95426UL, 0xfcdee89eUL,
    0x12715d8cUL, 0x7716e134UL, 0xce2e36a9UL, 0xab498a11UL, 0x45e63f03UL,
    0x208183bbUL, 0x7691e0e3UL, 0x13f65c5bUL, 0xfd59e949UL, 0x983e55f1UL,
    0x2106826cUL, 0x44613ed4UL, 0xaace8bc6UL, 0xcfa9377eUL, 0x38417fd6UL,
    0x5d26c36eUL, 0xb389767cUL, 0xd6eecac4UL, 0x6fd61d59UL, 0x0ab1a1e1UL,
    0xe41e14f3UL, 0x8179a84bUL, 0xd769cb13UL, 0xb20e77abUL, 0x5ca1c2b9UL,
    0x39c67e01UL, 0x80fea99cUL, 0xe5991524UL, 0x0b36a036UL, 0x6e511c8eUL,
    0xa7166686UL, 0xc271da3eUL, 0x2cde6f2cUL, 0x49b9d394UL, 0xf0810409UL,
    0x95e6b8b1UL, 0x7b490da3UL, 0x1e2eb11bUL, 0x483ed243UL, 0x2d596efbUL,
    0xc3f6dbe9UL, 0xa6916751UL, 0x1fa9b0ccUL, 0x7ace0c74UL, 0x9461b966UL,
    0xf10605deUL
  }
};
const z_crc_t * get_crc_table()
{
    return (const z_crc_t *)crc_table;
}
unsigned long crc32_z(crc, buf, len)
    unsigned long crc;
    const unsigned char *buf;
    z_size_t len;
{
    if (buf == 0) return 0UL;
    if (sizeof(void *) == sizeof(ptrdiff_t)) {
        z_crc_t endian;
        endian = 1;
        if (*((unsigned char *)(&endian)))
            return crc32_little(crc, buf, len);
        else
            return crc32_big(crc, buf, len);
    }
    crc = crc ^ 0xffffffffUL;
    while (len >= 8) {
        crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8); crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8); crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8); crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8); crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8); crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8); crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8); crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8);
        len -= 8;
    }
    if (len) do {
        crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8);
    } while (--len);
    return crc ^ 0xffffffffUL;
}
unsigned long crc32(crc, buf, len)
    unsigned long crc;
    const unsigned char *buf;
    uInt len;
{
    return crc32_z(crc, buf, len);
}
static unsigned long crc32_little(crc, buf, len)
    unsigned long crc;
    const unsigned char *buf;
    z_size_t len;
{
    register z_crc_t c;
    register const z_crc_t *buf4;
    c = (z_crc_t)crc;
    c = ~c;
    while (len && ((ptrdiff_t)buf & 3)) {
        c = crc_table[0][(c ^ *buf++) & 0xff] ^ (c >> 8);
        len--;
    }
    buf4 = (const z_crc_t *)(const void *)buf;
    while (len >= 32) {
        c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24]; c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24]; c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24]; c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24]; c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24]; c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24]; c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24]; c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24];
        len -= 32;
    }
    while (len >= 4) {
        c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24];
        len -= 4;
    }
    buf = (const unsigned char *)buf4;
    if (len) do {
        c = crc_table[0][(c ^ *buf++) & 0xff] ^ (c >> 8);
    } while (--len);
    c = ~c;
    return (unsigned long)c;
}
static unsigned long crc32_big(crc, buf, len)
    unsigned long crc;
    const unsigned char *buf;
    z_size_t len;
{
    register z_crc_t c;
    register const z_crc_t *buf4;
    c = (((((z_crc_t)crc) >> 24) & 0xff) + ((((z_crc_t)crc) >> 8) & 0xff00) + ((((z_crc_t)crc) & 0xff00) << 8) + ((((z_crc_t)crc) & 0xff) << 24));
    c = ~c;
    while (len && ((ptrdiff_t)buf & 3)) {
        c = crc_table[4][(c >> 24) ^ *buf++] ^ (c << 8);
        len--;
    }
    buf4 = (const z_crc_t *)(const void *)buf;
    while (len >= 32) {
        c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24]; c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24]; c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24]; c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24]; c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24]; c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24]; c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24]; c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24];
        len -= 32;
    }
    while (len >= 4) {
        c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24];
        len -= 4;
    }
    buf = (const unsigned char *)buf4;
    if (len) do {
        c = crc_table[4][(c >> 24) ^ *buf++] ^ (c << 8);
    } while (--len);
    c = ~c;
    return (unsigned long)(((((c) >> 24) & 0xff) + (((c) >> 8) & 0xff00) + (((c) & 0xff00) << 8) + (((c) & 0xff) << 24)));
}
static unsigned long gf2_matrix_times(mat, vec)
    unsigned long *mat;
    unsigned long vec;
{
    unsigned long sum;
    sum = 0;
    while (vec) {
        if (vec & 1)
            sum ^= *mat;
        vec >>= 1;
        mat++;
    }
    return sum;
}
static void gf2_matrix_square(square, mat)
    unsigned long *square;
    unsigned long *mat;
{
    int n;
    for (n = 0; n < 32; n++)
        square[n] = gf2_matrix_times(mat, mat[n]);
}
static uLong crc32_combine_(crc1, crc2, len2)
    uLong crc1;
    uLong crc2;
    long len2;
{
    int n;
    unsigned long row;
    unsigned long even[32];
    unsigned long odd[32];
    if (len2 <= 0)
        return crc1;
    odd[0] = 0xedb88320UL;
    row = 1;
    for (n = 1; n < 32; n++) {
        odd[n] = row;
        row <<= 1;
    }
    gf2_matrix_square(even, odd);
    gf2_matrix_square(odd, even);
    do {
        gf2_matrix_square(even, odd);
        if (len2 & 1)
            crc1 = gf2_matrix_times(even, crc1);
        len2 >>= 1;
        if (len2 == 0)
            break;
        gf2_matrix_square(odd, even);
        if (len2 & 1)
            crc1 = gf2_matrix_times(odd, crc1);
        len2 >>= 1;
    } while (len2 != 0);
    crc1 ^= crc2;
    return crc1;
}
uLong crc32_combine(crc1, crc2, len2)
    uLong crc1;
    uLong crc2;
    long len2;
{
    return crc32_combine_(crc1, crc2, len2);
}
uLong crc32_combine64(crc1, crc2, len2)
    uLong crc1;
    uLong crc2;
    long len2;
{
    return crc32_combine_(crc1, crc2, len2);
}
        char * const z_errmsg[10] = {
    ( char *)"need dictionary",
    ( char *)"stream end",
    ( char *)"",
    ( char *)"file error",
    ( char *)"stream error",
    ( char *)"data error",
    ( char *)"insufficient memory",
    ( char *)"buffer error",
    ( char *)"incompatible version",
    ( char *)""
};
const char * zlibVersion()
{
    return "1.2.11-optipng";
}
uLong zlibCompileFlags()
{
    uLong flags;
    flags = 0;
    switch ((int)(sizeof(uInt))) {
    case 2: break;
    case 4: flags += 1; break;
    case 8: flags += 2; break;
    default: flags += 3;
    }
    switch ((int)(sizeof(uLong))) {
    case 2: break;
    case 4: flags += 1 << 2; break;
    case 8: flags += 2 << 2; break;
    default: flags += 3 << 2;
    }
    switch ((int)(sizeof(voidpf))) {
    case 2: break;
    case 4: flags += 1 << 4; break;
    case 8: flags += 2 << 4; break;
    default: flags += 3 << 4;
    }
    switch ((int)(sizeof(long))) {
    case 2: break;
    case 4: flags += 1 << 6; break;
    case 8: flags += 2 << 6; break;
    default: flags += 3 << 6;
    }
    flags += 1L << 16;
    flags += 1L << 17;
    return flags;
}
const char * zError(err)
    int err;
{
    return z_errmsg[2 -(err)];
}
voidpf zcalloc (opaque, items, size)
    voidpf opaque;
    unsigned items;
    unsigned size;
{
    (void)opaque;
    return sizeof(uInt) > 2 ? (voidpf)malloc(items * size) :
                              (voidpf)calloc(items, size);
}
void zcfree (opaque, ptr)
    voidpf opaque;
    voidpf ptr;
{
    (void)opaque;
    free(ptr);
}
static const int extra_lbits[29]
   = {0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,0};
static const int extra_dbits[30]
   = {0,0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13};
static const int extra_blbits[19]
   = {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,3,7};
static const uch bl_order[19]
   = {16,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1,15};
static const ct_data static_ltree[(256 +1+29)+2] = {
{{ 12},{ 8}}, {{140},{ 8}}, {{ 76},{ 8}}, {{204},{ 8}}, {{ 44},{ 8}},
{{172},{ 8}}, {{108},{ 8}}, {{236},{ 8}}, {{ 28},{ 8}}, {{156},{ 8}},
{{ 92},{ 8}}, {{220},{ 8}}, {{ 60},{ 8}}, {{188},{ 8}}, {{124},{ 8}},
{{252},{ 8}}, {{ 2},{ 8}}, {{130},{ 8}}, {{ 66},{ 8}}, {{194},{ 8}},
{{ 34},{ 8}}, {{162},{ 8}}, {{ 98},{ 8}}, {{226},{ 8}}, {{ 18},{ 8}},
{{146},{ 8}}, {{ 82},{ 8}}, {{210},{ 8}}, {{ 50},{ 8}}, {{178},{ 8}},
{{114},{ 8}}, {{242},{ 8}}, {{ 10},{ 8}}, {{138},{ 8}}, {{ 74},{ 8}},
{{202},{ 8}}, {{ 42},{ 8}}, {{170},{ 8}}, {{106},{ 8}}, {{234},{ 8}},
{{ 26},{ 8}}, {{154},{ 8}}, {{ 90},{ 8}}, {{218},{ 8}}, {{ 58},{ 8}},
{{186},{ 8}}, {{122},{ 8}}, {{250},{ 8}}, {{ 6},{ 8}}, {{134},{ 8}},
{{ 70},{ 8}}, {{198},{ 8}}, {{ 38},{ 8}}, {{166},{ 8}}, {{102},{ 8}},
{{230},{ 8}}, {{ 22},{ 8}}, {{150},{ 8}}, {{ 86},{ 8}}, {{214},{ 8}},
{{ 54},{ 8}}, {{182},{ 8}}, {{118},{ 8}}, {{246},{ 8}}, {{ 14},{ 8}},
{{142},{ 8}}, {{ 78},{ 8}}, {{206},{ 8}}, {{ 46},{ 8}}, {{174},{ 8}},
{{110},{ 8}}, {{238},{ 8}}, {{ 30},{ 8}}, {{158},{ 8}}, {{ 94},{ 8}},
{{222},{ 8}}, {{ 62},{ 8}}, {{190},{ 8}}, {{126},{ 8}}, {{254},{ 8}},
{{ 1},{ 8}}, {{129},{ 8}}, {{ 65},{ 8}}, {{193},{ 8}}, {{ 33},{ 8}},
{{161},{ 8}}, {{ 97},{ 8}}, {{225},{ 8}}, {{ 17},{ 8}}, {{145},{ 8}},
{{ 81},{ 8}}, {{209},{ 8}}, {{ 49},{ 8}}, {{177},{ 8}}, {{113},{ 8}},
{{241},{ 8}}, {{ 9},{ 8}}, {{137},{ 8}}, {{ 73},{ 8}}, {{201},{ 8}},
{{ 41},{ 8}}, {{169},{ 8}}, {{105},{ 8}}, {{233},{ 8}}, {{ 25},{ 8}},
{{153},{ 8}}, {{ 89},{ 8}}, {{217},{ 8}}, {{ 57},{ 8}}, {{185},{ 8}},
{{121},{ 8}}, {{249},{ 8}}, {{ 5},{ 8}}, {{133},{ 8}}, {{ 69},{ 8}},
{{197},{ 8}}, {{ 37},{ 8}}, {{165},{ 8}}, {{101},{ 8}}, {{229},{ 8}},
{{ 21},{ 8}}, {{149},{ 8}}, {{ 85},{ 8}}, {{213},{ 8}}, {{ 53},{ 8}},
{{181},{ 8}}, {{117},{ 8}}, {{245},{ 8}}, {{ 13},{ 8}}, {{141},{ 8}},
{{ 77},{ 8}}, {{205},{ 8}}, {{ 45},{ 8}}, {{173},{ 8}}, {{109},{ 8}},
{{237},{ 8}}, {{ 29},{ 8}}, {{157},{ 8}}, {{ 93},{ 8}}, {{221},{ 8}},
{{ 61},{ 8}}, {{189},{ 8}}, {{125},{ 8}}, {{253},{ 8}}, {{ 19},{ 9}},
{{275},{ 9}}, {{147},{ 9}}, {{403},{ 9}}, {{ 83},{ 9}}, {{339},{ 9}},
{{211},{ 9}}, {{467},{ 9}}, {{ 51},{ 9}}, {{307},{ 9}}, {{179},{ 9}},
{{435},{ 9}}, {{115},{ 9}}, {{371},{ 9}}, {{243},{ 9}}, {{499},{ 9}},
{{ 11},{ 9}}, {{267},{ 9}}, {{139},{ 9}}, {{395},{ 9}}, {{ 75},{ 9}},
{{331},{ 9}}, {{203},{ 9}}, {{459},{ 9}}, {{ 43},{ 9}}, {{299},{ 9}},
{{171},{ 9}}, {{427},{ 9}}, {{107},{ 9}}, {{363},{ 9}}, {{235},{ 9}},
{{491},{ 9}}, {{ 27},{ 9}}, {{283},{ 9}}, {{155},{ 9}}, {{411},{ 9}},
{{ 91},{ 9}}, {{347},{ 9}}, {{219},{ 9}}, {{475},{ 9}}, {{ 59},{ 9}},
{{315},{ 9}}, {{187},{ 9}}, {{443},{ 9}}, {{123},{ 9}}, {{379},{ 9}},
{{251},{ 9}}, {{507},{ 9}}, {{ 7},{ 9}}, {{263},{ 9}}, {{135},{ 9}},
{{391},{ 9}}, {{ 71},{ 9}}, {{327},{ 9}}, {{199},{ 9}}, {{455},{ 9}},
{{ 39},{ 9}}, {{295},{ 9}}, {{167},{ 9}}, {{423},{ 9}}, {{103},{ 9}},
{{359},{ 9}}, {{231},{ 9}}, {{487},{ 9}}, {{ 23},{ 9}}, {{279},{ 9}},
{{151},{ 9}}, {{407},{ 9}}, {{ 87},{ 9}}, {{343},{ 9}}, {{215},{ 9}},
{{471},{ 9}}, {{ 55},{ 9}}, {{311},{ 9}}, {{183},{ 9}}, {{439},{ 9}},
{{119},{ 9}}, {{375},{ 9}}, {{247},{ 9}}, {{503},{ 9}}, {{ 15},{ 9}},
{{271},{ 9}}, {{143},{ 9}}, {{399},{ 9}}, {{ 79},{ 9}}, {{335},{ 9}},
{{207},{ 9}}, {{463},{ 9}}, {{ 47},{ 9}}, {{303},{ 9}}, {{175},{ 9}},
{{431},{ 9}}, {{111},{ 9}}, {{367},{ 9}}, {{239},{ 9}}, {{495},{ 9}},
{{ 31},{ 9}}, {{287},{ 9}}, {{159},{ 9}}, {{415},{ 9}}, {{ 95},{ 9}},
{{351},{ 9}}, {{223},{ 9}}, {{479},{ 9}}, {{ 63},{ 9}}, {{319},{ 9}},
{{191},{ 9}}, {{447},{ 9}}, {{127},{ 9}}, {{383},{ 9}}, {{255},{ 9}},
{{511},{ 9}}, {{ 0},{ 7}}, {{ 64},{ 7}}, {{ 32},{ 7}}, {{ 96},{ 7}},
{{ 16},{ 7}}, {{ 80},{ 7}}, {{ 48},{ 7}}, {{112},{ 7}}, {{ 8},{ 7}},
{{ 72},{ 7}}, {{ 40},{ 7}}, {{104},{ 7}}, {{ 24},{ 7}}, {{ 88},{ 7}},
{{ 56},{ 7}}, {{120},{ 7}}, {{ 4},{ 7}}, {{ 68},{ 7}}, {{ 36},{ 7}},
{{100},{ 7}}, {{ 20},{ 7}}, {{ 84},{ 7}}, {{ 52},{ 7}}, {{116},{ 7}},
{{ 3},{ 8}}, {{131},{ 8}}, {{ 67},{ 8}}, {{195},{ 8}}, {{ 35},{ 8}},
{{163},{ 8}}, {{ 99},{ 8}}, {{227},{ 8}}
};
static const ct_data static_dtree[30] = {
{{ 0},{ 5}}, {{16},{ 5}}, {{ 8},{ 5}}, {{24},{ 5}}, {{ 4},{ 5}},
{{20},{ 5}}, {{12},{ 5}}, {{28},{ 5}}, {{ 2},{ 5}}, {{18},{ 5}},
{{10},{ 5}}, {{26},{ 5}}, {{ 6},{ 5}}, {{22},{ 5}}, {{14},{ 5}},
{{30},{ 5}}, {{ 1},{ 5}}, {{17},{ 5}}, {{ 9},{ 5}}, {{25},{ 5}},
{{ 5},{ 5}}, {{21},{ 5}}, {{13},{ 5}}, {{29},{ 5}}, {{ 3},{ 5}},
{{19},{ 5}}, {{11},{ 5}}, {{27},{ 5}}, {{ 7},{ 5}}, {{23},{ 5}}
};
const uch _dist_code[512] = {
 0, 1, 2, 3, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8,
 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10,
10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 13, 13, 13, 13,
13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
13, 13, 13, 13, 13, 13, 13, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 15, 15,
15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 0, 0, 16, 17,
18, 18, 19, 19, 20, 20, 20, 20, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22, 22,
23, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
24, 24, 24, 24, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 27, 27, 27, 27, 27, 27, 27, 27,
27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27,
27, 27, 27, 27, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
28, 28, 28, 28, 28, 28, 28, 28, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29
};
const uch _length_code[258 -3 +1]= {
 0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 12, 12,
13, 13, 13, 13, 14, 14, 14, 14, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16, 16, 16,
17, 17, 17, 17, 17, 17, 17, 17, 18, 18, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19,
19, 19, 19, 19, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22,
22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 23,
23, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 26, 26, 26, 26, 26, 26, 26, 26,
26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
26, 26, 26, 26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27,
27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28
};
static const int base_length[29] = {
0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 20, 24, 28, 32, 40, 48, 56,
64, 80, 96, 112, 128, 160, 192, 224, 0
};
static const int base_dist[30] = {
    0, 1, 2, 3, 4, 6, 8, 12, 16, 24,
   32, 48, 64, 96, 128, 192, 256, 384, 512, 768,
 1024, 1536, 2048, 3072, 4096, 6144, 8192, 12288, 16384, 24576
};
struct static_tree_desc_s {
    const ct_data *static_tree;
    const intf *extra_bits;
    int extra_base;
    int elems;
    int max_length;
};
static const static_tree_desc static_l_desc =
{static_ltree, extra_lbits, 256 +1, (256 +1+29), 15};
static const static_tree_desc static_d_desc =
{static_dtree, extra_dbits, 0, 30, 15};
static const static_tree_desc static_bl_desc =
{(const ct_data *)0, extra_blbits, 0, 19, 7};
static void tr_static_init (void);
static void init_block (deflate_state *s);
static void pqdownheap (deflate_state *s, ct_data *tree, int k);
static void gen_bitlen (deflate_state *s, tree_desc *desc);
static void gen_codes (ct_data *tree, int max_code, ushf *bl_count);
static void build_tree (deflate_state *s, tree_desc *desc);
static void scan_tree (deflate_state *s, ct_data *tree, int max_code);
static void send_tree (deflate_state *s, ct_data *tree, int max_code);
static int build_bl_tree (deflate_state *s);
static void send_all_trees (deflate_state *s, int lcodes, int dcodes, int blcodes);
static void compress_block (deflate_state *s, const ct_data *ltree, const ct_data *dtree);
static int detect_data_type (deflate_state *s);
static unsigned bi_reverse (unsigned value, int length);
static void bi_windup (deflate_state *s);
static void bi_flush (deflate_state *s);
static void tr_static_init()
{
}
void _tr_init(s)
    deflate_state *s;
{
    tr_static_init();
    s->l_desc.dyn_tree = s->dyn_ltree;
    s->l_desc.stat_desc = &static_l_desc;
    s->d_desc.dyn_tree = s->dyn_dtree;
    s->d_desc.stat_desc = &static_d_desc;
    s->bl_desc.dyn_tree = s->bl_tree;
    s->bl_desc.stat_desc = &static_bl_desc;
    s->bi_buf = 0;
    s->bi_valid = 0;
    init_block(s);
}
static void init_block(s)
    deflate_state *s;
{
    int n;
    for (n = 0; n < (256 +1+29); n++) s->dyn_ltree[n].fc.freq = 0;
    for (n = 0; n < 30; n++) s->dyn_dtree[n].fc.freq = 0;
    for (n = 0; n < 19; n++) s->bl_tree[n].fc.freq = 0;
    s->dyn_ltree[256].fc.freq = 1;
    s->opt_len = s->static_len = 0L;
    s->last_lit = s->matches = 0;
}
static void pqdownheap(s, tree, k)
    deflate_state *s;
    ct_data *tree;
    int k;
{
    int v = s->heap[k];
    int j = k << 1;
    while (j <= s->heap_len) {
        if (j < s->heap_len &&
            (tree[s->heap[j+1]].fc.freq < tree[s->heap[j]].fc.freq || (tree[s->heap[j+1]].fc.freq == tree[s->heap[j]].fc.freq && s->depth[s->heap[j+1]] <= s->depth[s->heap[j]]))) {
            j++;
        }
        if ((tree[v].fc.freq < tree[s->heap[j]].fc.freq || (tree[v].fc.freq == tree[s->heap[j]].fc.freq && s->depth[v] <= s->depth[s->heap[j]]))) break;
        s->heap[k] = s->heap[j]; k = j;
        j <<= 1;
    }
    s->heap[k] = v;
}
static void gen_bitlen(s, desc)
    deflate_state *s;
    tree_desc *desc;
{
    ct_data *tree = desc->dyn_tree;
    int max_code = desc->max_code;
    const ct_data *stree = desc->stat_desc->static_tree;
    const intf *extra = desc->stat_desc->extra_bits;
    int base = desc->stat_desc->extra_base;
    int max_length = desc->stat_desc->max_length;
    int h;
    int n, m;
    int bits;
    int xbits;
    ush f;
    int overflow = 0;
    for (bits = 0; bits <= 15; bits++) s->bl_count[bits] = 0;
    tree[s->heap[s->heap_max]].dl.len = 0;
    for (h = s->heap_max+1; h < (2*(256 +1+29)+1); h++) {
        n = s->heap[h];
        bits = tree[tree[n].dl.dad].dl.len + 1;
        if (bits > max_length) bits = max_length, overflow++;
        tree[n].dl.len = (ush)bits;
        if (n > max_code) continue;
        s->bl_count[bits]++;
        xbits = 0;
        if (n >= base) xbits = extra[n-base];
        f = tree[n].fc.freq;
        s->opt_len += (ulg)f * (unsigned)(bits + xbits);
        if (stree) s->static_len += (ulg)f * (unsigned)(stree[n].dl.len + xbits);
    }
    if (overflow == 0) return;
                                              ;
    do {
        bits = max_length-1;
        while (s->bl_count[bits] == 0) bits--;
        s->bl_count[bits]--;
        s->bl_count[bits+1] += 2;
        s->bl_count[max_length]--;
        overflow -= 2;
    } while (overflow > 0);
    for (bits = max_length; bits != 0; bits--) {
        n = s->bl_count[bits];
        while (n != 0) {
            m = s->heap[--h];
            if (m > max_code) continue;
            if ((unsigned) tree[m].dl.len != (unsigned) bits) {
                                                                              ;
                s->opt_len += ((ulg)bits - tree[m].dl.len) * tree[m].fc.freq;
                tree[m].dl.len = (ush)bits;
            }
            n--;
        }
    }
}
static void gen_codes (tree, max_code, bl_count)
    ct_data *tree;
    int max_code;
    ushf *bl_count;
{
    ush next_code[15 +1];
    unsigned code = 0;
    int bits;
    int n;
    for (bits = 1; bits <= 15; bits++) {
        code = (code + bl_count[bits-1]) << 1;
        next_code[bits] = (ush)code;
    }
                                      ;
                                                          ;
    for (n = 0; n <= max_code; n++) {
        int len = tree[n].dl.len;
        if (len == 0) continue;
        tree[n].fc.code = (ush)bi_reverse(next_code[len]++, len);
                                                                             ;
    }
}
static void build_tree(s, desc)
    deflate_state *s;
    tree_desc *desc;
{
    ct_data *tree = desc->dyn_tree;
    const ct_data *stree = desc->stat_desc->static_tree;
    int elems = desc->stat_desc->elems;
    int n, m;
    int max_code = -1;
    int node;
    s->heap_len = 0, s->heap_max = (2*(256 +1+29)+1);
    for (n = 0; n < elems; n++) {
        if (tree[n].fc.freq != 0) {
            s->heap[++(s->heap_len)] = max_code = n;
            s->depth[n] = 0;
        } else {
            tree[n].dl.len = 0;
        }
    }
    while (s->heap_len < 2) {
        node = s->heap[++(s->heap_len)] = (max_code < 2 ? ++max_code : 0);
        tree[node].fc.freq = 1;
        s->depth[node] = 0;
        s->opt_len--; if (stree) s->static_len -= stree[node].dl.len;
    }
    desc->max_code = max_code;
    for (n = s->heap_len/2; n >= 1; n--) pqdownheap(s, tree, n);
    node = elems;
    do {
        { n = s->heap[1]; s->heap[1] = s->heap[s->heap_len--]; pqdownheap(s, tree, 1); };
        m = s->heap[1];
        s->heap[--(s->heap_max)] = n;
        s->heap[--(s->heap_max)] = m;
        tree[node].fc.freq = tree[n].fc.freq + tree[m].fc.freq;
        s->depth[node] = (uch)((s->depth[n] >= s->depth[m] ?
                                s->depth[n] : s->depth[m]) + 1);
        tree[n].dl.dad = tree[m].dl.dad = (ush)node;
        s->heap[1] = node++;
        pqdownheap(s, tree, 1);
    } while (s->heap_len >= 2);
    s->heap[--(s->heap_max)] = s->heap[1];
    gen_bitlen(s, (tree_desc *)desc);
    gen_codes ((ct_data *)tree, max_code, s->bl_count);
}
static void scan_tree (s, tree, max_code)
    deflate_state *s;
    ct_data *tree;
    int max_code;
{
    int n;
    int prevlen = -1;
    int curlen;
    int nextlen = tree[0].dl.len;
    int count = 0;
    int max_count = 7;
    int min_count = 4;
    if (nextlen == 0) max_count = 138, min_count = 3;
    tree[max_code+1].dl.len = (ush)0xffff;
    for (n = 0; n <= max_code; n++) {
        curlen = nextlen; nextlen = tree[n+1].dl.len;
        if (++count < max_count && curlen == nextlen) {
            continue;
        } else if (count < min_count) {
            s->bl_tree[curlen].fc.freq += count;
        } else if (curlen != 0) {
            if (curlen != prevlen) s->bl_tree[curlen].fc.freq++;
            s->bl_tree[16].fc.freq++;
        } else if (count <= 10) {
            s->bl_tree[17].fc.freq++;
        } else {
            s->bl_tree[18].fc.freq++;
        }
        count = 0; prevlen = curlen;
        if (nextlen == 0) {
            max_count = 138, min_count = 3;
        } else if (curlen == nextlen) {
            max_count = 6, min_count = 3;
        } else {
            max_count = 7, min_count = 4;
        }
    }
}
static void send_tree (s, tree, max_code)
    deflate_state *s;
    ct_data *tree;
    int max_code;
{
    int n;
    int prevlen = -1;
    int curlen;
    int nextlen = tree[0].dl.len;
    int count = 0;
    int max_count = 7;
    int min_count = 4;
    if (nextlen == 0) max_count = 138, min_count = 3;
    for (n = 0; n <= max_code; n++) {
        curlen = nextlen; nextlen = tree[n+1].dl.len;
        if (++count < max_count && curlen == nextlen) {
            continue;
        } else if (count < min_count) {
            do { { int len = s->bl_tree[curlen].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)s->bl_tree[curlen].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(s->bl_tree[curlen].fc.code) << s->bi_valid; s->bi_valid += len; }}; } while (--count != 0);
        } else if (curlen != 0) {
            if (curlen != prevlen) {
                { int len = s->bl_tree[curlen].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)s->bl_tree[curlen].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(s->bl_tree[curlen].fc.code) << s->bi_valid; s->bi_valid += len; }}; count--;
            }
                                                     ;
            { int len = s->bl_tree[16].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)s->bl_tree[16].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(s->bl_tree[16].fc.code) << s->bi_valid; s->bi_valid += len; }}; { int len = 2; if (s->bi_valid > (int)16 - len) { int val = (int)count-3; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(count-3) << s->bi_valid; s->bi_valid += len; }};
        } else if (count <= 10) {
            { int len = s->bl_tree[17].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)s->bl_tree[17].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(s->bl_tree[17].fc.code) << s->bi_valid; s->bi_valid += len; }}; { int len = 3; if (s->bi_valid > (int)16 - len) { int val = (int)count-3; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(count-3) << s->bi_valid; s->bi_valid += len; }};
        } else {
            { int len = s->bl_tree[18].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)s->bl_tree[18].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(s->bl_tree[18].fc.code) << s->bi_valid; s->bi_valid += len; }}; { int len = 7; if (s->bi_valid > (int)16 - len) { int val = (int)count-11; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(count-11) << s->bi_valid; s->bi_valid += len; }};
        }
        count = 0; prevlen = curlen;
        if (nextlen == 0) {
            max_count = 138, min_count = 3;
        } else if (curlen == nextlen) {
            max_count = 6, min_count = 3;
        } else {
            max_count = 7, min_count = 4;
        }
    }
}
static int build_bl_tree(s)
    deflate_state *s;
{
    int max_blindex;
    scan_tree(s, (ct_data *)s->dyn_ltree, s->l_desc.max_code);
    scan_tree(s, (ct_data *)s->dyn_dtree, s->d_desc.max_code);
    build_tree(s, (tree_desc *)(&(s->bl_desc)));
    for (max_blindex = 19 -1; max_blindex >= 3; max_blindex--) {
        if (s->bl_tree[bl_order[max_blindex]].dl.len != 0) break;
    }
    s->opt_len += 3*((ulg)max_blindex+1) + 5+5+4;
                                       ;
    return max_blindex;
}
static void send_all_trees(s, lcodes, dcodes, blcodes)
    deflate_state *s;
    int lcodes, dcodes, blcodes;
{
    int rank;
                                                                             ;
                             ;
                                     ;
    { int len = 5; if (s->bi_valid > (int)16 - len) { int val = (int)lcodes-257; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(lcodes-257) << s->bi_valid; s->bi_valid += len; }};
    { int len = 5; if (s->bi_valid > (int)16 - len) { int val = (int)dcodes-1; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(dcodes-1) << s->bi_valid; s->bi_valid += len; }};
    { int len = 4; if (s->bi_valid > (int)16 - len) { int val = (int)blcodes-4; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(blcodes-4) << s->bi_valid; s->bi_valid += len; }};
    for (rank = 0; rank < blcodes; rank++) {
                                                          ;
        { int len = 3; if (s->bi_valid > (int)16 - len) { int val = (int)s->bl_tree[bl_order[rank]].dl.len; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(s->bl_tree[bl_order[rank]].dl.len) << s->bi_valid; s->bi_valid += len; }};
    }
                                                         ;
    send_tree(s, (ct_data *)s->dyn_ltree, lcodes-1);
                                                          ;
    send_tree(s, (ct_data *)s->dyn_dtree, dcodes-1);
                                                           ;
}
void _tr_stored_block(s, buf, stored_len, last)
    deflate_state *s;
    charf *buf;
    ulg stored_len;
    int last;
{
    { int len = 3; if (s->bi_valid > (int)16 - len) { int val = (int)(0<<1)+last; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)((0<<1)+last) << s->bi_valid; s->bi_valid += len; }};
    bi_windup(s);
    { {s->pending_buf[s->pending++] = (Bytef)((uch)(((ush)stored_len) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)((ush)stored_len) >> 8));}; };
    { {s->pending_buf[s->pending++] = (Bytef)((uch)(((ush)~stored_len) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)((ush)~stored_len) >> 8));}; };
    memcpy(s->pending_buf + s->pending, (Bytef *)buf, stored_len);
    s->pending += stored_len;
}
void _tr_flush_bits(s)
    deflate_state *s;
{
    bi_flush(s);
}
void _tr_align(s)
    deflate_state *s;
{
    { int len = 3; if (s->bi_valid > (int)16 - len) { int val = (int)1<<1; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(1<<1) << s->bi_valid; s->bi_valid += len; }};
    { int len = static_ltree[256].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)static_ltree[256].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(static_ltree[256].fc.code) << s->bi_valid; s->bi_valid += len; }};
    bi_flush(s);
}
void _tr_flush_block(s, buf, stored_len, last)
    deflate_state *s;
    charf *buf;
    ulg stored_len;
    int last;
{
    ulg opt_lenb, static_lenb;
    int max_blindex = 0;
    if (s->level > 0) {
        if (s->strm->data_type == 2)
            s->strm->data_type = detect_data_type(s);
        build_tree(s, (tree_desc *)(&(s->l_desc)));
                               ;
        build_tree(s, (tree_desc *)(&(s->d_desc)));
                               ;
        max_blindex = build_bl_tree(s);
        opt_lenb = (s->opt_len+3+7)>>3;
        static_lenb = (s->static_len+3+7)>>3;
                             ;
        if (static_lenb <= opt_lenb) opt_lenb = static_lenb;
    } else {
                                           ;
        opt_lenb = static_lenb = stored_len + 5;
    }
    if (stored_len+4 <= opt_lenb && buf != (char*)0) {
        _tr_stored_block(s, buf, stored_len, last);
    } else if (s->strategy == 4 || static_lenb == opt_lenb) {
        { int len = 3; if (s->bi_valid > (int)16 - len) { int val = (int)(1<<1)+last; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)((1<<1)+last) << s->bi_valid; s->bi_valid += len; }};
        compress_block(s, (const ct_data *)static_ltree,
                       (const ct_data *)static_dtree);
    } else {
        { int len = 3; if (s->bi_valid > (int)16 - len) { int val = (int)(2<<1)+last; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)((2<<1)+last) << s->bi_valid; s->bi_valid += len; }};
        send_all_trees(s, s->l_desc.max_code+1, s->d_desc.max_code+1,
                       max_blindex+1);
        compress_block(s, (const ct_data *)s->dyn_ltree,
                       (const ct_data *)s->dyn_dtree);
    }
                                                                     ;
    init_block(s);
    if (last) {
        bi_windup(s);
    }
                                     ;
}
int _tr_tally (s, dist, lc)
    deflate_state *s;
    unsigned dist;
    unsigned lc;
{
    s->d_buf[s->last_lit] = (ush)dist;
    s->l_buf[s->last_lit++] = (uch)lc;
    if (dist == 0) {
        s->dyn_ltree[lc].fc.freq++;
    } else {
        s->matches++;
        dist--;
                                                                         ;
        s->dyn_ltree[_length_code[lc]+256 +1].fc.freq++;
        s->dyn_dtree[((dist) < 256 ? _dist_code[dist] : _dist_code[256+((dist)>>7)])].fc.freq++;
    }
    return (s->last_lit == s->lit_bufsize-1);
}
static void compress_block(s, ltree, dtree)
    deflate_state *s;
    const ct_data *ltree;
    const ct_data *dtree;
{
    unsigned dist;
    int lc;
    unsigned lx = 0;
    unsigned code;
    int extra;
    if (s->last_lit != 0) do {
        dist = s->d_buf[lx];
        lc = s->l_buf[lx++];
        if (dist == 0) {
            { int len = ltree[lc].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)ltree[lc].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(ltree[lc].fc.code) << s->bi_valid; s->bi_valid += len; }};
                                                       ;
        } else {
            code = _length_code[lc];
            { int len = ltree[code+256 +1].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)ltree[code+256 +1].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(ltree[code+256 +1].fc.code) << s->bi_valid; s->bi_valid += len; }};
            extra = extra_lbits[code];
            if (extra != 0) {
                lc -= base_length[code];
                { int len = extra; if (s->bi_valid > (int)16 - len) { int val = (int)lc; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(lc) << s->bi_valid; s->bi_valid += len; }};
            }
            dist--;
            code = ((dist) < 256 ? _dist_code[dist] : _dist_code[256+((dist)>>7)]);
                                                 ;
            { int len = dtree[code].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)dtree[code].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(dtree[code].fc.code) << s->bi_valid; s->bi_valid += len; }};
            extra = extra_dbits[code];
            if (extra != 0) {
                dist -= (unsigned)base_dist[code];
                { int len = extra; if (s->bi_valid > (int)16 - len) { int val = (int)dist; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(dist) << s->bi_valid; s->bi_valid += len; }};
            }
        }
                                     ;
    } while (lx < s->last_lit);
    { int len = ltree[256].dl.len; if (s->bi_valid > (int)16 - len) { int val = (int)ltree[256].fc.code; s->bi_buf |= (ush)val << s->bi_valid; { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; }; s->bi_buf = (ush)val >> (16 - s->bi_valid); s->bi_valid += len - 16; } else { s->bi_buf |= (ush)(ltree[256].fc.code) << s->bi_valid; s->bi_valid += len; }};
}
static int detect_data_type(s)
    deflate_state *s;
{
    unsigned long black_mask = 0xf3ffc07fUL;
    int n;
    for (n = 0; n <= 31; n++, black_mask >>= 1)
        if ((black_mask & 1) && (s->dyn_ltree[n].fc.freq != 0))
            return 0;
    if (s->dyn_ltree[9].fc.freq != 0 || s->dyn_ltree[10].fc.freq != 0
            || s->dyn_ltree[13].fc.freq != 0)
        return 1;
    for (n = 32; n < 256; n++)
        if (s->dyn_ltree[n].fc.freq != 0)
            return 1;
    return 0;
}
static unsigned bi_reverse(code, len)
    unsigned code;
    int len;
{
    register unsigned res = 0;
    do {
        res |= code & 1;
        code >>= 1, res <<= 1;
    } while (--len > 0);
    return res >> 1;
}
static void bi_flush(s)
    deflate_state *s;
{
    if (s->bi_valid == 16) {
        { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; };
        s->bi_buf = 0;
        s->bi_valid = 0;
    } else if (s->bi_valid >= 8) {
        {s->pending_buf[s->pending++] = (Bytef)((Byte)s->bi_buf);};
        s->bi_buf >>= 8;
        s->bi_valid -= 8;
    }
}
static void bi_windup(s)
    deflate_state *s;
{
    if (s->bi_valid > 8) {
        { {s->pending_buf[s->pending++] = (Bytef)((uch)((s->bi_buf) & 0xff));}; {s->pending_buf[s->pending++] = (Bytef)((uch)((ush)(s->bi_buf) >> 8));}; };
    } else if (s->bi_valid > 0) {
        {s->pending_buf[s->pending++] = (Bytef)((Byte)s->bi_buf);};
    }
    s->bi_buf = 0;
    s->bi_valid = 0;
}
const char deflate_copyright[] =
   " deflate 1.2.11 Copyright 1995-2017 Jean-loup Gailly and Mark Adler ";
typedef enum {
    need_more,
    block_done,
    finish_started,
    finish_done
} block_state;
typedef block_state (*compress_func) (deflate_state *s, int flush);
static int deflateStateCheck (z_streamp strm);
static void slide_hash (deflate_state *s);
static void fill_window (deflate_state *s);
static block_state deflate_stored (deflate_state *s, int flush);
static block_state deflate_fast (deflate_state *s, int flush);
static block_state deflate_slow (deflate_state *s, int flush);
static block_state deflate_rle (deflate_state *s, int flush);
static block_state deflate_huff (deflate_state *s, int flush);
static void lm_init (deflate_state *s);
static void putShortMSB (deflate_state *s, uInt b);
static void flush_pending (z_streamp strm);
static unsigned read_buf (z_streamp strm, Bytef *buf, unsigned size);
static uInt longest_match (deflate_state *s, IPos cur_match);
typedef struct config_s {
   ush good_length;
   ush max_lazy;
   ush nice_length;
   ush max_chain;
   compress_func func;
} config;
static const config configuration_table[10] = {
        {0, 0, 0, 0, deflate_stored},
        {4, 4, 8, 4, deflate_fast},
        {4, 5, 16, 8, deflate_fast},
        {4, 6, 32, 32, deflate_fast},
        {4, 4, 16, 16, deflate_slow},
        {8, 16, 32, 32, deflate_slow},
        {8, 16, 128, 128, deflate_slow},
        {8, 32, 128, 256, deflate_slow},
        {32, 128, 258, 1024, deflate_slow},
        {32, 258, 258, 4096, deflate_slow}};
static void slide_hash(s)
    deflate_state *s;
{
    unsigned n, m;
    Posf *p;
    uInt wsize = s->w_size;
    n = s->hash_size;
    p = &s->head[n];
    do {
        m = *--p;
        *p = (Pos)(m >= wsize ? m - wsize : 0);
    } while (--n);
    n = wsize;
    p = &s->prev[n];
    do {
        m = *--p;
        *p = (Pos)(m >= wsize ? m - wsize : 0);
    } while (--n);
}
int deflateInit_(strm, level, version, stream_size)
    z_streamp strm;
    int level;
    const char *version;
    int stream_size;
{
    return deflateInit2_(strm, level, 8, 15, 8,
                         0, version, stream_size);
}
int deflateInit2_(strm, level, method, windowBits, memLevel, strategy,
                  version, stream_size)
    z_streamp strm;
    int level;
    int method;
    int windowBits;
    int memLevel;
    int strategy;
    const char *version;
    int stream_size;
{
    deflate_state *s;
    int wrap = 1;
    static const char my_version[] = "1.2.11-optipng";
    ushf *overlay;
    if (version == 0 || version[0] != my_version[0] ||
        stream_size != sizeof(z_stream)) {
        return (-6);
    }
    if (strm == 0) return (-2);
    strm->msg = 0;
    if (strm->zalloc == (alloc_func)0) {
        strm->zalloc = zcalloc;
        strm->opaque = (voidpf)0;
    }
    if (strm->zfree == (free_func)0)
        strm->zfree = zcfree;
    if (level == (-1)) level = 6;
    if (windowBits < 0) {
        wrap = 0;
        windowBits = -windowBits;
    }
    if (memLevel < 1 || memLevel > 9 || method != 8 ||
        windowBits < 8 || windowBits > 15 || level < 0 || level > 9 ||
        strategy < 0 || strategy > 4 || (windowBits == 8 && wrap != 1)) {
        return (-2);
    }
    if (windowBits == 8) windowBits = 9;
    s = (deflate_state *) (*((strm)->zalloc))((strm)->opaque, (1), (sizeof(deflate_state)));
    if (s == 0) return (-4);
    strm->state = (struct internal_state *)s;
    s->strm = strm;
    s->status = 42;
    s->wrap = wrap;
    s->gzhead = 0;
    s->w_bits = (uInt)windowBits;
    s->w_size = 1 << s->w_bits;
    s->w_mask = s->w_size - 1;
    s->hash_bits = (uInt)memLevel + 7;
    s->hash_size = 1 << s->hash_bits;
    s->hash_mask = s->hash_size - 1;
    s->hash_shift = ((s->hash_bits+3 -1)/3);
    s->window = (Bytef *) (*((strm)->zalloc))((strm)->opaque, (s->w_size), (2*sizeof(Byte)));
    s->prev = (Posf *) (*((strm)->zalloc))((strm)->opaque, (s->w_size), (sizeof(Pos)));
    s->head = (Posf *) (*((strm)->zalloc))((strm)->opaque, (s->hash_size), (sizeof(Pos)));
    s->high_water = 0;
    s->lit_bufsize = 1 << (memLevel + 6);
    overlay = (ushf *) (*((strm)->zalloc))((strm)->opaque, (s->lit_bufsize), (sizeof(ush)+2));
    s->pending_buf = (uchf *) overlay;
    s->pending_buf_size = (ulg)s->lit_bufsize * (sizeof(ush)+2L);
    if (s->window == 0 || s->prev == 0 || s->head == 0 ||
        s->pending_buf == 0) {
        s->status = 666;
        strm->msg = z_errmsg[2 -((-4))];
        deflateEnd (strm);
        return (-4);
    }
    s->d_buf = overlay + s->lit_bufsize/sizeof(ush);
    s->l_buf = s->pending_buf + (1+sizeof(ush))*s->lit_bufsize;
    s->level = level;
    s->strategy = strategy;
    s->method = (Byte)method;
    return deflateReset(strm);
}
static int deflateStateCheck (strm)
    z_streamp strm;
{
    deflate_state *s;
    if (strm == 0 ||
        strm->zalloc == (alloc_func)0 || strm->zfree == (free_func)0)
        return 1;
    s = strm->state;
    if (s == 0 || s->strm != strm || (s->status != 42 &&
                                           s->status != 69 &&
                                           s->status != 73 &&
                                           s->status != 91 &&
                                           s->status != 103 &&
                                           s->status != 113 &&
                                           s->status != 666))
        return 1;
    return 0;
}
int deflateSetDictionary (strm, dictionary, dictLength)
    z_streamp strm;
    const Bytef *dictionary;
    uInt dictLength;
{
    deflate_state *s;
    uInt str, n;
    int wrap;
    unsigned avail;
            unsigned char *next;
    if (deflateStateCheck(strm) || dictionary == 0)
        return (-2);
    s = strm->state;
    wrap = s->wrap;
    if (wrap == 2 || (wrap == 1 && s->status != 42) || s->lookahead)
        return (-2);
    if (wrap == 1)
        strm->adler = adler32(strm->adler, dictionary, dictLength);
    s->wrap = 0;
    if (dictLength >= s->w_size) {
        if (wrap == 0) {
            s->head[s->hash_size-1] = 0; memset((Bytef *)s->head, 0, (unsigned)(s->hash_size-1)*sizeof(*s->head));;
            s->strstart = 0;
            s->block_start = 0L;
            s->insert = 0;
        }
        dictionary += dictLength - s->w_size;
        dictLength = s->w_size;
    }
    avail = strm->avail_in;
    next = strm->next_in;
    strm->avail_in = dictLength;
    strm->next_in = ( Bytef *)dictionary;
    fill_window(s);
    while (s->lookahead >= 3) {
        str = s->strstart;
        n = s->lookahead - (3 -1);
        do {
            (s->ins_h = (((s->ins_h)<<s->hash_shift) ^ (s->window[str + 3 -1])) & s->hash_mask);
            s->prev[str & s->w_mask] = s->head[s->ins_h];
            s->head[s->ins_h] = (Pos)str;
            str++;
        } while (--n);
        s->strstart = str;
        s->lookahead = 3 -1;
        fill_window(s);
    }
    s->strstart += s->lookahead;
    s->block_start = (long)s->strstart;
    s->insert = s->lookahead;
    s->lookahead = 0;
    s->match_length = s->prev_length = 3 -1;
    s->match_available = 0;
    strm->next_in = next;
    strm->avail_in = avail;
    s->wrap = wrap;
    return 0;
}
int deflateGetDictionary (strm, dictionary, dictLength)
    z_streamp strm;
    Bytef *dictionary;
    uInt *dictLength;
{
    deflate_state *s;
    uInt len;
    if (deflateStateCheck(strm))
        return (-2);
    s = strm->state;
    len = s->strstart + s->lookahead;
    if (len > s->w_size)
        len = s->w_size;
    if (dictionary != 0 && len)
        memcpy(dictionary, s->window + s->strstart + s->lookahead - len, len);
    if (dictLength != 0)
        *dictLength = len;
    return 0;
}
int deflateResetKeep (strm)
    z_streamp strm;
{
    deflate_state *s;
    if (deflateStateCheck(strm)) {
        return (-2);
    }
    strm->total_in = strm->total_out = 0;
    strm->msg = 0;
    strm->data_type = 2;
    s = (deflate_state *)strm->state;
    s->pending = 0;
    s->pending_out = s->pending_buf;
    if (s->wrap < 0) {
        s->wrap = -s->wrap;
    }
    s->status =
        s->wrap ? 42 : 113;
    strm->adler =
        adler32(0L, 0, 0);
    s->last_flush = 0;
    _tr_init(s);
    return 0;
}
int deflateReset (strm)
    z_streamp strm;
{
    int ret;
    ret = deflateResetKeep(strm);
    if (ret == 0)
        lm_init(strm->state);
    return ret;
}
int deflateSetHeader (strm, head)
    z_streamp strm;
    gz_headerp head;
{
    if (deflateStateCheck(strm) || strm->state->wrap != 2)
        return (-2);
    strm->state->gzhead = head;
    return 0;
}
int deflatePending (strm, pending, bits)
    unsigned *pending;
    int *bits;
    z_streamp strm;
{
    if (deflateStateCheck(strm)) return (-2);
    if (pending != 0)
        *pending = strm->state->pending;
    if (bits != 0)
        *bits = strm->state->bi_valid;
    return 0;
}
int deflatePrime (strm, bits, value)
    z_streamp strm;
    int bits;
    int value;
{
    deflate_state *s;
    int put;
    if (deflateStateCheck(strm)) return (-2);
    s = strm->state;
    if ((Bytef *)(s->d_buf) < s->pending_out + ((16 + 7) >> 3))
        return (-5);
    do {
        put = 16 - s->bi_valid;
        if (put > bits)
            put = bits;
        s->bi_buf |= (ush)((value & ((1 << put) - 1)) << s->bi_valid);
        s->bi_valid += put;
        _tr_flush_bits(s);
        value >>= put;
        bits -= put;
    } while (bits);
    return 0;
}
int deflateParams(strm, level, strategy)
    z_streamp strm;
    int level;
    int strategy;
{
    deflate_state *s;
    compress_func func;
    if (deflateStateCheck(strm)) return (-2);
    s = strm->state;
    if (level == (-1)) level = 6;
    if (level < 0 || level > 9 || strategy < 0 || strategy > 4) {
        return (-2);
    }
    func = configuration_table[s->level].func;
    if ((strategy != s->strategy || func != configuration_table[level].func) &&
        s->high_water) {
        int err = deflate(strm, 5);
        if (err == (-2))
            return err;
        if (strm->avail_out == 0)
            return (-5);
    }
    if (s->level != level) {
        if (s->level == 0 && s->matches != 0) {
            if (s->matches == 1)
                slide_hash(s);
            else
                s->head[s->hash_size-1] = 0; memset((Bytef *)s->head, 0, (unsigned)(s->hash_size-1)*sizeof(*s->head));;
            s->matches = 0;
        }
        s->level = level;
        s->max_lazy_match = configuration_table[level].max_lazy;
        s->good_match = configuration_table[level].good_length;
        s->nice_match = configuration_table[level].nice_length;
        s->max_chain_length = configuration_table[level].max_chain;
    }
    s->strategy = strategy;
    return 0;
}
int deflateTune(strm, good_length, max_lazy, nice_length, max_chain)
    z_streamp strm;
    int good_length;
    int max_lazy;
    int nice_length;
    int max_chain;
{
    deflate_state *s;
    if (deflateStateCheck(strm)) return (-2);
    s = strm->state;
    s->good_match = (uInt)good_length;
    s->max_lazy_match = (uInt)max_lazy;
    s->nice_match = nice_length;
    s->max_chain_length = (uInt)max_chain;
    return 0;
}
uLong deflateBound(strm, sourceLen)
    z_streamp strm;
    uLong sourceLen;
{
    deflate_state *s;
    uLong complen, wraplen;
    complen = sourceLen +
              ((sourceLen + 7) >> 3) + ((sourceLen + 63) >> 6) + 5;
    if (deflateStateCheck(strm))
        return complen + 6;
    s = strm->state;
    switch (s->wrap) {
    case 0:
        wraplen = 0;
        break;
    case 1:
        wraplen = 6 + (s->strstart ? 4 : 0);
        break;
    default:
        wraplen = 6;
    }
    if (s->w_bits != 15 || s->hash_bits != 8 + 7)
        return complen + wraplen;
    return sourceLen + (sourceLen >> 12) + (sourceLen >> 14) +
           (sourceLen >> 25) + 13 - 6 + wraplen;
}
static void putShortMSB (s, b)
    deflate_state *s;
    uInt b;
{
    {s->pending_buf[s->pending++] = (Bytef)((Byte)(b >> 8));};
    {s->pending_buf[s->pending++] = (Bytef)((Byte)(b & 0xff));};
}
static void flush_pending(strm)
    z_streamp strm;
{
    unsigned len;
    deflate_state *s = strm->state;
    _tr_flush_bits(s);
    len = s->pending;
    if (len > strm->avail_out) len = strm->avail_out;
    if (len == 0) return;
    memcpy(strm->next_out, s->pending_out, len);
    strm->next_out += len;
    s->pending_out += len;
    strm->total_out += len;
    strm->avail_out -= len;
    s->pending -= len;
    if (s->pending == 0) {
        s->pending_out = s->pending_buf;
    }
}
int deflate (strm, flush)
    z_streamp strm;
    int flush;
{
    int old_flush;
    deflate_state *s;
    if (deflateStateCheck(strm) || flush > 5 || flush < 0) {
        return (-2);
    }
    s = strm->state;
    if (strm->next_out == 0 ||
        (strm->avail_in != 0 && strm->next_in == 0) ||
        (s->status == 666 && flush != 4)) {
        return (strm->msg = z_errmsg[2 -((-2))], ((-2)));
    }
    if (strm->avail_out == 0) return (strm->msg = z_errmsg[2 -((-5))], ((-5)));
    old_flush = s->last_flush;
    s->last_flush = flush;
    if (s->pending != 0) {
        flush_pending(strm);
        if (strm->avail_out == 0) {
            s->last_flush = -1;
            return 0;
        }
    } else if (strm->avail_in == 0 && (((flush) * 2) - ((flush) > 4 ? 9 : 0)) <= (((old_flush) * 2) - ((old_flush) > 4 ? 9 : 0)) &&
               flush != 4) {
        return (strm->msg = z_errmsg[2 -((-5))], ((-5)));
    }
    if (s->status == 666 && strm->avail_in != 0) {
        return (strm->msg = z_errmsg[2 -((-5))], ((-5)));
    }
    if (s->status == 42) {
        uInt header = (8 + ((s->w_bits-8)<<4)) << 8;
        uInt level_flags;
        if (s->strategy >= 2 || s->level < 2)
            level_flags = 0;
        else if (s->level < 6)
            level_flags = 1;
        else if (s->level == 6)
            level_flags = 2;
        else
            level_flags = 3;
        header |= (level_flags << 6);
        if (s->strstart != 0) header |= 0x20;
        header += 31 - (header % 31);
        putShortMSB(s, header);
        if (s->strstart != 0) {
            putShortMSB(s, (uInt)(strm->adler >> 16));
            putShortMSB(s, (uInt)(strm->adler & 0xffff));
        }
        strm->adler = adler32(0L, 0, 0);
        s->status = 113;
        flush_pending(strm);
        if (s->pending != 0) {
            s->last_flush = -1;
            return 0;
        }
    }
    if (strm->avail_in != 0 || s->lookahead != 0 ||
        (flush != 0 && s->status != 666)) {
        block_state bstate;
        bstate = s->level == 0 ? deflate_stored(s, flush) :
                 s->strategy == 2 ? deflate_huff(s, flush) :
                 s->strategy == 3 ? deflate_rle(s, flush) :
                 (*(configuration_table[s->level].func))(s, flush);
        if (bstate == finish_started || bstate == finish_done) {
            s->status = 666;
        }
        if (bstate == need_more || bstate == finish_started) {
            if (strm->avail_out == 0) {
                s->last_flush = -1;
            }
            return 0;
        }
        if (bstate == block_done) {
            if (flush == 1) {
                _tr_align(s);
            } else if (flush != 5) {
                _tr_stored_block(s, (char*)0, 0L, 0);
                if (flush == 3) {
                    s->head[s->hash_size-1] = 0; memset((Bytef *)s->head, 0, (unsigned)(s->hash_size-1)*sizeof(*s->head));;
                    if (s->lookahead == 0) {
                        s->strstart = 0;
                        s->block_start = 0L;
                        s->insert = 0;
                    }
                }
            }
            flush_pending(strm);
            if (strm->avail_out == 0) {
              s->last_flush = -1;
              return 0;
            }
        }
    }
    if (flush != 4) return 0;
    if (s->wrap <= 0) return 1;
    {
        putShortMSB(s, (uInt)(strm->adler >> 16));
        putShortMSB(s, (uInt)(strm->adler & 0xffff));
    }
    flush_pending(strm);
    if (s->wrap > 0) s->wrap = -s->wrap;
    return s->pending != 0 ? 0 : 1;
}
int deflateEnd (strm)
    z_streamp strm;
{
    int status;
    if (deflateStateCheck(strm)) return (-2);
    status = strm->state->status;
    {if (strm->state->pending_buf) (*((strm)->zfree))((strm)->opaque, (voidpf)(strm->state->pending_buf));};
    {if (strm->state->head) (*((strm)->zfree))((strm)->opaque, (voidpf)(strm->state->head));};
    {if (strm->state->prev) (*((strm)->zfree))((strm)->opaque, (voidpf)(strm->state->prev));};
    {if (strm->state->window) (*((strm)->zfree))((strm)->opaque, (voidpf)(strm->state->window));};
    (*((strm)->zfree))((strm)->opaque, (voidpf)(strm->state));
    strm->state = 0;
    return status == 113 ? (-3) : 0;
}
int deflateCopy (dest, source)
    z_streamp dest;
    z_streamp source;
{
    deflate_state *ds;
    deflate_state *ss;
    ushf *overlay;
    if (deflateStateCheck(source) || dest == 0) {
        return (-2);
    }
    ss = source->state;
    memcpy((voidpf)dest, (voidpf)source, sizeof(z_stream));
    ds = (deflate_state *) (*((dest)->zalloc))((dest)->opaque, (1), (sizeof(deflate_state)));
    if (ds == 0) return (-4);
    dest->state = (struct internal_state *) ds;
    memcpy((voidpf)ds, (voidpf)ss, sizeof(deflate_state));
    ds->strm = dest;
    ds->window = (Bytef *) (*((dest)->zalloc))((dest)->opaque, (ds->w_size), (2*sizeof(Byte)));
    ds->prev = (Posf *) (*((dest)->zalloc))((dest)->opaque, (ds->w_size), (sizeof(Pos)));
    ds->head = (Posf *) (*((dest)->zalloc))((dest)->opaque, (ds->hash_size), (sizeof(Pos)));
    overlay = (ushf *) (*((dest)->zalloc))((dest)->opaque, (ds->lit_bufsize), (sizeof(ush)+2));
    ds->pending_buf = (uchf *) overlay;
    if (ds->window == 0 || ds->prev == 0 || ds->head == 0 ||
        ds->pending_buf == 0) {
        deflateEnd (dest);
        return (-4);
    }
    memcpy(ds->window, ss->window, ds->w_size * 2 * sizeof(Byte));
    memcpy((voidpf)ds->prev, (voidpf)ss->prev, ds->w_size * sizeof(Pos));
    memcpy((voidpf)ds->head, (voidpf)ss->head, ds->hash_size * sizeof(Pos));
    memcpy(ds->pending_buf, ss->pending_buf, (uInt)ds->pending_buf_size);
    ds->pending_out = ds->pending_buf + (ss->pending_out - ss->pending_buf);
    ds->d_buf = overlay + ds->lit_bufsize/sizeof(ush);
    ds->l_buf = ds->pending_buf + (1+sizeof(ush))*ds->lit_bufsize;
    ds->l_desc.dyn_tree = ds->dyn_ltree;
    ds->d_desc.dyn_tree = ds->dyn_dtree;
    ds->bl_desc.dyn_tree = ds->bl_tree;
    return 0;
}
static unsigned read_buf(strm, buf, size)
    z_streamp strm;
    Bytef *buf;
    unsigned size;
{
    unsigned len = strm->avail_in;
    if (len > size) len = size;
    if (len == 0) return 0;
    strm->avail_in -= len;
    memcpy(buf, strm->next_in, len);
    if (strm->state->wrap == 1) {
        strm->adler = adler32(strm->adler, buf, len);
    }
    strm->next_in += len;
    strm->total_in += len;
    return len;
}
static void lm_init (s)
    deflate_state *s;
{
    s->window_size = (ulg)2L*s->w_size;
    s->head[s->hash_size-1] = 0; memset((Bytef *)s->head, 0, (unsigned)(s->hash_size-1)*sizeof(*s->head));;
    s->max_lazy_match = configuration_table[s->level].max_lazy;
    s->good_match = configuration_table[s->level].good_length;
    s->nice_match = configuration_table[s->level].nice_length;
    s->max_chain_length = configuration_table[s->level].max_chain;
    s->strstart = 0;
    s->block_start = 0L;
    s->lookahead = 0;
    s->insert = 0;
    s->match_length = s->prev_length = 3 -1;
    s->match_available = 0;
    s->ins_h = 0;
}
static uInt longest_match(s, cur_match)
    deflate_state *s;
    IPos cur_match;
{
    unsigned chain_length = s->max_chain_length;
    register Bytef *scan = s->window + s->strstart;
    register Bytef *match;
    register int len;
    int best_len = (int)s->prev_length;
    int nice_match = s->nice_match;
    IPos limit = s->strstart > (IPos)((s)->w_size-(258 +3 +1)) ?
        s->strstart - (IPos)((s)->w_size-(258 +3 +1)) : 0;
    Posf *prev = s->prev;
    uInt wmask = s->w_mask;
    register Bytef *strend = s->window + s->strstart + 258;
    register Byte scan_end1 = scan[best_len-1];
    register Byte scan_end = scan[best_len];
                                                                    ;
    if (s->prev_length >= s->good_match) {
        chain_length >>= 2;
    }
    if ((uInt)nice_match > s->lookahead) nice_match = (int)s->lookahead;
                                                                              ;
    do {
                                                    ;
        match = s->window + cur_match;
        if (match[best_len] != scan_end ||
            match[best_len-1] != scan_end1 ||
            *match != *scan ||
            *++match != scan[1]) continue;
        scan += 2, match++;
                                            ;
        do {
        } while (*++scan == *++match && *++scan == *++match &&
                 *++scan == *++match && *++scan == *++match &&
                 *++scan == *++match && *++scan == *++match &&
                 *++scan == *++match && *++scan == *++match &&
                 scan < strend);
                                                                           ;
        len = 258 - (int)(strend - scan);
        scan = strend - 258;
        if (len > best_len) {
            s->match_start = cur_match;
            best_len = len;
            if (len >= nice_match) break;
            scan_end1 = scan[best_len-1];
            scan_end = scan[best_len];
        }
    } while ((cur_match = prev[cur_match & wmask]) > limit
             && --chain_length != 0);
    if ((uInt)best_len <= s->lookahead) return (uInt)best_len;
    return s->lookahead;
}
static void fill_window(s)
    deflate_state *s;
{
    unsigned n;
    unsigned more;
    uInt wsize = s->w_size;
                                                                    ;
    do {
        more = (unsigned)(s->window_size -(ulg)s->lookahead -(ulg)s->strstart);
        if (sizeof(int) <= 2) {
            if (more == 0 && s->strstart == 0 && s->lookahead == 0) {
                more = wsize;
            } else if (more == (unsigned)(-1)) {
                more--;
            }
        }
        if (s->strstart >= wsize+((s)->w_size-(258 +3 +1))) {
            memcpy(s->window, s->window+wsize, (unsigned)wsize - more);
            s->match_start -= wsize;
            s->strstart -= wsize;
            s->block_start -= (long) wsize;
            slide_hash(s);
            more += wsize;
        }
        if (s->strm->avail_in == 0) break;
                                     ;
        n = read_buf(s->strm, s->window + s->strstart + s->lookahead, more);
        s->lookahead += n;
        if (s->lookahead + s->insert >= 3) {
            uInt str = s->strstart - s->insert;
            s->ins_h = s->window[str];
            (s->ins_h = (((s->ins_h)<<s->hash_shift) ^ (s->window[str + 1])) & s->hash_mask);
            while (s->insert) {
                (s->ins_h = (((s->ins_h)<<s->hash_shift) ^ (s->window[str + 3 -1])) & s->hash_mask);
                s->prev[str & s->w_mask] = s->head[s->ins_h];
                s->head[s->ins_h] = (Pos)str;
                str++;
                s->insert--;
                if (s->lookahead + s->insert < 3)
                    break;
            }
        }
    } while (s->lookahead < (258 +3 +1) && s->strm->avail_in != 0);
    if (s->high_water < s->window_size) {
        ulg curr = s->strstart + (ulg)(s->lookahead);
        ulg init;
        if (s->high_water < curr) {
            init = s->window_size - curr;
            if (init > 258)
                init = 258;
            memset(s->window + curr, 0, (unsigned)init);
            s->high_water = curr + init;
        }
        else if (s->high_water < (ulg)curr + 258) {
            init = (ulg)curr + 258 - s->high_water;
            if (init > s->window_size - s->high_water)
                init = s->window_size - s->high_water;
            memset(s->window + s->high_water, 0, (unsigned)init);
            s->high_water += init;
        }
    }
                                        ;
}
static block_state deflate_stored(s, flush)
    deflate_state *s;
    int flush;
{
    unsigned min_block = ((s->pending_buf_size - 5) > (s->w_size) ? (s->w_size) : (s->pending_buf_size - 5));
    unsigned len, left, have, last = 0;
    unsigned used = s->strm->avail_in;
    do {
        len = 65535;
        have = (s->bi_valid + 42) >> 3;
        if (s->strm->avail_out < have)
            break;
        have = s->strm->avail_out - have;
        left = s->strstart - s->block_start;
        if (len > (ulg)left + s->strm->avail_in)
            len = left + s->strm->avail_in;
        if (len > have)
            len = have;
        if (len < min_block && ((len == 0 && flush != 4) ||
                                flush == 0 ||
                                len != left + s->strm->avail_in))
            break;
        last = flush == 4 && len == left + s->strm->avail_in ? 1 : 0;
        _tr_stored_block(s, (char *)0, 0L, last);
        s->pending_buf[s->pending - 4] = len;
        s->pending_buf[s->pending - 3] = len >> 8;
        s->pending_buf[s->pending - 2] = ~len;
        s->pending_buf[s->pending - 1] = ~len >> 8;
        flush_pending(s->strm);
        if (left) {
            if (left > len)
                left = len;
            memcpy(s->strm->next_out, s->window + s->block_start, left);
            s->strm->next_out += left;
            s->strm->avail_out -= left;
            s->strm->total_out += left;
            s->block_start += left;
            len -= left;
        }
        if (len) {
            read_buf(s->strm, s->strm->next_out, len);
            s->strm->next_out += len;
            s->strm->avail_out -= len;
            s->strm->total_out += len;
        }
    } while (last == 0);
    used -= s->strm->avail_in;
    if (used) {
        if (used >= s->w_size) {
            s->matches = 2;
            memcpy(s->window, s->strm->next_in - s->w_size, s->w_size);
            s->strstart = s->w_size;
        }
        else {
            if (s->window_size - s->strstart <= used) {
                s->strstart -= s->w_size;
                memcpy(s->window, s->window + s->w_size, s->strstart);
                if (s->matches < 2)
                    s->matches++;
            }
            memcpy(s->window + s->strstart, s->strm->next_in - used, used);
            s->strstart += used;
        }
        s->block_start = s->strstart;
        s->insert += ((used) > (s->w_size - s->insert) ? (s->w_size - s->insert) : (used));
    }
    if (s->high_water < s->strstart)
        s->high_water = s->strstart;
    if (last)
        return finish_done;
    if (flush != 0 && flush != 4 &&
        s->strm->avail_in == 0 && (long)s->strstart == s->block_start)
        return block_done;
    have = s->window_size - s->strstart - 1;
    if (s->strm->avail_in > have && s->block_start >= (long)s->w_size) {
        s->block_start -= s->w_size;
        s->strstart -= s->w_size;
        memcpy(s->window, s->window + s->w_size, s->strstart);
        if (s->matches < 2)
            s->matches++;
        have += s->w_size;
    }
    if (have > s->strm->avail_in)
        have = s->strm->avail_in;
    if (have) {
        read_buf(s->strm, s->window + s->strstart, have);
        s->strstart += have;
    }
    if (s->high_water < s->strstart)
        s->high_water = s->strstart;
    have = (s->bi_valid + 42) >> 3;
    have = ((s->pending_buf_size - have) > (65535) ? (65535) : (s->pending_buf_size - have));
    min_block = ((have) > (s->w_size) ? (s->w_size) : (have));
    left = s->strstart - s->block_start;
    if (left >= min_block ||
        ((left || flush == 4) && flush != 0 &&
         s->strm->avail_in == 0 && left <= have)) {
        len = ((left) > (have) ? (have) : (left));
        last = flush == 4 && s->strm->avail_in == 0 &&
               len == left ? 1 : 0;
        _tr_stored_block(s, (charf *)s->window + s->block_start, len, last);
        s->block_start += len;
        flush_pending(s->strm);
    }
    return last ? finish_started : need_more;
}
static block_state deflate_fast(s, flush)
    deflate_state *s;
    int flush;
{
    IPos hash_head;
    int bflush;
    for (;;) {
        if (s->lookahead < (258 +3 +1)) {
            fill_window(s);
            if (s->lookahead < (258 +3 +1) && flush == 0) {
                return need_more;
            }
            if (s->lookahead == 0) break;
        }
        hash_head = 0;
        if (s->lookahead >= 3) {
            ((s->ins_h = (((s->ins_h)<<s->hash_shift) ^ (s->window[(s->strstart) + (3 -1)])) & s->hash_mask), hash_head = s->prev[(s->strstart) & s->w_mask] = s->head[s->ins_h], s->head[s->ins_h] = (Pos)(s->strstart));
        }
        if (hash_head != 0 && s->strstart - hash_head <= ((s)->w_size-(258 +3 +1))) {
            s->match_length = longest_match (s, hash_head);
        }
        if (s->match_length >= 3) {
                                                                        ;
            { uch len = (uch)(s->match_length - 3); ush dist = (ush)(s->strstart - s->match_start); s->d_buf[s->last_lit] = dist; s->l_buf[s->last_lit++] = len; dist--; s->dyn_ltree[_length_code[len]+256 +1].fc.freq++; s->dyn_dtree[((dist) < 256 ? _dist_code[dist] : _dist_code[256+((dist)>>7)])].fc.freq++; bflush = (s->last_lit == s->lit_bufsize-1); };
            s->lookahead -= s->match_length;
            if (s->match_length <= s->max_lazy_match &&
                s->lookahead >= 3) {
                s->match_length--;
                do {
                    s->strstart++;
                    ((s->ins_h = (((s->ins_h)<<s->hash_shift) ^ (s->window[(s->strstart) + (3 -1)])) & s->hash_mask), hash_head = s->prev[(s->strstart) & s->w_mask] = s->head[s->ins_h], s->head[s->ins_h] = (Pos)(s->strstart));
                } while (--s->match_length != 0);
                s->strstart++;
            } else
            {
                s->strstart += s->match_length;
                s->match_length = 0;
                s->ins_h = s->window[s->strstart];
                (s->ins_h = (((s->ins_h)<<s->hash_shift) ^ (s->window[s->strstart+1])) & s->hash_mask);
            }
        } else {
                                                          ;
            { uch cc = (s->window[s->strstart]); s->d_buf[s->last_lit] = 0; s->l_buf[s->last_lit++] = cc; s->dyn_ltree[cc].fc.freq++; bflush = (s->last_lit == s->lit_bufsize-1); };
            s->lookahead--;
            s->strstart++;
        }
        if (bflush) { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (0) ? finish_started : need_more; };
    }
    s->insert = s->strstart < 3 -1 ? s->strstart : 3 -1;
    if (flush == 4) {
        { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (1)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (1) ? finish_started : need_more; };
        return finish_done;
    }
    if (s->last_lit)
        { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (0) ? finish_started : need_more; };
    return block_done;
}
static block_state deflate_slow(s, flush)
    deflate_state *s;
    int flush;
{
    IPos hash_head;
    int bflush;
    for (;;) {
        if (s->lookahead < (258 +3 +1)) {
            fill_window(s);
            if (s->lookahead < (258 +3 +1) && flush == 0) {
                return need_more;
            }
            if (s->lookahead == 0) break;
        }
        hash_head = 0;
        if (s->lookahead >= 3) {
            ((s->ins_h = (((s->ins_h)<<s->hash_shift) ^ (s->window[(s->strstart) + (3 -1)])) & s->hash_mask), hash_head = s->prev[(s->strstart) & s->w_mask] = s->head[s->ins_h], s->head[s->ins_h] = (Pos)(s->strstart));
        }
        s->prev_length = s->match_length, s->prev_match = s->match_start;
        s->match_length = 3 -1;
        if (hash_head != 0 && s->prev_length < s->max_lazy_match &&
            s->strstart - hash_head <= ((s)->w_size-(258 +3 +1))) {
            s->match_length = longest_match (s, hash_head);
            if (s->match_length <= 5 && (s->strategy == 1
                )) {
                s->match_length = 3 -1;
            }
        }
        if (s->prev_length >= 3 && s->match_length <= s->prev_length) {
            uInt max_insert = s->strstart + s->lookahead - 3;
                                                                        ;
            { uch len = (uch)(s->prev_length - 3); ush dist = (ush)(s->strstart -1 - s->prev_match); s->d_buf[s->last_lit] = dist; s->l_buf[s->last_lit++] = len; dist--; s->dyn_ltree[_length_code[len]+256 +1].fc.freq++; s->dyn_dtree[((dist) < 256 ? _dist_code[dist] : _dist_code[256+((dist)>>7)])].fc.freq++; bflush = (s->last_lit == s->lit_bufsize-1); };
            s->lookahead -= s->prev_length-1;
            s->prev_length -= 2;
            do {
                if (++s->strstart <= max_insert) {
                    ((s->ins_h = (((s->ins_h)<<s->hash_shift) ^ (s->window[(s->strstart) + (3 -1)])) & s->hash_mask), hash_head = s->prev[(s->strstart) & s->w_mask] = s->head[s->ins_h], s->head[s->ins_h] = (Pos)(s->strstart));
                }
            } while (--s->prev_length != 0);
            s->match_available = 0;
            s->match_length = 3 -1;
            s->strstart++;
            if (bflush) { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (0) ? finish_started : need_more; };
        } else if (s->match_available) {
                                                            ;
            { uch cc = (s->window[s->strstart-1]); s->d_buf[s->last_lit] = 0; s->l_buf[s->last_lit++] = cc; s->dyn_ltree[cc].fc.freq++; bflush = (s->last_lit == s->lit_bufsize-1); };
            if (bflush) {
                { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; };
            }
            s->strstart++;
            s->lookahead--;
            if (s->strm->avail_out == 0) return need_more;
        } else {
            s->match_available = 1;
            s->strstart++;
            s->lookahead--;
        }
    }
                                             ;
    if (s->match_available) {
                                                        ;
        { uch cc = (s->window[s->strstart-1]); s->d_buf[s->last_lit] = 0; s->l_buf[s->last_lit++] = cc; s->dyn_ltree[cc].fc.freq++; bflush = (s->last_lit == s->lit_bufsize-1); };
        s->match_available = 0;
    }
    s->insert = s->strstart < 3 -1 ? s->strstart : 3 -1;
    if (flush == 4) {
        { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (1)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (1) ? finish_started : need_more; };
        return finish_done;
    }
    if (s->last_lit)
        { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (0) ? finish_started : need_more; };
    return block_done;
}
static block_state deflate_rle(s, flush)
    deflate_state *s;
    int flush;
{
    int bflush;
    uInt prev;
    Bytef *scan, *strend;
    for (;;) {
        if (s->lookahead <= 258) {
            fill_window(s);
            if (s->lookahead <= 258 && flush == 0) {
                return need_more;
            }
            if (s->lookahead == 0) break;
        }
        s->match_length = 0;
        if (s->lookahead >= 3 && s->strstart > 0) {
            scan = s->window + s->strstart - 1;
            prev = *scan;
            if (prev == *++scan && prev == *++scan && prev == *++scan) {
                strend = s->window + s->strstart + 258;
                do {
                } while (prev == *++scan && prev == *++scan &&
                         prev == *++scan && prev == *++scan &&
                         prev == *++scan && prev == *++scan &&
                         prev == *++scan && prev == *++scan &&
                         scan < strend);
                s->match_length = 258 - (uInt)(strend - scan);
                if (s->match_length > s->lookahead)
                    s->match_length = s->lookahead;
            }
                                                                           ;
        }
        if (s->match_length >= 3) {
                                                                         ;
            { uch len = (uch)(s->match_length - 3); ush dist = (ush)(1); s->d_buf[s->last_lit] = dist; s->l_buf[s->last_lit++] = len; dist--; s->dyn_ltree[_length_code[len]+256 +1].fc.freq++; s->dyn_dtree[((dist) < 256 ? _dist_code[dist] : _dist_code[256+((dist)>>7)])].fc.freq++; bflush = (s->last_lit == s->lit_bufsize-1); };
            s->lookahead -= s->match_length;
            s->strstart += s->match_length;
            s->match_length = 0;
        } else {
                                                          ;
            { uch cc = (s->window[s->strstart]); s->d_buf[s->last_lit] = 0; s->l_buf[s->last_lit++] = cc; s->dyn_ltree[cc].fc.freq++; bflush = (s->last_lit == s->lit_bufsize-1); };
            s->lookahead--;
            s->strstart++;
        }
        if (bflush) { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (0) ? finish_started : need_more; };
    }
    s->insert = 0;
    if (flush == 4) {
        { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (1)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (1) ? finish_started : need_more; };
        return finish_done;
    }
    if (s->last_lit)
        { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (0) ? finish_started : need_more; };
    return block_done;
}
static block_state deflate_huff(s, flush)
    deflate_state *s;
    int flush;
{
    int bflush;
    for (;;) {
        if (s->lookahead == 0) {
            fill_window(s);
            if (s->lookahead == 0) {
                if (flush == 0)
                    return need_more;
                break;
            }
        }
        s->match_length = 0;
                                                      ;
        { uch cc = (s->window[s->strstart]); s->d_buf[s->last_lit] = 0; s->l_buf[s->last_lit++] = cc; s->dyn_ltree[cc].fc.freq++; bflush = (s->last_lit == s->lit_bufsize-1); };
        s->lookahead--;
        s->strstart++;
        if (bflush) { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (0) ? finish_started : need_more; };
    }
    s->insert = 0;
    if (flush == 4) {
        { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (1)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (1) ? finish_started : need_more; };
        return finish_done;
    }
    if (s->last_lit)
        { { _tr_flush_block(s, (s->block_start >= 0L ? (charf *)&s->window[(unsigned)s->block_start] : (charf *)0), (ulg)((long)s->strstart - s->block_start), (0)); s->block_start = s->strstart; flush_pending(s->strm); ; }; if (s->strm->avail_out == 0) return (0) ? finish_started : need_more; };
    return block_done;
}
int compress2 (dest, destLen, source, sourceLen, level)
    Bytef *dest;
    uLongf *destLen;
    const Bytef *source;
    uLong sourceLen;
    int level;
{
    z_stream stream;
    int err;
    const uInt max = (uInt)-1;
    uLong left;
    left = *destLen;
    *destLen = 0;
    stream.zalloc = (alloc_func)0;
    stream.zfree = (free_func)0;
    stream.opaque = (voidpf)0;
    err = deflateInit_((&stream), (level), "1.2.11-optipng", (int)sizeof(z_stream));
    if (err != 0) return err;
    stream.next_out = dest;
    stream.avail_out = 0;
    stream.next_in = ( Bytef *)source;
    stream.avail_in = 0;
    do {
        if (stream.avail_out == 0) {
            stream.avail_out = left > (uLong)max ? max : (uInt)left;
            left -= stream.avail_out;
        }
        if (stream.avail_in == 0) {
            stream.avail_in = sourceLen > (uLong)max ? max : (uInt)sourceLen;
            sourceLen -= stream.avail_in;
        }
        err = deflate(&stream, sourceLen ? 0 : 4);
    } while (err == 0);
    *destLen = stream.total_out;
    deflateEnd(&stream);
    return err == 1 ? 0 : err;
}
int compress (dest, destLen, source, sourceLen)
    Bytef *dest;
    uLongf *destLen;
    const Bytef *source;
    uLong sourceLen;
{
    return compress2(dest, destLen, source, sourceLen, (-1));
}
uLong compressBound (sourceLen)
    uLong sourceLen;
{
    return sourceLen + (sourceLen >> 12) + (sourceLen >> 14) +
           (sourceLen >> 25) + 13;
}
int zlib_compress_observe(int8_t packet[512])
{
    uLong source_length = ((unsigned char)packet[0]) % 128;
    uLongf destination_length = 383;
    int result = compress2(
        (Bytef *)&packet[129],
        &destination_length,
        (const Bytef *)&packet[1],
        source_length,
        6);
    memcpy(&packet[0], &destination_length, sizeof(destination_length));
    return result;
}
