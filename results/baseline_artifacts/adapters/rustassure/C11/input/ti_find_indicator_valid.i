extern void __assert_fail (const char *__assertion, const char *__file,
      unsigned int __line, const char *__function)
     __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern void __assert_perror_fail (int __errnum, const char *__file,
      unsigned int __line, const char *__function)
     __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
extern void __assert (const char *__assertion, const char *__file, int __line)
     __attribute__ ((__nothrow__ )) __attribute__ ((__noreturn__));
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
typedef float _Float32;
typedef double _Float64;
typedef double _Float32x;
typedef long double _Float64x;
typedef float float_t;
typedef double double_t;
extern int __fpclassify (double __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
extern int __signbit (double __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
extern int __isinf (double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __finite (double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __isnan (double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __iseqsig (double __x, double __y) __attribute__ ((__nothrow__ ));
extern int __issignaling (double __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
 extern double acos (double __x) __attribute__ ((__nothrow__ )); extern double __acos (double __x) __attribute__ ((__nothrow__ ));
 extern double asin (double __x) __attribute__ ((__nothrow__ )); extern double __asin (double __x) __attribute__ ((__nothrow__ ));
 extern double atan (double __x) __attribute__ ((__nothrow__ )); extern double __atan (double __x) __attribute__ ((__nothrow__ ));
 extern double atan2 (double __y, double __x) __attribute__ ((__nothrow__ )); extern double __atan2 (double __y, double __x) __attribute__ ((__nothrow__ ));
 extern double cos (double __x) __attribute__ ((__nothrow__ )); extern double __cos (double __x) __attribute__ ((__nothrow__ ));
 extern double sin (double __x) __attribute__ ((__nothrow__ )); extern double __sin (double __x) __attribute__ ((__nothrow__ ));
 extern double tan (double __x) __attribute__ ((__nothrow__ )); extern double __tan (double __x) __attribute__ ((__nothrow__ ));
 extern double cosh (double __x) __attribute__ ((__nothrow__ )); extern double __cosh (double __x) __attribute__ ((__nothrow__ ));
 extern double sinh (double __x) __attribute__ ((__nothrow__ )); extern double __sinh (double __x) __attribute__ ((__nothrow__ ));
 extern double tanh (double __x) __attribute__ ((__nothrow__ )); extern double __tanh (double __x) __attribute__ ((__nothrow__ ));
 extern double acosh (double __x) __attribute__ ((__nothrow__ )); extern double __acosh (double __x) __attribute__ ((__nothrow__ ));
 extern double asinh (double __x) __attribute__ ((__nothrow__ )); extern double __asinh (double __x) __attribute__ ((__nothrow__ ));
 extern double atanh (double __x) __attribute__ ((__nothrow__ )); extern double __atanh (double __x) __attribute__ ((__nothrow__ ));
 extern double exp (double __x) __attribute__ ((__nothrow__ )); extern double __exp (double __x) __attribute__ ((__nothrow__ ));
extern double frexp (double __x, int *__exponent) __attribute__ ((__nothrow__ )); extern double __frexp (double __x, int *__exponent) __attribute__ ((__nothrow__ ));
extern double ldexp (double __x, int __exponent) __attribute__ ((__nothrow__ )); extern double __ldexp (double __x, int __exponent) __attribute__ ((__nothrow__ ));
 extern double log (double __x) __attribute__ ((__nothrow__ )); extern double __log (double __x) __attribute__ ((__nothrow__ ));
 extern double log10 (double __x) __attribute__ ((__nothrow__ )); extern double __log10 (double __x) __attribute__ ((__nothrow__ ));
extern double modf (double __x, double *__iptr) __attribute__ ((__nothrow__ )); extern double __modf (double __x, double *__iptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
 extern double expm1 (double __x) __attribute__ ((__nothrow__ )); extern double __expm1 (double __x) __attribute__ ((__nothrow__ ));
 extern double log1p (double __x) __attribute__ ((__nothrow__ )); extern double __log1p (double __x) __attribute__ ((__nothrow__ ));
extern double logb (double __x) __attribute__ ((__nothrow__ )); extern double __logb (double __x) __attribute__ ((__nothrow__ ));
 extern double exp2 (double __x) __attribute__ ((__nothrow__ )); extern double __exp2 (double __x) __attribute__ ((__nothrow__ ));
 extern double log2 (double __x) __attribute__ ((__nothrow__ )); extern double __log2 (double __x) __attribute__ ((__nothrow__ ));
 extern double pow (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __pow (double __x, double __y) __attribute__ ((__nothrow__ ));
extern double sqrt (double __x) __attribute__ ((__nothrow__ )); extern double __sqrt (double __x) __attribute__ ((__nothrow__ ));
 extern double hypot (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __hypot (double __x, double __y) __attribute__ ((__nothrow__ ));
 extern double cbrt (double __x) __attribute__ ((__nothrow__ )); extern double __cbrt (double __x) __attribute__ ((__nothrow__ ));
extern double ceil (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern double __ceil (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern double fabs (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern double __fabs (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern double floor (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern double __floor (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern double fmod (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __fmod (double __x, double __y) __attribute__ ((__nothrow__ ));
extern int isinf (double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int finite (double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern double drem (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __drem (double __x, double __y) __attribute__ ((__nothrow__ ));
extern double significand (double __x) __attribute__ ((__nothrow__ )); extern double __significand (double __x) __attribute__ ((__nothrow__ ));
extern double copysign (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern double __copysign (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern double nan (const char *__tagb) __attribute__ ((__nothrow__ )); extern double __nan (const char *__tagb) __attribute__ ((__nothrow__ ));
extern int isnan (double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern double j0 (double) __attribute__ ((__nothrow__ )); extern double __j0 (double) __attribute__ ((__nothrow__ ));
extern double j1 (double) __attribute__ ((__nothrow__ )); extern double __j1 (double) __attribute__ ((__nothrow__ ));
extern double jn (int, double) __attribute__ ((__nothrow__ )); extern double __jn (int, double) __attribute__ ((__nothrow__ ));
extern double y0 (double) __attribute__ ((__nothrow__ )); extern double __y0 (double) __attribute__ ((__nothrow__ ));
extern double y1 (double) __attribute__ ((__nothrow__ )); extern double __y1 (double) __attribute__ ((__nothrow__ ));
extern double yn (int, double) __attribute__ ((__nothrow__ )); extern double __yn (int, double) __attribute__ ((__nothrow__ ));
 extern double erf (double) __attribute__ ((__nothrow__ )); extern double __erf (double) __attribute__ ((__nothrow__ ));
 extern double erfc (double) __attribute__ ((__nothrow__ )); extern double __erfc (double) __attribute__ ((__nothrow__ ));
extern double lgamma (double) __attribute__ ((__nothrow__ )); extern double __lgamma (double) __attribute__ ((__nothrow__ ));
extern double tgamma (double) __attribute__ ((__nothrow__ )); extern double __tgamma (double) __attribute__ ((__nothrow__ ));
extern double gamma (double) __attribute__ ((__nothrow__ )); extern double __gamma (double) __attribute__ ((__nothrow__ ));
extern double lgamma_r (double, int *__signgamp) __attribute__ ((__nothrow__ )); extern double __lgamma_r (double, int *__signgamp) __attribute__ ((__nothrow__ ));
extern double rint (double __x) __attribute__ ((__nothrow__ )); extern double __rint (double __x) __attribute__ ((__nothrow__ ));
extern double nextafter (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __nextafter (double __x, double __y) __attribute__ ((__nothrow__ ));
extern double nexttoward (double __x, long double __y) __attribute__ ((__nothrow__ )); extern double __nexttoward (double __x, long double __y) __attribute__ ((__nothrow__ ));
extern double remainder (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __remainder (double __x, double __y) __attribute__ ((__nothrow__ ));
extern double scalbn (double __x, int __n) __attribute__ ((__nothrow__ )); extern double __scalbn (double __x, int __n) __attribute__ ((__nothrow__ ));
extern int ilogb (double __x) __attribute__ ((__nothrow__ )); extern int __ilogb (double __x) __attribute__ ((__nothrow__ ));
extern double scalbln (double __x, long int __n) __attribute__ ((__nothrow__ )); extern double __scalbln (double __x, long int __n) __attribute__ ((__nothrow__ ));
extern double nearbyint (double __x) __attribute__ ((__nothrow__ )); extern double __nearbyint (double __x) __attribute__ ((__nothrow__ ));
extern double round (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern double __round (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern double trunc (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern double __trunc (double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern double remquo (double __x, double __y, int *__quo) __attribute__ ((__nothrow__ )); extern double __remquo (double __x, double __y, int *__quo) __attribute__ ((__nothrow__ ));
extern long int lrint (double __x) __attribute__ ((__nothrow__ )); extern long int __lrint (double __x) __attribute__ ((__nothrow__ ));
__extension__
extern long long int llrint (double __x) __attribute__ ((__nothrow__ )); extern long long int __llrint (double __x) __attribute__ ((__nothrow__ ));
extern long int lround (double __x) __attribute__ ((__nothrow__ )); extern long int __lround (double __x) __attribute__ ((__nothrow__ ));
__extension__
extern long long int llround (double __x) __attribute__ ((__nothrow__ )); extern long long int __llround (double __x) __attribute__ ((__nothrow__ ));
extern double fdim (double __x, double __y) __attribute__ ((__nothrow__ )); extern double __fdim (double __x, double __y) __attribute__ ((__nothrow__ ));
extern double fmax (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern double __fmax (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern double fmin (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern double __fmin (double __x, double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern double fma (double __x, double __y, double __z) __attribute__ ((__nothrow__ )); extern double __fma (double __x, double __y, double __z) __attribute__ ((__nothrow__ ));
extern double scalb (double __x, double __n) __attribute__ ((__nothrow__ )); extern double __scalb (double __x, double __n) __attribute__ ((__nothrow__ ));
extern int __fpclassifyf (float __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
extern int __signbitf (float __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
extern int __isinff (float __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __finitef (float __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __isnanf (float __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __iseqsigf (float __x, float __y) __attribute__ ((__nothrow__ ));
extern int __issignalingf (float __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
 extern float acosf (float __x) __attribute__ ((__nothrow__ )); extern float __acosf (float __x) __attribute__ ((__nothrow__ ));
 extern float asinf (float __x) __attribute__ ((__nothrow__ )); extern float __asinf (float __x) __attribute__ ((__nothrow__ ));
 extern float atanf (float __x) __attribute__ ((__nothrow__ )); extern float __atanf (float __x) __attribute__ ((__nothrow__ ));
 extern float atan2f (float __y, float __x) __attribute__ ((__nothrow__ )); extern float __atan2f (float __y, float __x) __attribute__ ((__nothrow__ ));
 extern float cosf (float __x) __attribute__ ((__nothrow__ )); extern float __cosf (float __x) __attribute__ ((__nothrow__ ));
 extern float sinf (float __x) __attribute__ ((__nothrow__ )); extern float __sinf (float __x) __attribute__ ((__nothrow__ ));
 extern float tanf (float __x) __attribute__ ((__nothrow__ )); extern float __tanf (float __x) __attribute__ ((__nothrow__ ));
 extern float coshf (float __x) __attribute__ ((__nothrow__ )); extern float __coshf (float __x) __attribute__ ((__nothrow__ ));
 extern float sinhf (float __x) __attribute__ ((__nothrow__ )); extern float __sinhf (float __x) __attribute__ ((__nothrow__ ));
 extern float tanhf (float __x) __attribute__ ((__nothrow__ )); extern float __tanhf (float __x) __attribute__ ((__nothrow__ ));
 extern float acoshf (float __x) __attribute__ ((__nothrow__ )); extern float __acoshf (float __x) __attribute__ ((__nothrow__ ));
 extern float asinhf (float __x) __attribute__ ((__nothrow__ )); extern float __asinhf (float __x) __attribute__ ((__nothrow__ ));
 extern float atanhf (float __x) __attribute__ ((__nothrow__ )); extern float __atanhf (float __x) __attribute__ ((__nothrow__ ));
 extern float expf (float __x) __attribute__ ((__nothrow__ )); extern float __expf (float __x) __attribute__ ((__nothrow__ ));
extern float frexpf (float __x, int *__exponent) __attribute__ ((__nothrow__ )); extern float __frexpf (float __x, int *__exponent) __attribute__ ((__nothrow__ ));
extern float ldexpf (float __x, int __exponent) __attribute__ ((__nothrow__ )); extern float __ldexpf (float __x, int __exponent) __attribute__ ((__nothrow__ ));
 extern float logf (float __x) __attribute__ ((__nothrow__ )); extern float __logf (float __x) __attribute__ ((__nothrow__ ));
 extern float log10f (float __x) __attribute__ ((__nothrow__ )); extern float __log10f (float __x) __attribute__ ((__nothrow__ ));
extern float modff (float __x, float *__iptr) __attribute__ ((__nothrow__ )); extern float __modff (float __x, float *__iptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
 extern float expm1f (float __x) __attribute__ ((__nothrow__ )); extern float __expm1f (float __x) __attribute__ ((__nothrow__ ));
 extern float log1pf (float __x) __attribute__ ((__nothrow__ )); extern float __log1pf (float __x) __attribute__ ((__nothrow__ ));
extern float logbf (float __x) __attribute__ ((__nothrow__ )); extern float __logbf (float __x) __attribute__ ((__nothrow__ ));
 extern float exp2f (float __x) __attribute__ ((__nothrow__ )); extern float __exp2f (float __x) __attribute__ ((__nothrow__ ));
 extern float log2f (float __x) __attribute__ ((__nothrow__ )); extern float __log2f (float __x) __attribute__ ((__nothrow__ ));
 extern float powf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __powf (float __x, float __y) __attribute__ ((__nothrow__ ));
extern float sqrtf (float __x) __attribute__ ((__nothrow__ )); extern float __sqrtf (float __x) __attribute__ ((__nothrow__ ));
 extern float hypotf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __hypotf (float __x, float __y) __attribute__ ((__nothrow__ ));
 extern float cbrtf (float __x) __attribute__ ((__nothrow__ )); extern float __cbrtf (float __x) __attribute__ ((__nothrow__ ));
extern float ceilf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern float __ceilf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern float fabsf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern float __fabsf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern float floorf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern float __floorf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern float fmodf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __fmodf (float __x, float __y) __attribute__ ((__nothrow__ ));
extern int isinff (float __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int finitef (float __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern float dremf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __dremf (float __x, float __y) __attribute__ ((__nothrow__ ));
extern float significandf (float __x) __attribute__ ((__nothrow__ )); extern float __significandf (float __x) __attribute__ ((__nothrow__ ));
extern float copysignf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern float __copysignf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern float nanf (const char *__tagb) __attribute__ ((__nothrow__ )); extern float __nanf (const char *__tagb) __attribute__ ((__nothrow__ ));
extern int isnanf (float __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern float j0f (float) __attribute__ ((__nothrow__ )); extern float __j0f (float) __attribute__ ((__nothrow__ ));
extern float j1f (float) __attribute__ ((__nothrow__ )); extern float __j1f (float) __attribute__ ((__nothrow__ ));
extern float jnf (int, float) __attribute__ ((__nothrow__ )); extern float __jnf (int, float) __attribute__ ((__nothrow__ ));
extern float y0f (float) __attribute__ ((__nothrow__ )); extern float __y0f (float) __attribute__ ((__nothrow__ ));
extern float y1f (float) __attribute__ ((__nothrow__ )); extern float __y1f (float) __attribute__ ((__nothrow__ ));
extern float ynf (int, float) __attribute__ ((__nothrow__ )); extern float __ynf (int, float) __attribute__ ((__nothrow__ ));
 extern float erff (float) __attribute__ ((__nothrow__ )); extern float __erff (float) __attribute__ ((__nothrow__ ));
 extern float erfcf (float) __attribute__ ((__nothrow__ )); extern float __erfcf (float) __attribute__ ((__nothrow__ ));
extern float lgammaf (float) __attribute__ ((__nothrow__ )); extern float __lgammaf (float) __attribute__ ((__nothrow__ ));
extern float tgammaf (float) __attribute__ ((__nothrow__ )); extern float __tgammaf (float) __attribute__ ((__nothrow__ ));
extern float gammaf (float) __attribute__ ((__nothrow__ )); extern float __gammaf (float) __attribute__ ((__nothrow__ ));
extern float lgammaf_r (float, int *__signgamp) __attribute__ ((__nothrow__ )); extern float __lgammaf_r (float, int *__signgamp) __attribute__ ((__nothrow__ ));
extern float rintf (float __x) __attribute__ ((__nothrow__ )); extern float __rintf (float __x) __attribute__ ((__nothrow__ ));
extern float nextafterf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __nextafterf (float __x, float __y) __attribute__ ((__nothrow__ ));
extern float nexttowardf (float __x, long double __y) __attribute__ ((__nothrow__ )); extern float __nexttowardf (float __x, long double __y) __attribute__ ((__nothrow__ ));
extern float remainderf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __remainderf (float __x, float __y) __attribute__ ((__nothrow__ ));
extern float scalbnf (float __x, int __n) __attribute__ ((__nothrow__ )); extern float __scalbnf (float __x, int __n) __attribute__ ((__nothrow__ ));
extern int ilogbf (float __x) __attribute__ ((__nothrow__ )); extern int __ilogbf (float __x) __attribute__ ((__nothrow__ ));
extern float scalblnf (float __x, long int __n) __attribute__ ((__nothrow__ )); extern float __scalblnf (float __x, long int __n) __attribute__ ((__nothrow__ ));
extern float nearbyintf (float __x) __attribute__ ((__nothrow__ )); extern float __nearbyintf (float __x) __attribute__ ((__nothrow__ ));
extern float roundf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern float __roundf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern float truncf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern float __truncf (float __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern float remquof (float __x, float __y, int *__quo) __attribute__ ((__nothrow__ )); extern float __remquof (float __x, float __y, int *__quo) __attribute__ ((__nothrow__ ));
extern long int lrintf (float __x) __attribute__ ((__nothrow__ )); extern long int __lrintf (float __x) __attribute__ ((__nothrow__ ));
__extension__
extern long long int llrintf (float __x) __attribute__ ((__nothrow__ )); extern long long int __llrintf (float __x) __attribute__ ((__nothrow__ ));
extern long int lroundf (float __x) __attribute__ ((__nothrow__ )); extern long int __lroundf (float __x) __attribute__ ((__nothrow__ ));
__extension__
extern long long int llroundf (float __x) __attribute__ ((__nothrow__ )); extern long long int __llroundf (float __x) __attribute__ ((__nothrow__ ));
extern float fdimf (float __x, float __y) __attribute__ ((__nothrow__ )); extern float __fdimf (float __x, float __y) __attribute__ ((__nothrow__ ));
extern float fmaxf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern float __fmaxf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern float fminf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern float __fminf (float __x, float __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern float fmaf (float __x, float __y, float __z) __attribute__ ((__nothrow__ )); extern float __fmaf (float __x, float __y, float __z) __attribute__ ((__nothrow__ ));
extern float scalbf (float __x, float __n) __attribute__ ((__nothrow__ )); extern float __scalbf (float __x, float __n) __attribute__ ((__nothrow__ ));
extern int __fpclassifyl (long double __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
extern int __signbitl (long double __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
extern int __isinfl (long double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __finitel (long double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __isnanl (long double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int __iseqsigl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
extern int __issignalingl (long double __value) __attribute__ ((__nothrow__ ))
     __attribute__ ((__const__));
 extern long double acosl (long double __x) __attribute__ ((__nothrow__ )); extern long double __acosl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double asinl (long double __x) __attribute__ ((__nothrow__ )); extern long double __asinl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double atanl (long double __x) __attribute__ ((__nothrow__ )); extern long double __atanl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double atan2l (long double __y, long double __x) __attribute__ ((__nothrow__ )); extern long double __atan2l (long double __y, long double __x) __attribute__ ((__nothrow__ ));
 extern long double cosl (long double __x) __attribute__ ((__nothrow__ )); extern long double __cosl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double sinl (long double __x) __attribute__ ((__nothrow__ )); extern long double __sinl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double tanl (long double __x) __attribute__ ((__nothrow__ )); extern long double __tanl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double coshl (long double __x) __attribute__ ((__nothrow__ )); extern long double __coshl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double sinhl (long double __x) __attribute__ ((__nothrow__ )); extern long double __sinhl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double tanhl (long double __x) __attribute__ ((__nothrow__ )); extern long double __tanhl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double acoshl (long double __x) __attribute__ ((__nothrow__ )); extern long double __acoshl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double asinhl (long double __x) __attribute__ ((__nothrow__ )); extern long double __asinhl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double atanhl (long double __x) __attribute__ ((__nothrow__ )); extern long double __atanhl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double expl (long double __x) __attribute__ ((__nothrow__ )); extern long double __expl (long double __x) __attribute__ ((__nothrow__ ));
extern long double frexpl (long double __x, int *__exponent) __attribute__ ((__nothrow__ )); extern long double __frexpl (long double __x, int *__exponent) __attribute__ ((__nothrow__ ));
extern long double ldexpl (long double __x, int __exponent) __attribute__ ((__nothrow__ )); extern long double __ldexpl (long double __x, int __exponent) __attribute__ ((__nothrow__ ));
 extern long double logl (long double __x) __attribute__ ((__nothrow__ )); extern long double __logl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double log10l (long double __x) __attribute__ ((__nothrow__ )); extern long double __log10l (long double __x) __attribute__ ((__nothrow__ ));
extern long double modfl (long double __x, long double *__iptr) __attribute__ ((__nothrow__ )); extern long double __modfl (long double __x, long double *__iptr) __attribute__ ((__nothrow__ )) __attribute__ ((__nonnull__ (2)));
 extern long double expm1l (long double __x) __attribute__ ((__nothrow__ )); extern long double __expm1l (long double __x) __attribute__ ((__nothrow__ ));
 extern long double log1pl (long double __x) __attribute__ ((__nothrow__ )); extern long double __log1pl (long double __x) __attribute__ ((__nothrow__ ));
extern long double logbl (long double __x) __attribute__ ((__nothrow__ )); extern long double __logbl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double exp2l (long double __x) __attribute__ ((__nothrow__ )); extern long double __exp2l (long double __x) __attribute__ ((__nothrow__ ));
 extern long double log2l (long double __x) __attribute__ ((__nothrow__ )); extern long double __log2l (long double __x) __attribute__ ((__nothrow__ ));
 extern long double powl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __powl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
extern long double sqrtl (long double __x) __attribute__ ((__nothrow__ )); extern long double __sqrtl (long double __x) __attribute__ ((__nothrow__ ));
 extern long double hypotl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __hypotl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
 extern long double cbrtl (long double __x) __attribute__ ((__nothrow__ )); extern long double __cbrtl (long double __x) __attribute__ ((__nothrow__ ));
extern long double ceill (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern long double __ceill (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern long double fabsl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern long double __fabsl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern long double floorl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern long double __floorl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern long double fmodl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __fmodl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
extern int isinfl (long double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern int finitel (long double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern long double dreml (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __dreml (long double __x, long double __y) __attribute__ ((__nothrow__ ));
extern long double significandl (long double __x) __attribute__ ((__nothrow__ )); extern long double __significandl (long double __x) __attribute__ ((__nothrow__ ));
extern long double copysignl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern long double __copysignl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern long double nanl (const char *__tagb) __attribute__ ((__nothrow__ )); extern long double __nanl (const char *__tagb) __attribute__ ((__nothrow__ ));
extern int isnanl (long double __value) __attribute__ ((__nothrow__ ))
  __attribute__ ((__const__));
extern long double j0l (long double) __attribute__ ((__nothrow__ )); extern long double __j0l (long double) __attribute__ ((__nothrow__ ));
extern long double j1l (long double) __attribute__ ((__nothrow__ )); extern long double __j1l (long double) __attribute__ ((__nothrow__ ));
extern long double jnl (int, long double) __attribute__ ((__nothrow__ )); extern long double __jnl (int, long double) __attribute__ ((__nothrow__ ));
extern long double y0l (long double) __attribute__ ((__nothrow__ )); extern long double __y0l (long double) __attribute__ ((__nothrow__ ));
extern long double y1l (long double) __attribute__ ((__nothrow__ )); extern long double __y1l (long double) __attribute__ ((__nothrow__ ));
extern long double ynl (int, long double) __attribute__ ((__nothrow__ )); extern long double __ynl (int, long double) __attribute__ ((__nothrow__ ));
 extern long double erfl (long double) __attribute__ ((__nothrow__ )); extern long double __erfl (long double) __attribute__ ((__nothrow__ ));
 extern long double erfcl (long double) __attribute__ ((__nothrow__ )); extern long double __erfcl (long double) __attribute__ ((__nothrow__ ));
extern long double lgammal (long double) __attribute__ ((__nothrow__ )); extern long double __lgammal (long double) __attribute__ ((__nothrow__ ));
extern long double tgammal (long double) __attribute__ ((__nothrow__ )); extern long double __tgammal (long double) __attribute__ ((__nothrow__ ));
extern long double gammal (long double) __attribute__ ((__nothrow__ )); extern long double __gammal (long double) __attribute__ ((__nothrow__ ));
extern long double lgammal_r (long double, int *__signgamp) __attribute__ ((__nothrow__ )); extern long double __lgammal_r (long double, int *__signgamp) __attribute__ ((__nothrow__ ));
extern long double rintl (long double __x) __attribute__ ((__nothrow__ )); extern long double __rintl (long double __x) __attribute__ ((__nothrow__ ));
extern long double nextafterl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __nextafterl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
extern long double nexttowardl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __nexttowardl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
extern long double remainderl (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __remainderl (long double __x, long double __y) __attribute__ ((__nothrow__ ));
extern long double scalbnl (long double __x, int __n) __attribute__ ((__nothrow__ )); extern long double __scalbnl (long double __x, int __n) __attribute__ ((__nothrow__ ));
extern int ilogbl (long double __x) __attribute__ ((__nothrow__ )); extern int __ilogbl (long double __x) __attribute__ ((__nothrow__ ));
extern long double scalblnl (long double __x, long int __n) __attribute__ ((__nothrow__ )); extern long double __scalblnl (long double __x, long int __n) __attribute__ ((__nothrow__ ));
extern long double nearbyintl (long double __x) __attribute__ ((__nothrow__ )); extern long double __nearbyintl (long double __x) __attribute__ ((__nothrow__ ));
extern long double roundl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern long double __roundl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern long double truncl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern long double __truncl (long double __x) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern long double remquol (long double __x, long double __y, int *__quo) __attribute__ ((__nothrow__ )); extern long double __remquol (long double __x, long double __y, int *__quo) __attribute__ ((__nothrow__ ));
extern long int lrintl (long double __x) __attribute__ ((__nothrow__ )); extern long int __lrintl (long double __x) __attribute__ ((__nothrow__ ));
__extension__
extern long long int llrintl (long double __x) __attribute__ ((__nothrow__ )); extern long long int __llrintl (long double __x) __attribute__ ((__nothrow__ ));
extern long int lroundl (long double __x) __attribute__ ((__nothrow__ )); extern long int __lroundl (long double __x) __attribute__ ((__nothrow__ ));
__extension__
extern long long int llroundl (long double __x) __attribute__ ((__nothrow__ )); extern long long int __llroundl (long double __x) __attribute__ ((__nothrow__ ));
extern long double fdiml (long double __x, long double __y) __attribute__ ((__nothrow__ )); extern long double __fdiml (long double __x, long double __y) __attribute__ ((__nothrow__ ));
extern long double fmaxl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern long double __fmaxl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern long double fminl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__)); extern long double __fminl (long double __x, long double __y) __attribute__ ((__nothrow__ )) __attribute__ ((__const__));
extern long double fmal (long double __x, long double __y, long double __z) __attribute__ ((__nothrow__ )); extern long double __fmal (long double __x, long double __y, long double __z) __attribute__ ((__nothrow__ ));
extern long double scalbl (long double __x, long double __n) __attribute__ ((__nothrow__ )); extern long double __scalbl (long double __x, long double __n) __attribute__ ((__nothrow__ ));
extern int signgam;
enum
  {
    FP_NAN =
      0,
    FP_INFINITE =
      1,
    FP_ZERO =
      2,
    FP_SUBNORMAL =
      3,
    FP_NORMAL =
      4
  };

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
typedef long unsigned int size_t;
typedef int wchar_t;
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
typedef __off_t off_t;
typedef __pid_t pid_t;
typedef __id_t id_t;
typedef __ssize_t ssize_t;
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

int ti_find_indicator_valid(int8_t name[16]);
const char* ti_version();
long int ti_build();
typedef int (*ti_indicator_start_function)(double const *options);
typedef int (*ti_indicator_function)(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
typedef struct ti_indicator_info {
    char *name;
    char *full_name;
    ti_indicator_start_function start;
    ti_indicator_function indicator;
    int type, inputs, options, outputs;
    char *input_names[10];
    char *option_names[10];
    char *output_names[10];
} ti_indicator_info;
extern ti_indicator_info ti_indicators[];
const ti_indicator_info *ti_find_indicator(const char *name);
int ti_abs_start(double const *options);
int ti_abs(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_acos_start(double const *options);
int ti_acos(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_ad_start(double const *options);
int ti_ad(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_add_start(double const *options);
int ti_add(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_adosc_start(double const *options);
int ti_adosc(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_adx_start(double const *options);
int ti_adx(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_adxr_start(double const *options);
int ti_adxr(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_ao_start(double const *options);
int ti_ao(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_apo_start(double const *options);
int ti_apo(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_aroon_start(double const *options);
int ti_aroon(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_aroonosc_start(double const *options);
int ti_aroonosc(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_asin_start(double const *options);
int ti_asin(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_atan_start(double const *options);
int ti_atan(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_atr_start(double const *options);
int ti_atr(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_avgprice_start(double const *options);
int ti_avgprice(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_bbands_start(double const *options);
int ti_bbands(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_bop_start(double const *options);
int ti_bop(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_cci_start(double const *options);
int ti_cci(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_ceil_start(double const *options);
int ti_ceil(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_cmo_start(double const *options);
int ti_cmo(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_cos_start(double const *options);
int ti_cos(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_cosh_start(double const *options);
int ti_cosh(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_crossany_start(double const *options);
int ti_crossany(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_crossover_start(double const *options);
int ti_crossover(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_cvi_start(double const *options);
int ti_cvi(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_decay_start(double const *options);
int ti_decay(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_dema_start(double const *options);
int ti_dema(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_di_start(double const *options);
int ti_di(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_div_start(double const *options);
int ti_div(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_dm_start(double const *options);
int ti_dm(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_dpo_start(double const *options);
int ti_dpo(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_dx_start(double const *options);
int ti_dx(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_edecay_start(double const *options);
int ti_edecay(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_ema_start(double const *options);
int ti_ema(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_emv_start(double const *options);
int ti_emv(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_exp_start(double const *options);
int ti_exp(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_fisher_start(double const *options);
int ti_fisher(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_floor_start(double const *options);
int ti_floor(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_fosc_start(double const *options);
int ti_fosc(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_hma_start(double const *options);
int ti_hma(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_kama_start(double const *options);
int ti_kama(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_kvo_start(double const *options);
int ti_kvo(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_lag_start(double const *options);
int ti_lag(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_linreg_start(double const *options);
int ti_linreg(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_linregintercept_start(double const *options);
int ti_linregintercept(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_linregslope_start(double const *options);
int ti_linregslope(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_ln_start(double const *options);
int ti_ln(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_log10_start(double const *options);
int ti_log10(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_macd_start(double const *options);
int ti_macd(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_marketfi_start(double const *options);
int ti_marketfi(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_mass_start(double const *options);
int ti_mass(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_max_start(double const *options);
int ti_max(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_md_start(double const *options);
int ti_md(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_medprice_start(double const *options);
int ti_medprice(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_mfi_start(double const *options);
int ti_mfi(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_min_start(double const *options);
int ti_min(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_mom_start(double const *options);
int ti_mom(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_msw_start(double const *options);
int ti_msw(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_mul_start(double const *options);
int ti_mul(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_natr_start(double const *options);
int ti_natr(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_nvi_start(double const *options);
int ti_nvi(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_obv_start(double const *options);
int ti_obv(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_ppo_start(double const *options);
int ti_ppo(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_psar_start(double const *options);
int ti_psar(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_pvi_start(double const *options);
int ti_pvi(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_qstick_start(double const *options);
int ti_qstick(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_roc_start(double const *options);
int ti_roc(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_rocr_start(double const *options);
int ti_rocr(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_round_start(double const *options);
int ti_round(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_rsi_start(double const *options);
int ti_rsi(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_sin_start(double const *options);
int ti_sin(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_sinh_start(double const *options);
int ti_sinh(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_sma_start(double const *options);
int ti_sma(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_sqrt_start(double const *options);
int ti_sqrt(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_stddev_start(double const *options);
int ti_stddev(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_stderr_start(double const *options);
int ti_stderr(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_stoch_start(double const *options);
int ti_stoch(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_stochrsi_start(double const *options);
int ti_stochrsi(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_sub_start(double const *options);
int ti_sub(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_sum_start(double const *options);
int ti_sum(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_tan_start(double const *options);
int ti_tan(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_tanh_start(double const *options);
int ti_tanh(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_tema_start(double const *options);
int ti_tema(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_todeg_start(double const *options);
int ti_todeg(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_torad_start(double const *options);
int ti_torad(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_tr_start(double const *options);
int ti_tr(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_trima_start(double const *options);
int ti_trima(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_trix_start(double const *options);
int ti_trix(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_trunc_start(double const *options);
int ti_trunc(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_tsf_start(double const *options);
int ti_tsf(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_typprice_start(double const *options);
int ti_typprice(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_ultosc_start(double const *options);
int ti_ultosc(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_var_start(double const *options);
int ti_var(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_vhf_start(double const *options);
int ti_vhf(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_vidya_start(double const *options);
int ti_vidya(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_volatility_start(double const *options);
int ti_volatility(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_vosc_start(double const *options);
int ti_vosc(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_vwma_start(double const *options);
int ti_vwma(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_wad_start(double const *options);
int ti_wad(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_wcprice_start(double const *options);
int ti_wcprice(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_wilders_start(double const *options);
int ti_wilders(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_willr_start(double const *options);
int ti_willr(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_wma_start(double const *options);
int ti_wma(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
int ti_zlema_start(double const *options);
int ti_zlema(int size,
      double const *const *inputs,
      double const *options,
      double *const *outputs);
typedef struct {
    int size, pushes, index;
    double sum;
    double vals[1];
} ti_buffer;
ti_buffer *ti_buffer_new(int size);
void ti_buffer_free(ti_buffer *buffer);
int ti_abs_start(double const *options) { (void)options; return 0; } int ti_abs(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (fabs(in1[i])); } return 0; }
int ti_acos_start(double const *options) { (void)options; return 0; } int ti_acos(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (acos(in1[i])); } return 0; }
int ti_ad_start(double const *options) {
    (void)options;
    return 0;
}
int ti_ad(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const double *volume = inputs[3];
    (void)options;
    double *output = outputs[0];
    double sum = 0;
    int i;
    for (i = 0; i < size; ++i) {
        const double hl = (high[i] - low[i]);
        if (hl != 0.0) {
            sum += (close[i] - low[i] - high[i] + close[i]) / hl * volume[i];
        }
        output[i] = sum;
    }
    return 0;
}
int ti_add_start(double const *options) { (void)options; return 0; } int ti_add(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; const double *in2 = inputs[1]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (in1[i] + in2[i]); } return 0; }
int ti_adosc_start(double const *options) {
    return (int)(options[1])-1;
}
int ti_adosc(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const double *volume = inputs[3];
    const int short_period = (int)options[0];
    const int long_period = (int)options[1];
    const int start = long_period - 1;
    if (short_period < 1) return 1;
    if (long_period < short_period) return 1;
    if (size <= ti_adosc_start(options)) return 0;
    const double short_per = 2 / ((double)short_period + 1);
    const double long_per = 2 / ((double)long_period + 1);
    double *output = outputs[0];
    double sum = 0, short_ema = 0, long_ema = 0;
    int i;
    for (i = 0; i < size; ++i) {
        const double hl = (high[i] - low[i]);
        if (hl != 0.0) {
            sum += (close[i] - low[i] - high[i] + close[i]) / hl * volume[i];
        }
        if (i == 0) {
            short_ema = sum;
            long_ema = sum;
        } else {
            short_ema = (sum-short_ema) * short_per + short_ema;
            long_ema = (sum-long_ema) * long_per + long_ema;
        }
        if (i >= start) {
            *output++ = short_ema - long_ema;
        }
    }
    ((void) sizeof ((output - outputs[0] == size - ti_adosc_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_adosc_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_adosc_start(options)", "<stdin>", 2410, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_adx_start(double const *options) {
    return ((int)options[0]-1) * 2;
}
int ti_adx(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 2) return 1;
    if (size <= ti_adx_start(options)) return 0;
    const double per = ((double)period-1) / ((double)period);
    const double invper = 1.0 / ((double)period);
    double atr = 0;
    double dmup = 0;
    double dmdown = 0;
    int i;
    for (i = 1; i < period; ++i) {
        double truerange;
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        atr += truerange;
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup += dp;
        dmdown += dm;
    }
    double adx = 0.0;
    {
        double di_up = dmup / atr;
        double di_down = dmdown / atr;
        double dm_diff = fabs(di_up - di_down);
        double dm_sum = di_up + di_down;
        double dx = dm_diff / dm_sum * 100;
        adx += dx;
    }
    for (i = period; i < size; ++i) {
        double truerange;
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        atr = atr * per + truerange;
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup = dmup * per + dp;
        dmdown = dmdown * per + dm;
        double di_up = dmup / atr;
        double di_down = dmdown / atr;
        double dm_diff = fabs(di_up - di_down);
        double dm_sum = di_up + di_down;
        double dx = dm_diff / dm_sum * 100;
        if (i-period < period-2) {
            adx += dx;
        } else if (i-period == period-2) {
            adx += dx;
            *output++ = adx * invper;
        } else {
            adx = adx * per + dx;
            *output++ = adx * invper;
        }
    }
    ((void) sizeof ((output - outputs[0] == size - ti_adx_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_adx_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_adx_start(options)", "<stdin>", 2525, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_adxr_start(double const *options) {
    return ((int)options[0]-1) * 3;
}
int ti_adxr(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 2) return 1;
    if (size <= ti_adxr_start(options)) return 0;
    const double per = ((double)period-1) / ((double)period);
    const double invper = 1.0 / ((double)period);
    double atr = 0;
    double dmup = 0;
    double dmdown = 0;
    int i;
    for (i = 1; i < period; ++i) {
        double truerange;
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        atr += truerange;
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup += dp;
        dmdown += dm;
    }
    double adx = 0.0;
    {
        double di_up = dmup / atr;
        double di_down = dmdown / atr;
        double dm_diff = fabs(di_up - di_down);
        double dm_sum = di_up + di_down;
        double dx = dm_diff / dm_sum * 100;
        adx += dx;
    }
    ti_buffer *adxr = ti_buffer_new(period-1);
    const int first_adxr = ti_adxr_start(options);
    for (i = period; i < size; ++i) {
        double truerange;
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        atr = atr * per + truerange;
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup = dmup * per + dp;
        dmdown = dmdown * per + dm;
        double di_up = dmup / atr;
        double di_down = dmdown / atr;
        double dm_diff = fabs(di_up - di_down);
        double dm_sum = di_up + di_down;
        double dx = dm_diff / dm_sum * 100;
        if (i-period < period-2) {
            adx += dx;
        } else if (i-period == period-2) {
            adx += dx;
            do { (adxr)->vals[(adxr)->index] = (adx * invper); (adxr)->index = ((adxr)->index + 1); if ((adxr)->index >= (adxr)->size) (adxr)->index = 0; } while (0);
        } else {
            adx = adx * per + dx;
            if (i >= first_adxr) {
                *output++ = 0.5 * (adx * invper + ((adxr)->vals[((adxr)->index + (adxr)->size - 1 + (1)) % (adxr)->size]));
            }
            do { (adxr)->vals[(adxr)->index] = (adx * invper); (adxr)->index = ((adxr)->index + 1); if ((adxr)->index >= (adxr)->size) (adxr)->index = 0; } while (0);
        }
    }
    ti_buffer_free(adxr);
    ((void) sizeof ((output - outputs[0] == size - ti_adxr_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_adxr_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_adxr_start(options)", "<stdin>", 2648, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_ao_start(double const *options) {
    (void)options;
    return 33;
}
int ti_ao(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const int period = 34;
    double *output = outputs[0];
    if (size <= ti_ao_start(options)) return 0;
    double sum34 = 0;
    double sum5 = 0;
    const double per34 = 1.0 / 34.0;
    const double per5 = 1.0 / 5.0;
    int i;
    for (i = 0; i < 34; ++i) {
        double hl = 0.5 * (high[i] + low[i]);
        sum34 += hl;
        if (i >= 29) sum5 += hl;
    }
    *output++ = (per5 * sum5 - per34 * sum34);
    for (i = period; i < size; ++i) {
        double hl = 0.5 * (high[i] + low[i]);
        sum34 += hl;
        sum5 += hl;
        sum34 -= 0.5 * (high[i-34] + low[i-34]);
        sum5 -= 0.5 * (high[i-5] + low[i-5]);
        *output++ = (per5 * sum5 - per34 * sum34);
    }
    ((void) sizeof ((output - outputs[0] == size - ti_ao_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_ao_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_ao_start(options)", "<stdin>", 2717, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_apo_start(double const *options) {
    (void)options;
    return 1;
}
int ti_apo(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *apo = outputs[0];
    const int short_period = (int)options[0];
    const int long_period = (int)options[1];
    if (short_period < 1) return 1;
    if (long_period < 2) return 1;
    if (long_period < short_period) return 1;
    if (size <= ti_apo_start(options)) return 0;
    double short_per = 2 / ((double)short_period + 1);
    double long_per = 2 / ((double)long_period + 1);
    double short_ema = input[0];
    double long_ema = input[0];
    int i;
    for (i = 1; i < size; ++i) {
        short_ema = (input[i]-short_ema) * short_per + short_ema;
        long_ema = (input[i]-long_ema) * long_per + long_ema;
        const double out = short_ema - long_ema;
        *apo++ = out;
    }
    ((void) sizeof ((apo - outputs[0] == size - ti_apo_start(options)) ? 1 : 0), __extension__ ({ if (apo - outputs[0] == size - ti_apo_start(options)) ; else __assert_fail ("apo - outputs[0] == size - ti_apo_start(options)", "<stdin>", 2781, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_aroon_start(double const *options) {
    return (int)options[0];
}
int ti_aroon(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    double *adown = outputs[0];
    double *aup = outputs[1];
    const int period = (int)options[0];
    if (period < 1) return 1;
    if (size <= ti_aroon_start(options)) return 0;
    const double scale = 100.0 / period;
    int trail = 0, maxi = -1, mini = -1;
    double max = high[0];
    double min = low[0];
    double bar;
    int i, j;
    for (i = period; i < size; ++i, ++trail) {
        bar = high[i];
        if (maxi < trail) {
            maxi = trail;
            max = high[maxi];
            j = trail;
            while(++j <= i) {
                bar = high[j];
                if (bar >= max) {
                    max = bar;
                    maxi = j;
                }
            }
        } else if (bar >= max) {
            maxi = i;
            max = bar;
        }
        bar = low[i];
        if (mini < trail) {
            mini = trail;
            min = low[mini];
            j = trail;
            while(++j <= i) {
                bar = low[j];
                if (bar <= min) {
                    min = bar;
                    mini = j;
                }
            }
        } else if (bar <= min) {
            mini = i;
            min = bar;
        }
        *adown++ = ((double)period - (i-mini)) * scale;
        *aup++ = ((double)period - (i-maxi)) * scale;
    }
    ((void) sizeof ((adown - outputs[0] == size - ti_aroon_start(options)) ? 1 : 0), __extension__ ({ if (adown - outputs[0] == size - ti_aroon_start(options)) ; else __assert_fail ("adown - outputs[0] == size - ti_aroon_start(options)", "<stdin>", 2877, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((aup - outputs[1] == size - ti_aroon_start(options)) ? 1 : 0), __extension__ ({ if (aup - outputs[1] == size - ti_aroon_start(options)) ; else __assert_fail ("aup - outputs[1] == size - ti_aroon_start(options)", "<stdin>", 2878, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_aroonosc_start(double const *options) {
    return (int)options[0];
}
int ti_aroonosc(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    double *output = outputs[0];
    const int period = (int)options[0];
    if (period < 1) return 1;
    if (size <= ti_aroon_start(options)) return 0;
    const double scale = 100.0 / period;
    int trail = 0, maxi = -1, mini = -1;
    double max = high[0];
    double min = low[0];
    int i, j;
    for (i = period; i < size; ++i, ++trail) {
        double bar = high[i];
        if (maxi < trail) {
            maxi = trail;
            max = high[maxi];
            j = trail;
            while(++j <= i) {
                bar = high[j];
                if (bar >= max) {
                    max = bar;
                    maxi = j;
                }
            }
        } else if (bar >= max) {
            maxi = i;
            max = bar;
        }
        bar = low[i];
        if (mini < trail) {
            mini = trail;
            min = low[mini];
            j = trail;
            while(++j <= i) {
                bar = low[j];
                if (bar <= min) {
                    min = bar;
                    mini = j;
                }
            }
        } else if (bar <= min) {
            mini = i;
            min = bar;
        }
        *output++ = (maxi-mini) * scale;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_aroonosc_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_aroonosc_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_aroonosc_start(options)", "<stdin>", 2980, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_asin_start(double const *options) { (void)options; return 0; } int ti_asin(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (asin(in1[i])); } return 0; }
int ti_atan_start(double const *options) { (void)options; return 0; } int ti_atan(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (atan(in1[i])); } return 0; }
int ti_atr_start(double const *options) {
    return (int)options[0]-1;
}
int ti_atr(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_atr_start(options)) return 0;
    const double per = 1.0 / ((double)period);
    double sum = 0;
    double truerange;
    sum += high[0] - low[0];
    int i;
    for (i = 1; i < period; ++i) {
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        sum += truerange;
    }
    double val = sum / period;
    *output++ = val;
    for (i = period; i < size; ++i) {
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        val = (truerange-val) * per + val;
        *output++ = val;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_atr_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_atr_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_atr_start(options)", "<stdin>", 3103, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_avgprice_start(double const *options) {
    (void)options;
    return 0;
}
int ti_avgprice(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *open = inputs[0];
    const double *high = inputs[1];
    const double *low = inputs[2];
    const double *close = inputs[3];
    (void)options;
    double *output = outputs[0];
    int i;
    for (i = 0; i < size; ++i) {
        output[i] = (open[i] + high[i] + low[i] + close[i]) * 0.25;
    }
    return 0;
}
int ti_bbands_start(double const *options) {
    return (int)options[0]-1;
}
int ti_bbands(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *lower = outputs[0];
    double *middle = outputs[1];
    double *upper = outputs[2];
    const int period = (int)options[0];
    const double stddev = options[1];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_bbands_start(options)) return 0;
    double sum = 0;
    double sum2 = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i];
        sum2 += input[i] * input[i];
    }
    double sd = sqrt(sum2 * scale - (sum * scale) * (sum * scale));
    *middle = sum * scale;
    *lower++ = *middle - stddev * sd;
    *upper++ = *middle + stddev * sd;
    ++middle;
    for (i = period; i < size; ++i) {
        sum += input[i];
        sum2 += input[i] * input[i];
        sum -= input[i-period];
        sum2 -= input[i-period] * input[i-period];
        sd = sqrt(sum2 * scale - (sum * scale) * (sum * scale));
        *middle = sum * scale;
        *upper++ = *middle + stddev * sd;
        *lower++ = *middle - stddev * sd;
        ++middle;
    }
    ((void) sizeof ((lower - outputs[0] == size - ti_bbands_start(options)) ? 1 : 0), __extension__ ({ if (lower - outputs[0] == size - ti_bbands_start(options)) ; else __assert_fail ("lower - outputs[0] == size - ti_bbands_start(options)", "<stdin>", 3227, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((middle - outputs[1] == size - ti_bbands_start(options)) ? 1 : 0), __extension__ ({ if (middle - outputs[1] == size - ti_bbands_start(options)) ; else __assert_fail ("middle - outputs[1] == size - ti_bbands_start(options)", "<stdin>", 3228, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((upper - outputs[2] == size - ti_bbands_start(options)) ? 1 : 0), __extension__ ({ if (upper - outputs[2] == size - ti_bbands_start(options)) ; else __assert_fail ("upper - outputs[2] == size - ti_bbands_start(options)", "<stdin>", 3229, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_bop_start(double const *options) {
    (void)options;
    return 0;
}
int ti_bop(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *open = inputs[0];
    const double *high = inputs[1];
    const double *low = inputs[2];
    const double *close = inputs[3];
    (void)options;
    double *output = outputs[0];
    int i;
    for (i = 0; i < size; ++i) {
        double hl = high[i] - low[i];
        if (hl <= 0.0) {
            output[i] = 0;
        } else {
            output[i] = (close[i] - open[i]) / hl;
        }
    }
    return 0;
}
int ti_cci_start(double const *options) {
    const int period = (int)options[0];
    return (period-1) * 2;
}
int ti_cci(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int period = (int)options[0];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_cci_start(options)) return 0;
    double *output = outputs[0];
    ti_buffer *sum = ti_buffer_new(period);
    int i, j;
    for (i = 0; i < size; ++i) {
        const double today = ((high[(i)] + low[(i)] + close[(i)]) * (1.0/3.0));
        do { if ((sum)->pushes >= (sum)->size) { (sum)->sum -= (sum)->vals[(sum)->index]; } (sum)->sum += (today); (sum)->vals[(sum)->index] = (today); (sum)->pushes += 1; (sum)->index = ((sum)->index + 1); if ((sum)->index >= (sum)->size) (sum)->index = 0; } while (0);
        const double avg = sum->sum * scale;
        if (i >= period * 2 - 2) {
            double acc = 0;
            for (j = 0; j < period; ++j) {
                acc += fabs(avg - sum->vals[j]);
            }
            double cci = acc * scale;
            cci *= .015;
            cci = (today-avg)/cci;
            *output++ = cci;
        }
    }
    ti_buffer_free(sum);
    ((void) sizeof ((output - outputs[0] == size - ti_cci_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_cci_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_cci_start(options)", "<stdin>", 3356, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_ceil_start(double const *options) { (void)options; return 0; } int ti_ceil(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (ceil(in1[i])); } return 0; }
int ti_cmo_start(double const *options) {
    return (int)options[0];
}
int ti_cmo(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *output = outputs[0];
    const int period = (int)options[0];
    if (period < 1) return 1;
    if (size <= ti_cmo_start(options)) return 0;
    double up_sum = 0, down_sum = 0;
    int i;
    for (i = 1; i <= period; ++i) {
        up_sum += (input[(i)] > input[(i)-1] ? input[(i)] - input[(i)-1] : 0);
        down_sum += (input[(i)] < input[(i)-1] ? input[(i)-1] - input[(i)] : 0);
    }
    *output++ = 100 * (up_sum - down_sum) / (up_sum + down_sum);
    for (i = period+1; i < size; ++i) {
        up_sum -= (input[(i-period)] > input[(i-period)-1] ? input[(i-period)] - input[(i-period)-1] : 0);
        down_sum -= (input[(i-period)] < input[(i-period)-1] ? input[(i-period)-1] - input[(i-period)] : 0);
        up_sum += (input[(i)] > input[(i)-1] ? input[(i)] - input[(i)-1] : 0);
        down_sum += (input[(i)] < input[(i)-1] ? input[(i)-1] - input[(i)] : 0);
        *output++ = 100 * (up_sum - down_sum) / (up_sum + down_sum);
    }
    ((void) sizeof ((output - outputs[0] == size - ti_cmo_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_cmo_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_cmo_start(options)", "<stdin>", 3449, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_cos_start(double const *options) { (void)options; return 0; } int ti_cos(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (cos(in1[i])); } return 0; }
int ti_cosh_start(double const *options) { (void)options; return 0; } int ti_cosh(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (cosh(in1[i])); } return 0; }
int ti_crossany_start(double const *options) {
    (void)options;
    return 1;
}
int ti_crossany(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *a = inputs[0];
    const double *b = inputs[1];
    (void)options;
    double *output = outputs[0];
    int i;
    for (i = 1; i < size; ++i) {
        *output++ = (a[i] > b[i] && a[i-1] <= b[i-1])
                 || (a[i] < b[i] && a[i-1] >= b[i-1]);
    }
    return 0;
}
int ti_crossover_start(double const *options) {
    (void)options;
    return 1;
}
int ti_crossover(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *a = inputs[0];
    const double *b = inputs[1];
    (void)options;
    double *output = outputs[0];
    int i;
    for (i = 1; i < size; ++i) {
        *output++ = a[i] > b[i] && a[i-1] <= b[i-1];
    }
    return 0;
}
int ti_cvi_start(double const *options) {
    const int n = (int)options[0];
    return n*2-1;
}
int ti_cvi(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_cvi_start(options)) return 0;
    const double per = 2 / ((double)period + 1);
    ti_buffer *lag = ti_buffer_new(period);
    double val = high[0]-low[0];
    int i;
    for (i = 1; i < period*2-1; ++i) {
        val = ((high[i]-low[i])-val) * per + val;
        do { (lag)->vals[(lag)->index] = (val); (lag)->index = ((lag)->index + 1); if ((lag)->index >= (lag)->size) (lag)->index = 0; } while (0);
    }
    for (i = period*2-1; i < size; ++i) {
        val = ((high[i]-low[i])-val) * per + val;
        const double old = lag->vals[lag->index];
        *output++ = 100.0 * (val - old) / old;
        do { (lag)->vals[(lag)->index] = (val); (lag)->index = ((lag)->index + 1); if ((lag)->index >= (lag)->size) (lag)->index = 0; } while (0);
    }
    ti_buffer_free(lag);
    ((void) sizeof ((output - outputs[0] == size - ti_cvi_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_cvi_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_cvi_start(options)", "<stdin>", 3664, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_decay_start(double const *options) {
    (void)options;
    return 0;
}
int ti_decay(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *output = outputs[0];
    const int period = (int)options[0];
    const double scale = 1.0 / period;
    *output++ = input[0];
    int i;
    for (i = 1; i < size; ++i) {
        double d = output[-1] - scale;
        *output++ = input[i] > d ? input[i] : d;
    }
    return 0;
}
int ti_dema_start(double const *options) {
    const int period = (int)options[0];
    return (period-1) * 2;
}
int ti_dema(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_dema_start(options)) return 0;
    const double per = 2 / ((double)period + 1);
    const double per1 = 1.0 - per;
    double ema = input[0];
    double ema2 = ema;
    int i;
    for (i = 0; i < size; ++i) {
        ema = ema * per1 + input[i] * per;
        if (i == period-1) {
            ema2 = ema;
        }
        if (i >= period-1) {
            ema2 = ema2 * per1 + ema * per;
            if (i >= (period-1) * 2) {
                *output = ema * 2 - ema2;
                ++output;
            }
        }
    }
    ((void) sizeof ((output - outputs[0] == size - ti_dema_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_dema_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_dema_start(options)", "<stdin>", 3778, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_di_start(double const *options) {
    return (int)options[0]-1;
}
int ti_di(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int period = (int)options[0];
    double *plus_di = outputs[0];
    double *minus_di = outputs[1];
    if (period < 1) return 1;
    if (size <= ti_di_start(options)) return 0;
    const double per = ((double)period-1) / ((double)period);
    double atr = 0;
    double dmup = 0;
    double dmdown = 0;
    int i;
    for (i = 1; i < period; ++i) {
        double truerange;
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        atr += truerange;
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup += dp;
        dmdown += dm;
    }
    *plus_di++ = 100.0 * dmup / atr;
    *minus_di++ = 100.0 * dmdown / atr;
    for (i = period; i < size; ++i) {
        double truerange;
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        atr = atr * per + truerange;
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup = dmup * per + dp;
        dmdown = dmdown * per + dm;
        *plus_di++ = 100.0 * dmup / atr;
        *minus_di++ = 100.0 * dmdown / atr;
    }
    ((void) sizeof ((plus_di - outputs[0] == size - ti_di_start(options)) ? 1 : 0), __extension__ ({ if (plus_di - outputs[0] == size - ti_di_start(options)) ; else __assert_fail ("plus_di - outputs[0] == size - ti_di_start(options)", "<stdin>", 3867, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((minus_di - outputs[1] == size - ti_di_start(options)) ? 1 : 0), __extension__ ({ if (minus_di - outputs[1] == size - ti_di_start(options)) ; else __assert_fail ("minus_di - outputs[1] == size - ti_di_start(options)", "<stdin>", 3868, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_div_start(double const *options) { (void)options; return 0; } int ti_div(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; const double *in2 = inputs[1]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (in1[i] / in2[i]); } return 0; }
int ti_dm_start(double const *options) {
    return (int)options[0]-1;
}
int ti_dm(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const int period = (int)options[0];
    double *plus_dm = outputs[0];
    double *minus_dm = outputs[1];
    if (period < 1) return 1;
    if (size <= ti_dm_start(options)) return 0;
    const double per = ((double)period-1) / ((double)period);
    double dmup = 0;
    double dmdown = 0;
    int i;
    for (i = 1; i < period; ++i) {
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup += dp;
        dmdown += dm;
    }
    *plus_dm++ = dmup;
    *minus_dm++ = dmdown;
    for (i = period; i < size; ++i) {
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup = dmup * per + dp;
        dmdown = dmdown * per + dm;
        *plus_dm++ = dmup;
        *minus_dm++ = dmdown;
    }
    ((void) sizeof ((plus_dm - outputs[0] == size - ti_dm_start(options)) ? 1 : 0), __extension__ ({ if (plus_dm - outputs[0] == size - ti_dm_start(options)) ; else __assert_fail ("plus_dm - outputs[0] == size - ti_dm_start(options)", "<stdin>", 3974, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((minus_dm - outputs[1] == size - ti_dm_start(options)) ? 1 : 0), __extension__ ({ if (minus_dm - outputs[1] == size - ti_dm_start(options)) ; else __assert_fail ("minus_dm - outputs[1] == size - ti_dm_start(options)", "<stdin>", 3975, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_dpo_start(double const *options) {
    return (int)options[0]-1;
}
int ti_dpo(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    const int back = period / 2 + 1;
    double *output = outputs[0];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_dpo_start(options)) return 0;
    double sum = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i];
    }
    *output++ = input[period-1-back] - (sum * scale);
    for (i = period; i < size; ++i) {
        sum += input[i];
        sum -= input[i-period];
        *output++ = input[i-back] - (sum * scale);
    }
    ((void) sizeof ((output - outputs[0] == size - ti_dpo_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_dpo_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_dpo_start(options)", "<stdin>", 4035, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_dx_start(double const *options) {
    return (int)options[0]-1;
}
int ti_dx(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_dx_start(options)) return 0;
    const double per = ((double)period-1) / ((double)period);
    double atr = 0;
    double dmup = 0;
    double dmdown = 0;
    int i;
    for (i = 1; i < period; ++i) {
        double truerange;
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        atr += truerange;
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup += dp;
        dmdown += dm;
    }
    {
        double di_up = dmup / atr;
        double di_down = dmdown / atr;
        double dm_diff = fabs(di_up - di_down);
        double dm_sum = di_up + di_down;
        double dx = dm_diff / dm_sum * 100;
        *output++ = dx;
    }
    for (i = period; i < size; ++i) {
        double truerange;
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        atr = atr * per + truerange;
        double dp, dm;
        do { dp = high[i] - high[i-1]; dm = low[i-1] - low[i]; if (dp < 0) dp = 0; else if (dp > dm) dm = 0; if (dm < 0) dm = 0; else if (dm > dp) dp = 0;} while (0);
        dmup = dmup * per + dp;
        dmdown = dmdown * per + dm;
        double di_up = dmup / atr;
        double di_down = dmdown / atr;
        double dm_diff = fabs(di_up - di_down);
        double dm_sum = di_up + di_down;
        double dx = dm_diff / dm_sum * 100;
        *output++ = dx;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_dx_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_dx_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_dx_start(options)", "<stdin>", 4137, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_edecay_start(double const *options) {
    (void)options;
    return 0;
}
int ti_edecay(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    const double scale = 1.0 - 1.0 / period;
    *output++ = input[0];
    int i;
    for (i = 1; i < size; ++i) {
        double d = output[-1] * scale;
        *output++ = input[i] > d ? input[i] : d;
    }
    return 0;
}
int ti_ema_start(double const *options) {
    (void)options;
    return 0;
}
int ti_ema(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_ema_start(options)) return 0;
    const double per = 2 / ((double)period + 1);
    double val = input[0];
    *output++ = val;
    int i;
    for (i = 1; i < size; ++i) {
        val = (input[i]-val) * per + val;
        *output++ = val;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_ema_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_ema_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_ema_start(options)", "<stdin>", 4240, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_emv_start(double const *options) {
    (void)options;
    return 1;
}
int ti_emv(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *volume = inputs[2];
    (void)options;
    double *output = outputs[0];
    if (size <= ti_emv_start(options)) return 0;
    double last = (high[0] + low[0]) * 0.5;
    int i;
    for (i = 1; i < size; ++i) {
        double hl = (high[i] + low[i]) * 0.5;
        double br = volume[i] / 10000.0 / (high[i] - low[i]);
        *output++ = (hl - last) / br;
        last = hl;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_emv_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_emv_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_emv_start(options)", "<stdin>", 4296, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_exp_start(double const *options) { (void)options; return 0; } int ti_exp(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (exp(in1[i])); } return 0; }
int ti_fisher_start(double const *options) {
    return (int)options[0]-1;
}
int ti_fisher(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    double *fisher = outputs[0];
    double *signal = outputs[1];
    const int period = (int)options[0];
    if (period < 1) return 1;
    if (size <= ti_fisher_start(options)) return 0;
    int trail = 0, maxi = -1, mini = -1;
    double max = (0.5 * (high[(0)] + low[(0)]));
    double min = (0.5 * (high[(0)] + low[(0)]));
    double val1 = 0.0;
    double bar;
    double fish = 0.0;
    int i, j;
    for (i = period-1; i < size; ++i, ++trail) {
        bar = (0.5 * (high[(i)] + low[(i)]));
        if (maxi < trail) {
            maxi = trail;
            max = (0.5 * (high[(maxi)] + low[(maxi)]));
            j = trail;
            while(++j <= i) {
                bar = (0.5 * (high[(j)] + low[(j)]));
                if (bar >= max) {
                    max = bar;
                    maxi = j;
                }
            }
        } else if (bar >= max) {
            maxi = i;
            max = bar;
        }
        bar = (0.5 * (high[(i)] + low[(i)]));
        if (mini < trail) {
            mini = trail;
            min = (0.5 * (high[(mini)] + low[(mini)]));
            j = trail;
            while(++j <= i) {
                bar = (0.5 * (high[(j)] + low[(j)]));
                if (bar <= min) {
                    min = bar;
                    mini = j;
                }
            }
        } else if (bar <= min) {
            mini = i;
            min = bar;
        }
        double mm = max - min;
        if (mm == 0.0) mm = 0.001;
        val1 = 0.33 * 2.0 * ( ((0.5 * (high[(i)] + low[(i)]))-min) / (mm) - 0.5) + 0.67 * val1;
        if (val1 > 0.99) val1 = .999;
        if (val1 < -0.99) val1 = -.999;
        *signal++ = fish;
        fish = 0.5 * log((1.0+val1)/(1.0-val1)) + 0.5 * fish;
        *fisher++ = fish;
    }
    ((void) sizeof ((fisher - outputs[0] == size - ti_fisher_start(options)) ? 1 : 0), __extension__ ({ if (fisher - outputs[0] == size - ti_fisher_start(options)) ; else __assert_fail ("fisher - outputs[0] == size - ti_fisher_start(options)", "<stdin>", 4426, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((signal - outputs[1] == size - ti_fisher_start(options)) ? 1 : 0), __extension__ ({ if (signal - outputs[1] == size - ti_fisher_start(options)) ; else __assert_fail ("signal - outputs[1] == size - ti_fisher_start(options)", "<stdin>", 4427, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_floor_start(double const *options) { (void)options; return 0; } int ti_floor(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (floor(in1[i])); } return 0; }
int ti_fosc_start(double const *options) {
    return (int)options[0];
}
int ti_fosc(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_fosc_start(options)) return 0;
    do { double x = 0; double x2 = 0; double y = 0; double xy = 0; const double p = (1.0 / (period)); double tsf = 0;; int i; for (i = 0; i < (period)-1; ++i) { x += i+1; x2 += (i+1)*(i+1); xy += (input)[i] * (i+1); y += (input)[i]; } x += (period); x2 += (period) * (period); const double bd = 1.0 / ((period) * x2 - x * x); for (i = (period)-1; i < (size); ++i) { xy += (input)[i] * (period); y += (input)[i]; const double b = ((period) * xy - x * y) * bd; do { const double a = (y - b * x) * p; if (i >= (period)) {*output++ = 100 * (input[i] - tsf) / input[i];} tsf = (a + b * ((period+1))); } while (0); xy -= y; y -= (input)[i-(period)+1]; } } while (0);
    ((void) sizeof ((output - outputs[0] == size - ti_fosc_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_fosc_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_fosc_start(options)", "<stdin>", 4505, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_hma_start(double const *options) {
    const int period = (int)options[0];
    const int periodsqrt = (int)(sqrt(period));
    return period + periodsqrt - 2;
}
int ti_hma(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_hma_start(options)) return 0;
    const int period2 = (int)(period / 2);
    const int periodsqrt = (int)(sqrt(period));
    const double weights = period * (period+1) / 2;
    const double weights2 = period2 * (period2+1) / 2;
    const double weightssqrt = periodsqrt * (periodsqrt+1) / 2;
    double sum = 0;
    double weight_sum = 0;
    double sum2 = 0;
    double weight_sum2 = 0;
    double sumsqrt = 0;
    double weight_sumsqrt = 0;
    int i;
    for (i = 0; i < period-1; ++i) {
        weight_sum += input[i] * (i+1);
        sum += input[i];
        if (i >= period - period2) {
            weight_sum2 += input[i] * (i+1-(period-period2));
            sum2 += input[i];
        }
    }
    ti_buffer *buff = ti_buffer_new(periodsqrt);
    for (i = period-1; i < size; ++i) {
        weight_sum += input[i] * period;
        sum += input[i];
        weight_sum2 += input[i] * period2;
        sum2 += input[i];
        const double wma = weight_sum / weights;
        const double wma2 = weight_sum2 / weights2;
        const double diff = 2 * wma2 - wma;
        weight_sumsqrt += diff * periodsqrt;
        sumsqrt += diff;
        do { (buff)->vals[(buff)->index] = (diff); (buff)->index = ((buff)->index + 1); if ((buff)->index >= (buff)->size) (buff)->index = 0; } while (0);
        if (i >= (period-1) + (periodsqrt-1)) {
            *output++ = weight_sumsqrt / weightssqrt;
            weight_sumsqrt -= sumsqrt;
            sumsqrt -= ((buff)->vals[((buff)->index + (buff)->size - 1 + (1)) % (buff)->size]);
        } else {
            weight_sumsqrt -= sumsqrt;
        }
        weight_sum -= sum;
        sum -= input[i-period+1];
        weight_sum2 -= sum2;
        sum2 -= input[i-period2+1];
    }
    ti_buffer_free(buff);
    ((void) sizeof ((output - outputs[0] == size - ti_hma_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_hma_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_hma_start(options)", "<stdin>", 4617, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_kama_start(double const *options) {
    return (int)options[0]-1;
}
int ti_kama(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_kama_start(options)) return 0;
    const double short_per = 2 / (2.0 + 1);
    const double long_per = 2 / (30.0 + 1);
    double sum = 0;
    int i;
    for (i = 1; i < period; ++i) {
        sum += fabs(input[i] - input[i-1]);
    }
    double kama = input[period-1];
    *output++ = kama;
    double er, sc;
    for (i = period; i < size; ++i) {
        sum += fabs(input[i] - input[i-1]);
        if (i > period) {
            sum -= fabs(input[i-period] - input[i-period-1]);
        }
        if (sum != 0.0) {
            er = fabs(input[i] - input[i-period]) / sum;
        } else {
            er = 1.0;
        }
        sc = pow(er * (short_per - long_per) + long_per, 2);
        kama = kama + sc * (input[i] - kama);
        *output++ = kama;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_kama_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_kama_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_kama_start(options)", "<stdin>", 4692, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_kvo_start(double const *options) {
    (void)options;
    return 1;
}
int ti_kvo(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const double *volume = inputs[3];
    const int short_period = (int)options[0];
    const int long_period = (int)options[1];
    if (short_period < 1) return 1;
    if (long_period < short_period) return 1;
    if (size <= ti_kvo_start(options)) return 0;
    const double short_per = 2 / ((double)short_period + 1);
    const double long_per = 2 / ((double)long_period + 1);
    double *output = outputs[0];
    double cm = 0;
    double prev_hlc = high[0] + low[0] + close[0];
    int trend = -1;
    double short_ema = 0, long_ema = 0;
    int i;
    for (i = 1; i < size; ++i) {
        const double hlc = high[i] + low[i] + close[i];
        const double dm = high[i] - low[i];
        if (hlc > prev_hlc && trend != 1) {
            trend = 1;
            cm = high[i-1] - low[i-1];
        } else if (hlc < prev_hlc && trend != 0) {
            trend = 0;
            cm = high[i-1] - low[i-1];
        }
        cm += dm;
        const double vf = volume[i] * fabs(dm / cm * 2 - 1) * 100 * (trend ? 1.0 : -1.0);
        if (i == 1) {
            short_ema = vf;
            long_ema = vf;
        } else {
            short_ema = (vf-short_ema) * short_per + short_ema;
            long_ema = (vf-long_ema) * long_per + long_ema;
        }
        *output++ = short_ema - long_ema;
        prev_hlc = hlc;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_kvo_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_kvo_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_kvo_start(options)", "<stdin>", 4780, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_lag_start(double const *options) {
    return (int)options[0];
}
int ti_lag(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 0) return 1;
    if (size <= ti_lag_start(options)) return 0;
    int i;
    for (i = period; i < size; ++i) {
        *output++ = input[i-period];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_lag_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_lag_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_lag_start(options)", "<stdin>", 4827, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_linreg_start(double const *options) {
    return (int)options[0]-1;
}
int ti_linreg(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_linreg_start(options)) return 0;
    do { double x = 0; double x2 = 0; double y = 0; double xy = 0; const double p = (1.0 / (period)); double tsf = 0;; int i; for (i = 0; i < (period)-1; ++i) { x += i+1; x2 += (i+1)*(i+1); xy += (input)[i] * (i+1); y += (input)[i]; } x += (period); x2 += (period) * (period); const double bd = 1.0 / ((period) * x2 - x * x); for (i = (period)-1; i < (size); ++i) { xy += (input)[i] * (period); y += (input)[i]; const double b = ((period) * xy - x * y) * bd; do { const double a = (y - b * x) * p; if (i >= (period)) {*output++ = 100 * (input[i] - tsf) / input[i];} tsf = (a + b * ((period))); } while (0); xy -= y; y -= (input)[i-(period)+1]; } } while (0);
    ((void) sizeof ((output - outputs[0] == size - ti_linreg_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_linreg_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_linreg_start(options)", "<stdin>", 4871, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_linregintercept_start(double const *options) {
    return (int)options[0]-1;
}
int ti_linregintercept(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_linregintercept_start(options)) return 0;
    do { double x = 0; double x2 = 0; double y = 0; double xy = 0; const double p = (1.0 / (period)); double tsf = 0;; int i; for (i = 0; i < (period)-1; ++i) { x += i+1; x2 += (i+1)*(i+1); xy += (input)[i] * (i+1); y += (input)[i]; } x += (period); x2 += (period) * (period); const double bd = 1.0 / ((period) * x2 - x * x); for (i = (period)-1; i < (size); ++i) { xy += (input)[i] * (period); y += (input)[i]; const double b = ((period) * xy - x * y) * bd; do { const double a = (y - b * x) * p; if (i >= (period)) {*output++ = 100 * (input[i] - tsf) / input[i];} tsf = (a + b * ((1))); } while (0); xy -= y; y -= (input)[i-(period)+1]; } } while (0);
    ((void) sizeof ((output - outputs[0] == size - ti_linregintercept_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_linregintercept_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_linregintercept_start(options)", "<stdin>", 4915, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_linregslope_start(double const *options) {
    return (int)options[0]-1;
}
int ti_linregslope(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_linregslope_start(options)) return 0;
    do { double x = 0; double x2 = 0; double y = 0; double xy = 0; do{}while(0); int i; for (i = 0; i < (period)-1; ++i) { x += i+1; x2 += (i+1)*(i+1); xy += (input)[i] * (i+1); y += (input)[i]; } x += (period); x2 += (period) * (period); const double bd = 1.0 / ((period) * x2 - x * x); for (i = (period)-1; i < (size); ++i) { xy += (input)[i] * (period); y += (input)[i]; const double b = ((period) * xy - x * y) * bd; do { *output++ = b; } while (0); xy -= y; y -= (input)[i-(period)+1]; } } while (0);
    ((void) sizeof ((output - outputs[0] == size - ti_linregslope_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_linregslope_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_linregslope_start(options)", "<stdin>", 4965, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_ln_start(double const *options) { (void)options; return 0; } int ti_ln(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (log(in1[i])); } return 0; }
int ti_log10_start(double const *options) { (void)options; return 0; } int ti_log10(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (log10(in1[i])); } return 0; }
int ti_macd_start(double const *options) {
    const int long_period = (int)options[1];
    return (long_period-1);
}
int ti_macd(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *macd = outputs[0];
    double *signal = outputs[1];
    double *hist = outputs[2];
    const int short_period = (int)options[0];
    const int long_period = (int)options[1];
    const int signal_period = (int)options[2];
    if (short_period < 1) return 1;
    if (long_period < 2) return 1;
    if (long_period < short_period) return 1;
    if (signal_period < 1) return 1;
    if (size <= ti_macd_start(options)) return 0;
    double short_per = 2 / ((double)short_period + 1);
    double long_per = 2 / ((double)long_period + 1);
    double signal_per = 2 / ((double)signal_period + 1);
    if (short_period == 12 && long_period == 26) {
        short_per = 0.15;
        long_per = 0.075;
    }
    double short_ema = input[0];
    double long_ema = input[0];
    double signal_ema = 0;
    int i;
    for (i = 1; i < size; ++i) {
        short_ema = (input[i]-short_ema) * short_per + short_ema;
        long_ema = (input[i]-long_ema) * long_per + long_ema;
        const double out = short_ema - long_ema;
        if (i == long_period-1) {
            signal_ema = out;
        }
        if (i >= long_period-1) {
            signal_ema = (out-signal_ema) * signal_per + signal_ema;
            *macd++ = out;
            *signal++ = signal_ema;
            *hist++ = out - signal_ema;
        }
    }
    ((void) sizeof ((macd - outputs[0] == size - ti_macd_start(options)) ? 1 : 0), __extension__ ({ if (macd - outputs[0] == size - ti_macd_start(options)) ; else __assert_fail ("macd - outputs[0] == size - ti_macd_start(options)", "<stdin>", 5106, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((signal - outputs[1] == size - ti_macd_start(options)) ? 1 : 0), __extension__ ({ if (signal - outputs[1] == size - ti_macd_start(options)) ; else __assert_fail ("signal - outputs[1] == size - ti_macd_start(options)", "<stdin>", 5107, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((hist - outputs[2] == size - ti_macd_start(options)) ? 1 : 0), __extension__ ({ if (hist - outputs[2] == size - ti_macd_start(options)) ; else __assert_fail ("hist - outputs[2] == size - ti_macd_start(options)", "<stdin>", 5108, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_marketfi_start(double const *options) {
    (void)options;
    return 0;
}
int ti_marketfi(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *volume = inputs[2];
    (void)options;
    double *output = outputs[0];
    if (size <= ti_marketfi_start(options)) return 0;
    int i;
    for (i = 0; i < size; ++i) {
        *output++ = (high[i] - low[i]) / volume[i];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_marketfi_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_marketfi_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_marketfi_start(options)", "<stdin>", 5158, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_mass_start(double const *options) {
    int sum_p = (int)options[0]-1;
    return 16 + sum_p;
}
int ti_mass(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_mass_start(options)) return 0;
    const double per = 2 / (9.0 + 1);
    const double per1 = 1.0 - per;
    double ema = high[0] - low[0];
    double ema2 = ema;
    ti_buffer *sum = ti_buffer_new(period);
    int i;
    for (i = 0; i < size; ++i) {
        double hl = high[i] - low[i];
        ema = ema * per1 + hl * per;
        if (i == 8) {
            ema2 = ema;
        }
        if (i >= 8) {
            ema2 = ema2 * per1 + ema * per;
            if (i >= 16) {
                do { if ((sum)->pushes >= (sum)->size) { (sum)->sum -= (sum)->vals[(sum)->index]; } (sum)->sum += (ema/ema2); (sum)->vals[(sum)->index] = (ema/ema2); (sum)->pushes += 1; (sum)->index = ((sum)->index + 1); if ((sum)->index >= (sum)->size) (sum)->index = 0; } while (0);
                if (i >= 16 + period - 1) {
                    *output++ = sum->sum;
                }
            }
        }
    }
    ti_buffer_free(sum);
    ((void) sizeof ((output - outputs[0] == size - ti_mass_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_mass_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_mass_start(options)", "<stdin>", 5239, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_max_start(double const *options) {
    return (int)options[0]-1;
}
int ti_max(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_max_start(options)) return 0;
    int trail = 0, maxi = -1;
    double max = input[0];
    int i, j;
    for (i = period-1; i < size; ++i, ++trail) {
        double bar = input[i];
        if (maxi < trail) {
            maxi = trail;
            max = input[maxi];
            j = trail;
            while(++j <= i) {
                bar = input[j];
                if (bar >= max) {
                    max = bar;
                    maxi = j;
                }
            }
        } else if (bar >= max) {
            maxi = i;
            max = bar;
        }
        *output++ = max;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_max_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_max_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_max_start(options)", "<stdin>", 5306, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_md_start(double const *options) {
    return (int)options[0]-1;
}
int ti_md(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_md_start(options)) return 0;
    double sum = 0;
    int i, j;
    for (i = 0; i < size; ++i) {
        const double today = input[i];
        sum += today;
        if (i >= period) sum -= input[i-period];
        const double avg = sum * scale;
        if (i >= period - 1) {
            double acc = 0;
            for (j = 0; j < period; ++j) {
                acc += fabs(avg - input[i-j]);
            }
            *output++ = acc * scale;
        }
    }
    ((void) sizeof ((output - outputs[0] == size - ti_md_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_md_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_md_start(options)", "<stdin>", 5370, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_medprice_start(double const *options) {
    (void)options;
    return 0;
}
int ti_medprice(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    (void)options;
    double *output = outputs[0];
    int i;
    for (i = 0; i < size; ++i) {
        output[i] = (high[i] + low[i]) * 0.5;
    }
    return 0;
}
int ti_mfi_start(double const *options) {
    return (int)options[0];
}
int ti_mfi(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const double *volume = inputs[3];
    const int period = (int)options[0];
    if (period < 1) return 1;
    if (size <= ti_mfi_start(options)) return 0;
    double *output = outputs[0];
    double ytyp = ((high[(0)] + low[(0)] + close[(0)]) * (1.0/3.0));
    int i;
    ti_buffer *up = ti_buffer_new(period);
    ti_buffer *down = ti_buffer_new(period);
    for (i = 1; i < size; ++i) {
        const double typ = ((high[(i)] + low[(i)] + close[(i)]) * (1.0/3.0));
        const double bar = typ * volume[i];
        if (typ > ytyp) {
            do { if ((up)->pushes >= (up)->size) { (up)->sum -= (up)->vals[(up)->index]; } (up)->sum += (bar); (up)->vals[(up)->index] = (bar); (up)->pushes += 1; (up)->index = ((up)->index + 1); if ((up)->index >= (up)->size) (up)->index = 0; } while (0);
            do { if ((down)->pushes >= (down)->size) { (down)->sum -= (down)->vals[(down)->index]; } (down)->sum += (0.0); (down)->vals[(down)->index] = (0.0); (down)->pushes += 1; (down)->index = ((down)->index + 1); if ((down)->index >= (down)->size) (down)->index = 0; } while (0);
        } else if (typ < ytyp) {
            do { if ((down)->pushes >= (down)->size) { (down)->sum -= (down)->vals[(down)->index]; } (down)->sum += (bar); (down)->vals[(down)->index] = (bar); (down)->pushes += 1; (down)->index = ((down)->index + 1); if ((down)->index >= (down)->size) (down)->index = 0; } while (0);
            do { if ((up)->pushes >= (up)->size) { (up)->sum -= (up)->vals[(up)->index]; } (up)->sum += (0.0); (up)->vals[(up)->index] = (0.0); (up)->pushes += 1; (up)->index = ((up)->index + 1); if ((up)->index >= (up)->size) (up)->index = 0; } while (0);
        } else {
            do { if ((up)->pushes >= (up)->size) { (up)->sum -= (up)->vals[(up)->index]; } (up)->sum += (0.0); (up)->vals[(up)->index] = (0.0); (up)->pushes += 1; (up)->index = ((up)->index + 1); if ((up)->index >= (up)->size) (up)->index = 0; } while (0);
            do { if ((down)->pushes >= (down)->size) { (down)->sum -= (down)->vals[(down)->index]; } (down)->sum += (0.0); (down)->vals[(down)->index] = (0.0); (down)->pushes += 1; (down)->index = ((down)->index + 1); if ((down)->index >= (down)->size) (down)->index = 0; } while (0);
        }
        ytyp = typ;
        if (i >= period) {
            *output++ = up->sum / (up->sum + down->sum) * 100.0;
        }
    }
    ti_buffer_free(up);
    ti_buffer_free(down);
    ((void) sizeof ((output - outputs[0] == size - ti_mfi_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_mfi_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_mfi_start(options)", "<stdin>", 5498, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_min_start(double const *options) {
    return (int)options[0]-1;
}
int ti_min(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_min_start(options)) return 0;
    int trail = 0, mini = -1;
    double min = input[0];
    int i, j;
    for (i = period-1; i < size; ++i, ++trail) {
        double bar = input[i];
        if (mini < trail) {
            mini = trail;
            min = input[mini];
            j = trail;
            while(++j <= i) {
                bar = input[j];
                if (bar <= min) {
                    min = bar;
                    mini = j;
                }
            }
        } else if (bar <= min) {
            mini = i;
            min = bar;
        }
        *output++ = min;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_min_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_min_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_min_start(options)", "<stdin>", 5564, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_mom_start(double const *options) {
    return (int)options[0];
}
int ti_mom(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_mom_start(options)) return 0;
    int i;
    for (i = period; i < size; ++i) {
        *output++ = input[i] - input[i-period];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_mom_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_mom_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_mom_start(options)", "<stdin>", 5610, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_msw_start(double const *options) {
    return (int)options[0];
}
int ti_msw(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *sine = outputs[0];
    double *lead = outputs[1];
    const int period = (int)options[0];
    if (period < 1) return 1;
    if (size <= ti_msw_start(options)) return 0;
    const double pi = 3.1415926;
    const double tpi = 2 * pi;
    double weight = 0, phase;
    double rp, ip;
    int i, j;
    for (i = period; i < size; ++i) {
        rp = 0;
        ip = 0;
        for (j = 0; j < period; ++j) {
            weight = input[i-j];
            rp = rp + cos(tpi * j / period) * weight;
            ip = ip + sin(tpi * j / period) * weight;
        }
        if (fabs(rp) > .001) {
            phase = atan(ip/rp);
        } else {
            phase = tpi / 2.0 * (ip < 0 ? -1.0 : 1.0);
        }
        if (rp < 0.0) phase += pi;
        phase += pi/2.0;
        if (phase < 0.0) phase += tpi;
        if (phase > tpi) phase -= tpi;
        *sine++ = sin(phase);
        *lead++ = sin(phase + pi/4.0);
    }
    ((void) sizeof ((sine - outputs[0] == size - ti_msw_start(options)) ? 1 : 0), __extension__ ({ if (sine - outputs[0] == size - ti_msw_start(options)) ; else __assert_fail ("sine - outputs[0] == size - ti_msw_start(options)", "<stdin>", 5684, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((lead - outputs[1] == size - ti_msw_start(options)) ? 1 : 0), __extension__ ({ if (lead - outputs[1] == size - ti_msw_start(options)) ; else __assert_fail ("lead - outputs[1] == size - ti_msw_start(options)", "<stdin>", 5685, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_mul_start(double const *options) { (void)options; return 0; } int ti_mul(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; const double *in2 = inputs[1]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (in1[i] * in2[i]); } return 0; }
int ti_natr_start(double const *options) {
    return (int)options[0]-1;
}
int ti_natr(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_natr_start(options)) return 0;
    const double per = 1.0 / ((double)period);
    double sum = 0;
    double truerange;
    sum += high[0] - low[0];
    int i;
    for (i = 1; i < period; ++i) {
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        sum += truerange;
    }
    double val = sum / period;
    *output++ = 100 * (val) / close[period-1];
    for (i = period; i < size; ++i) {
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        val = (truerange-val) * per + val;
        *output++ = 100 * (val) / close[i];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_natr_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_natr_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_natr_start(options)", "<stdin>", 5781, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_nvi_start(double const *options) {
    (void)options;
    return 0;
}
int ti_nvi(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *close = inputs[0];
    const double *volume = inputs[1];
    (void)options;
    double *output = outputs[0];
    if (size <= ti_nvi_start(options)) return 0;
    double nvi = 1000;
    *output++ = nvi;
    int i;
    for (i = 1; i < size; ++i) {
        if (volume[i] < volume[i-1]) {
            nvi += ((close[i] - close[i-1])/close[i-1]) * nvi;
        }
        *output++ = nvi;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_nvi_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_nvi_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_nvi_start(options)", "<stdin>", 5837, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_obv_start(double const *options) {
    (void)options;
    return 0;
}
int ti_obv(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *close = inputs[0];
    const double *volume = inputs[1];
    (void)options;
    double *output = outputs[0];
    double sum = 0;
    *output++ = sum;
    double prev = close[0];
    int i;
    for (i = 1; i < size; ++i) {
        if (close[i] > prev) {
            sum += volume[i];
        } else if (close[i] < prev) {
            sum -= volume[i];
        } else {
        }
        prev = close[i];
        *output++ = sum;
    }
    return 0;
}
int ti_ppo_start(double const *options) {
    (void)options;
    return 1;
}
int ti_ppo(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *ppo = outputs[0];
    const int short_period = (int)options[0];
    const int long_period = (int)options[1];
    if (short_period < 1) return 1;
    if (long_period < 2) return 1;
    if (long_period < short_period) return 1;
    if (size <= ti_ppo_start(options)) return 0;
    double short_per = 2 / ((double)short_period + 1);
    double long_per = 2 / ((double)long_period + 1);
    double short_ema = input[0];
    double long_ema = input[0];
    int i;
    for (i = 1; i < size; ++i) {
        short_ema = (input[i]-short_ema) * short_per + short_ema;
        long_ema = (input[i]-long_ema) * long_per + long_ema;
        const double out = 100.0 * (short_ema - long_ema) / long_ema;
        *ppo++ = out;
    }
    ((void) sizeof ((ppo - outputs[0] == size - ti_ppo_start(options)) ? 1 : 0), __extension__ ({ if (ppo - outputs[0] == size - ti_ppo_start(options)) ; else __assert_fail ("ppo - outputs[0] == size - ti_ppo_start(options)", "<stdin>", 5958, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_psar_start(double const *options) {
    (void)options;
    return 1;
}
int ti_psar(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double accel_step = options[0];
    const double accel_max = options[1];
    double *output = outputs[0];
    if (accel_step <= 0) return 1;
    if (accel_max <= accel_step) return 1;
    if (size < 2) return 0;
    int lng;
    if (high[0] + low[0] <= high[1] + low[1])
        lng = 1;
    else
        lng = 0;
    double sar, extreme;
    if (lng) {
        extreme = high[0];
        sar = low[0];
    } else {
        extreme = low[0];
        sar = high[0];
    }
    double accel = accel_step;
    int i;
    for (i = 1; i < size; ++i) {
        sar = (extreme - sar) * accel + sar;
        if (lng) {
            if (i >= 2 && (sar > low[i-2])) sar = low[i-2];
            if ((sar > low[i-1])) sar = low[i-1];
            if (accel < accel_max && high[i] > extreme) {
                accel += accel_step;
                if (accel > accel_max) accel = accel_max;
            }
            if (high[i] > extreme) extreme = high[i];
        } else {
            if (i >= 2 && (sar < high[i-2])) sar = high[i-2];
            if ((sar < high[i-1])) sar = high[i-1];
            if (accel < accel_max && low[i] < extreme) {
                accel += accel_step;
                if (accel > accel_max) accel = accel_max;
            }
            if (low[i] < extreme) extreme = low[i];
        }
        if ((lng && low[i] < sar) || (!lng && high[i] > sar)) {
            accel = accel_step;
            sar = extreme;
            lng = !lng;
            if (!lng) extreme = low[i];
            else extreme = high[i];
        }
        *output++ = sar;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_psar_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_psar_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_psar_start(options)", "<stdin>", 6081, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_pvi_start(double const *options) {
    (void)options;
    return 0;
}
int ti_pvi(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *close = inputs[0];
    const double *volume = inputs[1];
    (void)options;
    double *output = outputs[0];
    if (size <= ti_pvi_start(options)) return 0;
    double pvi = 1000;
    *output++ = pvi;
    int i;
    for (i = 1; i < size; ++i) {
        if (volume[i] > volume[i-1]) {
            pvi += ((close[i] - close[i-1])/close[i-1]) * pvi;
        }
        *output++ = pvi;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_pvi_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_pvi_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_pvi_start(options)", "<stdin>", 6137, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_qstick_start(double const *options) {
    return (int)options[0]-1;
}
int ti_qstick(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *open = inputs[0];
    const double *close = inputs[1];
    double *output = outputs[0];
    const int period = (int)options[0];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_qstick_start(options)) return 0;
    double sum = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += close[i] - open[i];
    }
    *output++ = sum * scale;
    for (i = period; i < size; ++i) {
        sum += close[i] - open[i];
        sum -= close[i-period] - open[i-period];
        *output++ = sum * scale;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_qstick_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_qstick_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_qstick_start(options)", "<stdin>", 6195, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_roc_start(double const *options) {
    return (int)options[0];
}
int ti_roc(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_roc_start(options)) return 0;
    int i;
    for (i = period; i < size; ++i) {
        *output++ = (input[i] - input[i-period]) / input[i-period];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_roc_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_roc_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_roc_start(options)", "<stdin>", 6241, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_rocr_start(double const *options) {
    return (int)options[0];
}
int ti_rocr(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_rocr_start(options)) return 0;
    int i;
    for (i = period; i < size; ++i) {
        *output++ = input[i] / input[i-period];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_rocr_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_rocr_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_rocr_start(options)", "<stdin>", 6287, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_round_start(double const *options) { (void)options; return 0; } int ti_round(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (floor(in1[i] + 0.5)); } return 0; }
int ti_rsi_start(double const *options) {
    return (int)options[0];
}
int ti_rsi(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    const double per = 1.0 / ((double)period);
    if (period < 1) return 1;
    if (size <= ti_rsi_start(options)) return 0;
    double smooth_up = 0, smooth_down = 0;
    int i;
    for (i = 1; i <= period; ++i) {
        const double upward = input[i] > input[i-1] ? input[i] - input[i-1] : 0;
        const double downward = input[i] < input[i-1] ? input[i-1] - input[i] : 0;
        smooth_up += upward;
        smooth_down += downward;
    }
    smooth_up /= period;
    smooth_down /= period;
    *output++ = 100.0 * (smooth_up / (smooth_up + smooth_down));
    for (i = period+1; i < size; ++i) {
        const double upward = input[i] > input[i-1] ? input[i] - input[i-1] : 0;
        const double downward = input[i] < input[i-1] ? input[i-1] - input[i] : 0;
        smooth_up = (upward-smooth_up) * per + smooth_up;
        smooth_down = (downward-smooth_down) * per + smooth_down;
        *output++ = 100.0 * (smooth_up / (smooth_up + smooth_down));
    }
    ((void) sizeof ((output - outputs[0] == size - ti_rsi_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_rsi_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_rsi_start(options)", "<stdin>", 6380, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_sin_start(double const *options) { (void)options; return 0; } int ti_sin(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (sin(in1[i])); } return 0; }
int ti_sinh_start(double const *options) { (void)options; return 0; } int ti_sinh(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (sinh(in1[i])); } return 0; }
int ti_sma_start(double const *options) {
    return (int)options[0]-1;
}
int ti_sma(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_sma_start(options)) return 0;
    double sum = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i];
    }
    *output++ = sum * scale;
    for (i = period; i < size; ++i) {
        sum += input[i];
        sum -= input[i-period];
        *output++ = sum * scale;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_sma_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_sma_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_sma_start(options)", "<stdin>", 6491, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_sqrt_start(double const *options) { (void)options; return 0; } int ti_sqrt(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (sqrt(in1[i])); } return 0; }
int ti_stddev_start(double const *options) {
    return (int)options[0]-1;
}
int ti_stddev(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_stddev_start(options)) return 0;
    double sum = 0;
    double sum2 = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i];
        sum2 += input[i] * input[i];
    }
    {
        double s2s2 = (sum2 * scale - (sum * scale) * (sum * scale));
        if (s2s2 > 0.0) s2s2 = sqrt(s2s2);
        *output++ = s2s2;
    }
    for (i = period; i < size; ++i) {
        sum += input[i];
        sum2 += input[i] * input[i];
        sum -= input[i-period];
        sum2 -= input[i-period] * input[i-period];
        double s2s2 = (sum2 * scale - (sum * scale) * (sum * scale));
        if (s2s2 > 0.0) s2s2 = sqrt(s2s2);
        *output++ = s2s2;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_stddev_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_stddev_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_stddev_start(options)", "<stdin>", 6587, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_stderr_start(double const *options) {
    return (int)options[0]-1;
}
int ti_stderr(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_stderr_start(options)) return 0;
    double sum = 0;
    double sum2 = 0;
    const double mul = 1.0 / sqrt(period);
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i];
        sum2 += input[i] * input[i];
    }
    {
        double s2s2 = (sum2 * scale - (sum * scale) * (sum * scale));
        if (s2s2 > 0.0) s2s2 = sqrt(s2s2);
        *output++ = mul * s2s2;
    }
    for (i = period; i < size; ++i) {
        sum += input[i];
        sum2 += input[i] * input[i];
        sum -= input[i-period];
        sum2 -= input[i-period] * input[i-period];
        double s2s2 = (sum2 * scale - (sum * scale) * (sum * scale));
        if (s2s2 > 0.0) s2s2 = sqrt(s2s2);
        *output++ = mul * s2s2;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_stderr_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_stderr_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_stderr_start(options)", "<stdin>", 6658, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_stoch_start(double const *options) {
    const int kperiod = (int)options[0];
    const int kslow = (int)options[1];
    const int dperiod = (int)options[2];
    return kperiod + kslow + dperiod - 3;
}
int ti_stoch(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int kperiod = (int)options[0];
    const int kslow = (int)options[1];
    const int dperiod = (int)options[2];
    const double kper = 1.0 / kslow;
    const double dper = 1.0 / dperiod;
    double *stoch = outputs[0];
    double *stoch_ma = outputs[1];
    if (kperiod < 1) return 1;
    if (kslow < 1) return 1;
    if (dperiod < 1) return 1;
    if (size <= ti_stoch_start(options)) return 0;
    int trail = 0, maxi = -1, mini = -1;
    double max = high[0];
    double min = low[0];
    double bar;
    ti_buffer *k_sum = ti_buffer_new(kslow);
    ti_buffer *d_sum = ti_buffer_new(dperiod);
    int i, j;
    for (i = 0; i < size; ++i) {
        if (i >= kperiod) ++trail;
        bar = high[i];
        if (maxi < trail) {
            maxi = trail;
            max = high[maxi];
            j = trail;
            while(++j <= i) {
                bar = high[j];
                if (bar >= max) {
                    max = bar;
                    maxi = j;
                }
            }
        } else if (bar >= max) {
            maxi = i;
            max = bar;
        }
        bar = low[i];
        if (mini < trail) {
            mini = trail;
            min = low[mini];
            j = trail;
            while(++j <= i) {
                bar = low[j];
                if (bar <= min) {
                    min = bar;
                    mini = j;
                }
            }
        } else if (bar <= min) {
            mini = i;
            min = bar;
        }
        const double kdiff = (max - min);
        const double kfast = kdiff == 0.0 ? 0.0 : 100 * ((close[i] - min) / kdiff);
        do { if ((k_sum)->pushes >= (k_sum)->size) { (k_sum)->sum -= (k_sum)->vals[(k_sum)->index]; } (k_sum)->sum += (kfast); (k_sum)->vals[(k_sum)->index] = (kfast); (k_sum)->pushes += 1; (k_sum)->index = ((k_sum)->index + 1); if ((k_sum)->index >= (k_sum)->size) (k_sum)->index = 0; } while (0);
        if (i >= kperiod-1 + kslow-1) {
            const double k = k_sum->sum * kper;
            do { if ((d_sum)->pushes >= (d_sum)->size) { (d_sum)->sum -= (d_sum)->vals[(d_sum)->index]; } (d_sum)->sum += (k); (d_sum)->vals[(d_sum)->index] = (k); (d_sum)->pushes += 1; (d_sum)->index = ((d_sum)->index + 1); if ((d_sum)->index >= (d_sum)->size) (d_sum)->index = 0; } while (0);
            if (i >= kperiod-1 + kslow-1 + dperiod-1) {
                *stoch++ = k;
                *stoch_ma++ = d_sum->sum * dper;
            }
        }
    }
    ti_buffer_free(k_sum);
    ti_buffer_free(d_sum);
    ((void) sizeof ((stoch - outputs[0] == size - ti_stoch_start(options)) ? 1 : 0), __extension__ ({ if (stoch - outputs[0] == size - ti_stoch_start(options)) ; else __assert_fail ("stoch - outputs[0] == size - ti_stoch_start(options)", "<stdin>", 6784, __extension__ __PRETTY_FUNCTION__); }));
    ((void) sizeof ((stoch_ma - outputs[1] == size - ti_stoch_start(options)) ? 1 : 0), __extension__ ({ if (stoch_ma - outputs[1] == size - ti_stoch_start(options)) ; else __assert_fail ("stoch_ma - outputs[1] == size - ti_stoch_start(options)", "<stdin>", 6785, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_stochrsi_start(double const *options) {
    return ((int)options[0]) * 2 - 1;
}
int ti_stochrsi(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    const double per = 1.0 / ((double)period);
    if (period < 2) return 1;
    if (size <= ti_stochrsi_start(options)) return 0;
    ti_buffer *rsi = ti_buffer_new(period);
    double smooth_up = 0, smooth_down = 0;
    int i;
    for (i = 1; i <= period; ++i) {
        const double upward = input[i] > input[i-1] ? input[i] - input[i-1] : 0;
        const double downward = input[i] < input[i-1] ? input[i-1] - input[i] : 0;
        smooth_up += upward;
        smooth_down += downward;
    }
    smooth_up /= period;
    smooth_down /= period;
    double r = 100.0 * (smooth_up / (smooth_up + smooth_down));
    do { if ((rsi)->pushes >= (rsi)->size) { (rsi)->sum -= (rsi)->vals[(rsi)->index]; } (rsi)->sum += (r); (rsi)->vals[(rsi)->index] = (r); (rsi)->pushes += 1; (rsi)->index = ((rsi)->index + 1); if ((rsi)->index >= (rsi)->size) (rsi)->index = 0; } while (0);
    double min = r;
    double max = r;
    int mini = 0;
    int maxi = 0;
    for (i = period+1; i < size; ++i) {
        const double upward = input[i] > input[i-1] ? input[i] - input[i-1] : 0;
        const double downward = input[i] < input[i-1] ? input[i-1] - input[i] : 0;
        smooth_up = (upward-smooth_up) * per + smooth_up;
        smooth_down = (downward-smooth_down) * per + smooth_down;
        r = 100.0 * (smooth_up / (smooth_up + smooth_down));
        if (r > max) {
            max = r;
            maxi = rsi->index;
        } else if (maxi == rsi->index) {
            max = r;
            int j;
            for (j = 0; j < rsi->size; ++j) {
                if (j == rsi->index) continue;
                if (rsi->vals[j] > max) {
                    max = rsi->vals[j];
                    maxi = j;
                }
            }
        }
        if (r < min) {
            min = r;
            mini = rsi->index;
        } else if (mini == rsi->index) {
            min = r;
            int j;
            for (j = 0; j < rsi->size; ++j) {
                if (j == rsi->index) continue;
                if (rsi->vals[j] < min) {
                    min = rsi->vals[j];
                    mini = j;
                }
            }
        }
        do { (rsi)->vals[(rsi)->index] = (r); (rsi)->index = ((rsi)->index + 1); if ((rsi)->index >= (rsi)->size) (rsi)->index = 0; } while (0);
        if (i > period*2 - 2) {
            const double diff = max - min;
            if (diff == 0.0) {
                *output++ = 0.0;
            } else {
                *output++ = (r - min) / (diff);
            }
        }
    }
    ti_buffer_free(rsi);
    ((void) sizeof ((output - outputs[0] == size - ti_stochrsi_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_stochrsi_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_stochrsi_start(options)", "<stdin>", 6903, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_sub_start(double const *options) { (void)options; return 0; } int ti_sub(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; const double *in2 = inputs[1]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (in1[i] - in2[i]); } return 0; }
int ti_sum_start(double const *options) {
    return (int)options[0]-1;
}
int ti_sum(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_sum_start(options)) return 0;
    double sum = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i];
    }
    *output++ = sum;
    for (i = period; i < size; ++i) {
        sum += input[i];
        sum -= input[i-period];
        *output++ = sum;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_sum_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_sum_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_sum_start(options)", "<stdin>", 6986, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_tan_start(double const *options) { (void)options; return 0; } int ti_tan(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (tan(in1[i])); } return 0; }
int ti_tanh_start(double const *options) { (void)options; return 0; } int ti_tanh(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = (tanh(in1[i])); } return 0; }
int ti_tema_start(double const *options) {
    const int period = (int)options[0];
    return (period-1) * 3;
}
int ti_tema(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_tema_start(options)) return 0;
    const double per = 2 / ((double)period + 1);
    const double per1 = 1.0 - per;
    double ema = input[0];
    double ema2 = 0;
    double ema3 = 0;
    int i;
    for (i = 0; i < size; ++i) {
        ema = ema * per1 + input[i] * per;
        if (i == period-1) {
            ema2 = ema;
        }
        if (i >= period-1) {
            ema2 = ema2 * per1 + ema * per;
            if (i == (period-1) * 2) {
                ema3 = ema2;
            }
            if (i >= (period-1) * 2) {
                ema3 = ema3 * per1 + ema2 * per;
                if (i >= (period-1) * 3) {
                    *output = 3 * ema - 3 * ema2 + ema3;
                    ++output;
                }
            }
        }
    }
    ((void) sizeof ((output - outputs[0] == size - ti_tema_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_tema_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_tema_start(options)", "<stdin>", 7116, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_todeg_start(double const *options) { (void)options; return 0; } int ti_todeg(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = ((in1[i] * (180.0 / 3.14159265358979323846))); } return 0; }
int ti_torad_start(double const *options) { (void)options; return 0; } int ti_torad(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = ((in1[i] * (3.14159265358979323846 / 180.0))); } return 0; }
int ti_tr_start(double const *options) {
    (void)options;
    return 0;
}
int ti_tr(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    (void)options;
    double *output = outputs[0];
    double truerange;
    output[0] = high[0] - low[0];
    int i;
    for (i = 1; i < size; ++i) {
        do{ const double l = low[i]; const double h = high[i]; const double c = close[i-1]; const double ych = fabs(h - c); const double ycl = fabs(l - c); double v = h - l; if (ych > v) v = ych; if (ycl > v) v = ycl; truerange = v;}while(0);
        output[i] = truerange;
    }
    return 0;
}
int ti_trima_start(double const *options) {
    return (int)options[0]-1;
}
int ti_trima(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_trima_start(options)) return 0;
    if (period <= 2) return ti_sma(size, inputs, options, outputs);
    double weights = 1 / (double) ((period%2) ?
        ((period/2+1) * (period/2+1)):
        ((period/2+1) * (period/2)));
    double weight_sum = 0;
    double lead_sum = 0;
    double trail_sum = 0;
    const int lead_period = period%2 ? period/2 : period/2-1;
    const int trail_period = lead_period + 1;
    int i, w = 1;
    for (i = 0; i < period-1; ++i) {
        weight_sum += input[i] * w;
        if (i+1 > period-lead_period) lead_sum += input[i];
        if (i+1 <= trail_period) trail_sum += input[i];
        if (i+1 < trail_period) ++w;
        if (i+1 >= period-lead_period) --w;
    }
    int lsi = (period-1)-lead_period+1;
    int tsi1 = (period-1)-period+1+trail_period;
    int tsi2 = (period-1)-period+1;
    for (i = period-1; i < size; ++i) {
        weight_sum += input[i];
        *output++ = weight_sum * weights;
        lead_sum += input[i];
        weight_sum += lead_sum;
        weight_sum -= trail_sum;
        lead_sum -= input[lsi++];
        trail_sum += input[tsi1++];
        trail_sum -= input[tsi2++];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_trima_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_trima_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_trima_start(options)", "<stdin>", 7326, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_trix_start(double const *options) {
    const int period = (int)options[0];
    return ((period-1)*3)+1;
}
int ti_trix(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_trix_start(options)) return 0;
    const int start = (period*3)-2;
    ((void) sizeof ((start == ti_trix_start(options)) ? 1 : 0), __extension__ ({ if (start == ti_trix_start(options)) ; else __assert_fail ("start == ti_trix_start(options)", "<stdin>", 7369, __extension__ __PRETTY_FUNCTION__); }));
    const double per = 2 / ((double)period + 1);
    double ema1 = input[0];
    double ema2 = 0, ema3 = 0;
    int i;
    for (i = 1; i < start; ++i) {
        ema1 = (input[i]-ema1) * per + ema1;
        if (i == period-1) {
            ema2 = ema1;
        } else if (i > period-1) {
            ema2 = (ema1-ema2) * per + ema2;
            if (i == period * 2 - 2) {
                ema3 = ema2;
            } else if (i > period * 2 - 2) {
                ema3 = (ema2-ema3) * per + ema3;
            }
        }
    }
    for (i = start; i < size; ++i) {
        ema1 = (input[i]-ema1) * per + ema1;
        ema2 = (ema1-ema2) * per + ema2;
        const double last = ema3;
        ema3 = (ema2-ema3) * per + ema3;
        *output++ = (ema3-last)/ema3 * 100.0;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_trix_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_trix_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_trix_start(options)", "<stdin>", 7401, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_trunc_start(double const *options) { (void)options; return 0; } int ti_trunc(int size, double const *const *inputs, double const *options, double *const *outputs) { const double *in1 = inputs[0]; (void)options; double *output = outputs[0]; int i; for (i = 0; i < size; ++i) { output[i] = ((int)(in1[i])); } return 0; }
int ti_tsf_start(double const *options) {
    return (int)options[0]-1;
}
int ti_tsf(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_tsf_start(options)) return 0;
    do { double x = 0; double x2 = 0; double y = 0; double xy = 0; do{}while(0); int i; for (i = 0; i < (period)-1; ++i) { x += i+1; x2 += (i+1)*(i+1); xy += (input)[i] * (i+1); y += (input)[i]; } x += (period); x2 += (period) * (period); const double bd = 1.0 / ((period) * x2 - x * x); for (i = (period)-1; i < (size); ++i) { xy += (input)[i] * (period); y += (input)[i]; const double b = ((period) * xy - x * y) * bd; do { *output++ = b; } while (0); xy -= y; y -= (input)[i-(period)+1]; } } while (0);
    ((void) sizeof ((output - outputs[0] == size - ti_tsf_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_tsf_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_tsf_start(options)", "<stdin>", 7473, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_typprice_start(double const *options) {
    (void)options;
    return 0;
}
int ti_typprice(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    (void)options;
    double *output = outputs[0];
    int i;
    for (i = 0; i < size; ++i) {
        output[i] = (high[i] + low[i] + close[i]) * (1.0/3.0);
    }
    return 0;
}
int ti_ultosc_start(double const *options) {
    return (int)options[2];
}
int ti_ultosc(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int short_period = (int)options[0];
    const int medium_period = (int)options[1];
    const int long_period = (int)options[2];
    double *output = outputs[0];
    if (short_period < 1) return 1;
    if (medium_period < short_period) return 1;
    if (long_period < medium_period) return 1;
    if (size <= ti_ultosc_start(options)) return 0;
    ti_buffer *bp_buf = ti_buffer_new(long_period);
    ti_buffer *r_buf = ti_buffer_new(long_period);
    double bp_short_sum = 0, bp_medium_sum = 0;
    double r_short_sum = 0, r_medium_sum = 0;
    int i;
    for (i = 1; i < size; ++i) {
        const double true_low = ((low[i])<(close[i-1])?(low[i]):(close[i-1]));
        const double true_high = ((high[i])>(close[i-1])?(high[i]):(close[i-1]));
        const double bp = close[i] - true_low;
        const double r = true_high - true_low;
        bp_short_sum += bp;
        bp_medium_sum += bp;
        r_short_sum += r;
        r_medium_sum += r;
        do { if ((bp_buf)->pushes >= (bp_buf)->size) { (bp_buf)->sum -= (bp_buf)->vals[(bp_buf)->index]; } (bp_buf)->sum += (bp); (bp_buf)->vals[(bp_buf)->index] = (bp); (bp_buf)->pushes += 1; (bp_buf)->index = ((bp_buf)->index + 1); if ((bp_buf)->index >= (bp_buf)->size) (bp_buf)->index = 0; } while (0);
        do { if ((r_buf)->pushes >= (r_buf)->size) { (r_buf)->sum -= (r_buf)->vals[(r_buf)->index]; } (r_buf)->sum += (r); (r_buf)->vals[(r_buf)->index] = (r); (r_buf)->pushes += 1; (r_buf)->index = ((r_buf)->index + 1); if ((r_buf)->index >= (r_buf)->size) (r_buf)->index = 0; } while (0);
        if (i > short_period) {
            int short_index = bp_buf->index - short_period - 1;
            if (short_index < 0) short_index += long_period;
            bp_short_sum -= bp_buf->vals[short_index];
            r_short_sum -= r_buf->vals[short_index];
            if (i > medium_period) {
                int medium_index = bp_buf->index - medium_period - 1;
                if (medium_index < 0) medium_index += long_period;
                bp_medium_sum -= bp_buf->vals[medium_index];
                r_medium_sum -= r_buf->vals[medium_index];
            }
        }
        if (i >= long_period) {
            const double first = 4 * bp_short_sum / r_short_sum;
            const double second = 2 * bp_medium_sum / r_medium_sum;
            const double third = 1 * bp_buf->sum / r_buf->sum;
            const double ult = (first + second + third) * 100.0 / 7.0;
            *output++ = ult;
        }
    }
    ti_buffer_free(bp_buf);
    ti_buffer_free(r_buf);
    ((void) sizeof ((output - outputs[0] == size - ti_ultosc_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_ultosc_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_ultosc_start(options)", "<stdin>", 7624, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_var_start(double const *options) {
    return (int)options[0]-1;
}
int ti_var(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    const double scale = 1.0 / period;
    if (period < 1) return 1;
    if (size <= ti_var_start(options)) return 0;
    double sum = 0;
    double sum2 = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i];
        sum2 += input[i] * input[i];
    }
    *output++ = sum2 * scale - (sum * scale) * (sum * scale);
    for (i = period; i < size; ++i) {
        sum += input[i];
        sum2 += input[i] * input[i];
        sum -= input[i-period];
        sum2 -= input[i-period] * input[i-period];
        *output++ = sum2 * scale - (sum * scale) * (sum * scale);
    }
    ((void) sizeof ((output - outputs[0] == size - ti_var_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_var_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_var_start(options)", "<stdin>", 7690, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_vhf_start(double const *options) {
    return (int)options[0];
}
int ti_vhf(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *in = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_vhf_start(options)) return 0;
    int trail = 1, maxi = -1, mini = -1;
    double max = in[0], min = in[0];
    double bar;
    double sum = 0;
    int i, j;
    double yc = in[0];
    double c;
    for (i = 1; i < period; ++i) {
        c = in[i];
        sum += fabs(c - yc);
        yc = c;
    }
    for (i = period; i < size; ++i, ++trail) {
        c = in[i];
        sum += fabs(c - yc);
        yc = c;
        if (i > period) {
            sum -= fabs(in[i-period] - in[i-period-1]);
        }
        bar = c;
        if (maxi < trail) {
            maxi = trail;
            max = in[maxi];
            j = trail;
            while(++j <= i) {
                bar = in[j];
                if (bar >= max) {
                    max = bar;
                    maxi = j;
                }
            }
        } else if (bar >= max) {
            maxi = i;
            max = bar;
        }
        bar = c;
        if (mini < trail) {
            mini = trail;
            min = in[mini];
            j = trail;
            while(++j <= i) {
                bar = in[j];
                if (bar <= min) {
                    min = bar;
                    mini = j;
                }
            }
        } else if (bar <= min) {
            mini = i;
            min = bar;
        }
        *output++ = fabs(max - min) / sum;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_vhf_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_vhf_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_vhf_start(options)", "<stdin>", 7797, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_vidya_start(double const *options) {
    return ((int)(options[1])) - 2;
}
int ti_vidya(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int short_period = (int)options[0];
    const int long_period = (int)options[1];
    const double alpha = options[2];
    double *output = outputs[0];
    const double short_div = 1.0 / short_period;
    const double long_div = 1.0 / long_period;
    if (short_period < 1) return 1;
    if (long_period < short_period) return 1;
    if (long_period < 2) return 1;
    if (alpha < 0.0 || alpha > 1.0) return 1;
    if (size <= ti_vidya_start(options)) return 0;
    double short_sum = 0;
    double short_sum2 = 0;
    double long_sum = 0;
    double long_sum2 = 0;
    int i;
    for (i = 0; i < long_period; ++i) {
        long_sum += input[i];
        long_sum2 += input[i] * input[i];
        if (i >= long_period - short_period) {
            short_sum += input[i];
            short_sum2 += input[i] * input[i];
        }
    }
    double val = input[long_period-2];
    *output++ = val;
    if (long_period - 1 < size) {
        double short_stddev = sqrt(short_sum2 * short_div - (short_sum * short_div) * (short_sum * short_div));
        double long_stddev = sqrt(long_sum2 * long_div - (long_sum * long_div) * (long_sum * long_div));
        double k = short_stddev / long_stddev;
        if (k != k) k = 0;
        k *= alpha;
        val = (input[long_period-1]-val) * k + val;
        *output++ = val;
    }
    for (i = long_period; i < size; ++i) {
        long_sum += input[i];
        long_sum2 += input[i] * input[i];
        short_sum += input[i];
        short_sum2 += input[i] * input[i];
        long_sum -= input[i-long_period];
        long_sum2 -= input[i-long_period] * input[i-long_period];
        short_sum -= input[i-short_period];
        short_sum2 -= input[i-short_period] * input[i-short_period];
        {
            double short_stddev = sqrt(short_sum2 * short_div - (short_sum * short_div) * (short_sum * short_div));
            double long_stddev = sqrt(long_sum2 * long_div - (long_sum * long_div) * (long_sum * long_div));
            double k = short_stddev / long_stddev;
            if (k != k) k = 0;
            k *= alpha;
            val = (input[i]-val) * k + val;
            *output++ = val;
        }
    }
    ((void) sizeof ((output - outputs[0] == size - ti_vidya_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_vidya_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_vidya_start(options)", "<stdin>", 7904, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_volatility_start(double const *options) {
    return (int)options[0];
}
int ti_volatility(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *output = outputs[0];
    const int period = (int)options[0];
    const double scale = 1.0 / period;
    const double annual = sqrt(252);
    if (period < 1) return 1;
    if (size <= ti_volatility_start(options)) return 0;
    double sum = 0;
    double sum2 = 0;
    int i;
    for (i = 1; i <= period; ++i) {
        const double c = (input[i]/input[i-1]-1.0);
        sum += c;
        sum2 += c * c;
    }
    *output++ = sqrt(sum2 * scale - (sum * scale) * (sum * scale)) * annual;
    for (i = period+1; i < size; ++i) {
        const double c = (input[i]/input[i-1]-1.0);
        sum += c;
        sum2 += c * c;
        const double cp = (input[i-period]/input[i-period-1]-1.0);
        sum -= cp;
        sum2 -= cp * cp;
        *output++ = sqrt(sum2 * scale - (sum * scale) * (sum * scale)) * annual;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_volatility_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_volatility_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_volatility_start(options)", "<stdin>", 7975, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_vosc_start(double const *options) {
    return (int)options[1]-1;
}
int ti_vosc(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    double *output = outputs[0];
    const int short_period = (int)options[0];
    const int long_period = (int)options[1];
    const double short_div = 1.0 / short_period;
    const double long_div = 1.0 / long_period;
    if (short_period < 1) return 1;
    if (long_period < short_period) return 1;
    if (size <= ti_vosc_start(options)) return 0;
    double short_sum = 0;
    double long_sum = 0;
    int i;
    for (i = 0; i < long_period; ++i) {
        if (i >= (long_period - short_period)) {
            short_sum += input[i];
        }
        long_sum += input[i];
    }
    {
        const double savg = short_sum * short_div;
        const double lavg = long_sum * long_div;
        *output++ = 100.0 * (savg - lavg) / lavg;
    }
    for (i = long_period; i < size; ++i) {
        short_sum += input[i];
        short_sum -= input[i-short_period];
        long_sum += input[i];
        long_sum -= input[i-long_period];
        const double savg = short_sum * short_div;
        const double lavg = long_sum * long_div;
        *output++ = 100.0 * (savg - lavg) / lavg;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_vosc_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_vosc_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_vosc_start(options)", "<stdin>", 8052, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_vwma_start(double const *options) {
    return (int)options[0]-1;
}
int ti_vwma(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const double *volume = inputs[1];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_vwma_start(options)) return 0;
    double sum = 0;
    double vsum = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i] * volume[i];
        vsum += volume[i];
    }
    *output++ = sum / vsum;
    for (i = period; i < size; ++i) {
        sum += input[i] * volume[i];
        sum -= input[i-period] * volume[i-period];
        vsum += volume[i];
        vsum -= volume[i-period];
        *output++ = sum / vsum;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_vwma_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_vwma_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_vwma_start(options)", "<stdin>", 8116, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_wad_start(double const *options) {
    (void)options;
    return 1;
}
int ti_wad(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    (void)options;
    if (size <= ti_wad_start(options)) return 0;
    double *output = outputs[0];
    double sum = 0;
    double yc = close[0];
    int i;
    for (i = 1; i < size; ++i) {
        const double c = close[i];
        if (c > yc) {
            sum += c - ((yc)<(low[i])?(yc):(low[i]));
        } else if (c < yc) {
            sum += c - ((yc)>(high[i])?(yc):(high[i]));
        } else {
        }
        *output++ = sum;
        yc = close[i];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_wad_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_wad_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_wad_start(options)", "<stdin>", 8182, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_wcprice_start(double const *options) {
    (void)options;
    return 0;
}
int ti_wcprice(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    (void)options;
    double *output = outputs[0];
    int i;
    for (i = 0; i < size; ++i) {
        output[i] = (high[i] + low[i] + close[i] + close[i]) * 0.25;
    }
    return 0;
}
int ti_wilders_start(double const *options) {
    return (int)options[0]-1;
}
int ti_wilders(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_wilders_start(options)) return 0;
    const double per = 1.0 / ((double)period);
    double sum = 0;
    int i;
    for (i = 0; i < period; ++i) {
        sum += input[i];
    }
    double val = sum / period;
    *output++ = val;
    for (i = period; i < size; ++i) {
        val = (input[i]-val) * per + val;
        *output++ = val;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_wilders_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_wilders_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_wilders_start(options)", "<stdin>", 8289, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_willr_start(double const *options) {
    return (int)options[0]-1;
}
int ti_willr(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *high = inputs[0];
    const double *low = inputs[1];
    const double *close = inputs[2];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_willr_start(options)) return 0;
    int trail = 0, maxi = -1, mini = -1;
    double max = high[0];
    double min = low[0];
    double bar;
    int i, j;
    for (i = period-1; i < size; ++i, ++trail) {
        bar = high[i];
        if (maxi < trail) {
            maxi = trail;
            max = high[maxi];
            j = trail;
            while(++j <= i) {
                bar = high[j];
                if (bar >= max) {
                    max = bar;
                    maxi = j;
                }
            }
        } else if (bar >= max) {
            maxi = i;
            max = bar;
        }
        bar = low[i];
        if (mini < trail) {
            mini = trail;
            min = low[mini];
            j = trail;
            while(++j <= i) {
                bar = low[j];
                if (bar <= min) {
                    min = bar;
                    mini = j;
                }
            }
        } else if (bar <= min) {
            mini = i;
            min = bar;
        }
        const double highlow = (max - min);
        const double r = highlow == 0.0 ? 0.0 : -100 * ((max - close[i]) / highlow);
        *output++ = r;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_willr_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_willr_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_willr_start(options)", "<stdin>", 8385, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_wma_start(double const *options) {
    return (int)options[0]-1;
}
int ti_wma(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_wma_start(options)) return 0;
    const double weights = period * (period+1) / 2;
    double sum = 0;
    double weight_sum = 0;
    int i;
    for (i = 0; i < period-1; ++i) {
        weight_sum += input[i] * (i+1);
        sum += input[i];
    }
    for (i = period-1; i < size; ++i) {
        weight_sum += input[i] * period;
        sum += input[i];
        *output++ = weight_sum / weights;
        weight_sum -= sum;
        sum -= input[i-period+1];
    }
    ((void) sizeof ((output - outputs[0] == size - ti_wma_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_wma_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_wma_start(options)", "<stdin>", 8454, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
int ti_zlema_start(double const *options) {
    return ((int)options[0] - 1) / 2 - 1;
}
int ti_zlema(int size, double const *const *inputs, double const *options, double *const *outputs) {
    const double *input = inputs[0];
    const int period = (int)options[0];
    const int lag = (period - 1) / 2;
    double *output = outputs[0];
    if (period < 1) return 1;
    if (size <= ti_zlema_start(options)) return 0;
    const double per = 2 / ((double)period + 1);
    double val = input[lag-1];
    *output++ = val;
    int i;
    for (i = lag; i < size; ++i) {
        double c = input[i];
        double l = input[i-lag];
        val = ((c + (c-l))-val) * per + val;
        *output++ = val;
    }
    ((void) sizeof ((output - outputs[0] == size - ti_zlema_start(options)) ? 1 : 0), __extension__ ({ if (output - outputs[0] == size - ti_zlema_start(options)) ; else __assert_fail ("output - outputs[0] == size - ti_zlema_start(options)", "<stdin>", 8513, __extension__ __PRETTY_FUNCTION__); }));
    return 0;
}
ti_buffer *ti_buffer_new(int size) {
    const int s = (int)sizeof(ti_buffer) + (size-1) * (int)sizeof(double);
    ti_buffer *ret = (ti_buffer*)malloc((unsigned int)s);
    ret->size = size;
    ret->pushes = 0;
    ret->index = 0;
    ret->sum = 0;
    return ret;
}
void ti_buffer_free(ti_buffer *buffer) {
    free(buffer);
}
const char* ti_version() {return "0.8.4";}
long int ti_build() {return 1537377628;}
struct ti_indicator_info ti_indicators[] = {
    {"abs", "Vector Absolute Value", ti_abs_start, ti_abs, 4, 1, 0, 1, {"real",0}, {"",0}, {"abs",0}},
    {"acos", "Vector Arccosine", ti_acos_start, ti_acos, 4, 1, 0, 1, {"real",0}, {"",0}, {"acos",0}},
    {"ad", "Accumulation/Distribution Line", ti_ad_start, ti_ad, 2, 4, 0, 1, {"high","low","close","volume",0}, {"",0}, {"ad",0}},
    {"add", "Vector Addition", ti_add_start, ti_add, 4, 2, 0, 1, {"real","real",0}, {"",0}, {"add",0}},
    {"adosc", "Accumulation/Distribution Oscillator", ti_adosc_start, ti_adosc, 2, 4, 2, 1, {"high","low","close","volume",0}, {"short period","long period",0}, {"adosc",0}},
    {"adx", "Average Directional Movement Index", ti_adx_start, ti_adx, 2, 3, 1, 1, {"high","low","close",0}, {"period",0}, {"dx",0}},
    {"adxr", "Average Directional Movement Rating", ti_adxr_start, ti_adxr, 2, 3, 1, 1, {"high","low","close",0}, {"period",0}, {"dx",0}},
    {"ao", "Awesome Oscillator", ti_ao_start, ti_ao, 2, 2, 0, 1, {"high","low",0}, {"",0}, {"ao",0}},
    {"apo", "Absolute Price Oscillator", ti_apo_start, ti_apo, 2, 1, 2, 1, {"real",0}, {"short period","long period",0}, {"apo",0}},
    {"aroon", "Aroon", ti_aroon_start, ti_aroon, 2, 2, 1, 2, {"high","low",0}, {"period",0}, {"aroon_down","aroon_up",0}},
    {"aroonosc", "Aroon Oscillator", ti_aroonosc_start, ti_aroonosc, 2, 2, 1, 1, {"high","low",0}, {"period",0}, {"aroonosc",0}},
    {"asin", "Vector Arcsine", ti_asin_start, ti_asin, 4, 1, 0, 1, {"real",0}, {"",0}, {"asin",0}},
    {"atan", "Vector Arctangent", ti_atan_start, ti_atan, 4, 1, 0, 1, {"real",0}, {"",0}, {"atan",0}},
    {"atr", "Average True Range", ti_atr_start, ti_atr, 2, 3, 1, 1, {"high","low","close",0}, {"period",0}, {"atr",0}},
    {"avgprice", "Average Price", ti_avgprice_start, ti_avgprice, 1, 4, 0, 1, {"open","high","low","close",0}, {"",0}, {"avgprice",0}},
    {"bbands", "Bollinger Bands", ti_bbands_start, ti_bbands, 1, 1, 2, 3, {"real",0}, {"period","stddev",0}, {"bbands_lower","bbands_middle","bbands_upper",0}},
    {"bop", "Balance of Power", ti_bop_start, ti_bop, 2, 4, 0, 1, {"open","high","low","close",0}, {"",0}, {"bop",0}},
    {"cci", "Commodity Channel Index", ti_cci_start, ti_cci, 2, 3, 1, 1, {"high","low","close",0}, {"period",0}, {"cci",0}},
    {"ceil", "Vector Ceiling", ti_ceil_start, ti_ceil, 4, 1, 0, 1, {"real",0}, {"",0}, {"ceil",0}},
    {"cmo", "Chande Momentum Oscillator", ti_cmo_start, ti_cmo, 2, 1, 1, 1, {"real",0}, {"period",0}, {"cmo",0}},
    {"cos", "Vector Cosine", ti_cos_start, ti_cos, 4, 1, 0, 1, {"real",0}, {"",0}, {"cos",0}},
    {"cosh", "Vector Hyperbolic Cosine", ti_cosh_start, ti_cosh, 4, 1, 0, 1, {"real",0}, {"",0}, {"cosh",0}},
    {"crossany", "Crossany", ti_crossany_start, ti_crossany, 3, 2, 0, 1, {"real","real",0}, {"",0}, {"crossany",0}},
    {"crossover", "Crossover", ti_crossover_start, ti_crossover, 3, 2, 0, 1, {"real","real",0}, {"",0}, {"crossover",0}},
    {"cvi", "Chaikins Volatility", ti_cvi_start, ti_cvi, 2, 2, 1, 1, {"high","low",0}, {"period",0}, {"cvi",0}},
    {"decay", "Linear Decay", ti_decay_start, ti_decay, 3, 1, 1, 1, {"real",0}, {"period",0}, {"decay",0}},
    {"dema", "Double Exponential Moving Average", ti_dema_start, ti_dema, 1, 1, 1, 1, {"real",0}, {"period",0}, {"dema",0}},
    {"di", "Directional Indicator", ti_di_start, ti_di, 2, 3, 1, 2, {"high","low","close",0}, {"period",0}, {"plus_di","minus_di",0}},
    {"div", "Vector Division", ti_div_start, ti_div, 4, 2, 0, 1, {"real","real",0}, {"",0}, {"div",0}},
    {"dm", "Directional Movement", ti_dm_start, ti_dm, 2, 2, 1, 2, {"high","low",0}, {"period",0}, {"plus_dm","minus_dm",0}},
    {"dpo", "Detrended Price Oscillator", ti_dpo_start, ti_dpo, 2, 1, 1, 1, {"real",0}, {"period",0}, {"dpo",0}},
    {"dx", "Directional Movement Index", ti_dx_start, ti_dx, 2, 3, 1, 1, {"high","low","close",0}, {"period",0}, {"dx",0}},
    {"edecay", "Exponential Decay", ti_edecay_start, ti_edecay, 3, 1, 1, 1, {"real",0}, {"period",0}, {"edecay",0}},
    {"ema", "Exponential Moving Average", ti_ema_start, ti_ema, 1, 1, 1, 1, {"real",0}, {"period",0}, {"ema",0}},
    {"emv", "Ease of Movement", ti_emv_start, ti_emv, 2, 3, 0, 1, {"high","low","volume",0}, {"",0}, {"emv",0}},
    {"exp", "Vector Exponential", ti_exp_start, ti_exp, 4, 1, 0, 1, {"real",0}, {"",0}, {"exp",0}},
    {"fisher", "Fisher Transform", ti_fisher_start, ti_fisher, 2, 2, 1, 2, {"high","low",0}, {"period",0}, {"fisher","fisher_signal",0}},
    {"floor", "Vector Floor", ti_floor_start, ti_floor, 4, 1, 0, 1, {"real",0}, {"",0}, {"floor",0}},
    {"fosc", "Forecast Oscillator", ti_fosc_start, ti_fosc, 2, 1, 1, 1, {"real",0}, {"period",0}, {"fosc",0}},
    {"hma", "Hull Moving Average", ti_hma_start, ti_hma, 1, 1, 1, 1, {"real",0}, {"period",0}, {"hma",0}},
    {"kama", "Kaufman Adaptive Moving Average", ti_kama_start, ti_kama, 1, 1, 1, 1, {"real",0}, {"period",0}, {"kama",0}},
    {"kvo", "Klinger Volume Oscillator", ti_kvo_start, ti_kvo, 2, 4, 2, 1, {"high","low","close","volume",0}, {"short period","long period",0}, {"kvo",0}},
    {"lag", "Lag", ti_lag_start, ti_lag, 3, 1, 1, 1, {"real",0}, {"period",0}, {"lag",0}},
    {"linreg", "Linear Regression", ti_linreg_start, ti_linreg, 1, 1, 1, 1, {"real",0}, {"period",0}, {"linreg",0}},
    {"linregintercept", "Linear Regression Intercept", ti_linregintercept_start, ti_linregintercept, 2, 1, 1, 1, {"real",0}, {"period",0}, {"linregintercept",0}},
    {"linregslope", "Linear Regression Slope", ti_linregslope_start, ti_linregslope, 2, 1, 1, 1, {"real",0}, {"period",0}, {"linregslope",0}},
    {"ln", "Vector Natural Log", ti_ln_start, ti_ln, 4, 1, 0, 1, {"real",0}, {"",0}, {"ln",0}},
    {"log10", "Vector Base-10 Log", ti_log10_start, ti_log10, 4, 1, 0, 1, {"real",0}, {"",0}, {"log10",0}},
    {"macd", "Moving Average Convergence/Divergence", ti_macd_start, ti_macd, 2, 1, 3, 3, {"real",0}, {"short period","long period","signal period",0}, {"macd","macd_signal","macd_histogram",0}},
    {"marketfi", "Market Facilitation Index", ti_marketfi_start, ti_marketfi, 2, 3, 0, 1, {"high","low","volume",0}, {"",0}, {"marketfi",0}},
    {"mass", "Mass Index", ti_mass_start, ti_mass, 2, 2, 1, 1, {"high","low",0}, {"period",0}, {"mass",0}},
    {"max", "Maximum In Period", ti_max_start, ti_max, 3, 1, 1, 1, {"real",0}, {"period",0}, {"max",0}},
    {"md", "Mean Deviation Over Period", ti_md_start, ti_md, 3, 1, 1, 1, {"real",0}, {"period",0}, {"md",0}},
    {"medprice", "Median Price", ti_medprice_start, ti_medprice, 1, 2, 0, 1, {"high","low",0}, {"",0}, {"medprice",0}},
    {"mfi", "Money Flow Index", ti_mfi_start, ti_mfi, 2, 4, 1, 1, {"high","low","close","volume",0}, {"period",0}, {"mfi",0}},
    {"min", "Minimum In Period", ti_min_start, ti_min, 3, 1, 1, 1, {"real",0}, {"period",0}, {"min",0}},
    {"mom", "Momentum", ti_mom_start, ti_mom, 2, 1, 1, 1, {"real",0}, {"period",0}, {"mom",0}},
    {"msw", "Mesa Sine Wave", ti_msw_start, ti_msw, 2, 1, 1, 2, {"real",0}, {"period",0}, {"msw_sine","msw_lead",0}},
    {"mul", "Vector Multiplication", ti_mul_start, ti_mul, 4, 2, 0, 1, {"real","real",0}, {"",0}, {"mul",0}},
    {"natr", "Normalized Average True Range", ti_natr_start, ti_natr, 2, 3, 1, 1, {"high","low","close",0}, {"period",0}, {"natr",0}},
    {"nvi", "Negative Volume Index", ti_nvi_start, ti_nvi, 2, 2, 0, 1, {"close","volume",0}, {"",0}, {"nvi",0}},
    {"obv", "On Balance Volume", ti_obv_start, ti_obv, 2, 2, 0, 1, {"close","volume",0}, {"",0}, {"obv",0}},
    {"ppo", "Percentage Price Oscillator", ti_ppo_start, ti_ppo, 2, 1, 2, 1, {"real",0}, {"short period","long period",0}, {"ppo",0}},
    {"psar", "Parabolic SAR", ti_psar_start, ti_psar, 1, 2, 2, 1, {"high","low",0}, {"acceleration factor step","acceleration factor maximum",0}, {"psar",0}},
    {"pvi", "Positive Volume Index", ti_pvi_start, ti_pvi, 2, 2, 0, 1, {"close","volume",0}, {"",0}, {"pvi",0}},
    {"qstick", "Qstick", ti_qstick_start, ti_qstick, 2, 2, 1, 1, {"open","close",0}, {"period",0}, {"qstick",0}},
    {"roc", "Rate of Change", ti_roc_start, ti_roc, 2, 1, 1, 1, {"real",0}, {"period",0}, {"roc",0}},
    {"rocr", "Rate of Change Ratio", ti_rocr_start, ti_rocr, 2, 1, 1, 1, {"real",0}, {"period",0}, {"rocr",0}},
    {"round", "Vector Round", ti_round_start, ti_round, 4, 1, 0, 1, {"real",0}, {"",0}, {"round",0}},
    {"rsi", "Relative Strength Index", ti_rsi_start, ti_rsi, 2, 1, 1, 1, {"real",0}, {"period",0}, {"rsi",0}},
    {"sin", "Vector Sine", ti_sin_start, ti_sin, 4, 1, 0, 1, {"real",0}, {"",0}, {"sin",0}},
    {"sinh", "Vector Hyperbolic Sine", ti_sinh_start, ti_sinh, 4, 1, 0, 1, {"real",0}, {"",0}, {"sinh",0}},
    {"sma", "Simple Moving Average", ti_sma_start, ti_sma, 1, 1, 1, 1, {"real",0}, {"period",0}, {"sma",0}},
    {"sqrt", "Vector Square Root", ti_sqrt_start, ti_sqrt, 4, 1, 0, 1, {"real",0}, {"",0}, {"sqrt",0}},
    {"stddev", "Standard Deviation Over Period", ti_stddev_start, ti_stddev, 3, 1, 1, 1, {"real",0}, {"period",0}, {"stddev",0}},
    {"stderr", "Standard Error Over Period", ti_stderr_start, ti_stderr, 3, 1, 1, 1, {"real",0}, {"period",0}, {"stderr",0}},
    {"stoch", "Stochastic Oscillator", ti_stoch_start, ti_stoch, 2, 3, 3, 2, {"high","low","close",0}, {"%k period","%k slowing period","%d period",0}, {"stoch_k","stoch_d",0}},
    {"stochrsi", "Stochastic RSI", ti_stochrsi_start, ti_stochrsi, 2, 1, 1, 1, {"real",0}, {"period",0}, {"stochrsi",0}},
    {"sub", "Vector Subtraction", ti_sub_start, ti_sub, 4, 2, 0, 1, {"real","real",0}, {"",0}, {"sub",0}},
    {"sum", "Sum Over Period", ti_sum_start, ti_sum, 3, 1, 1, 1, {"real",0}, {"period",0}, {"sum",0}},
    {"tan", "Vector Tangent", ti_tan_start, ti_tan, 4, 1, 0, 1, {"real",0}, {"",0}, {"tan",0}},
    {"tanh", "Vector Hyperbolic Tangent", ti_tanh_start, ti_tanh, 4, 1, 0, 1, {"real",0}, {"",0}, {"tanh",0}},
    {"tema", "Triple Exponential Moving Average", ti_tema_start, ti_tema, 1, 1, 1, 1, {"real",0}, {"period",0}, {"tema",0}},
    {"todeg", "Vector Degree Conversion", ti_todeg_start, ti_todeg, 4, 1, 0, 1, {"real",0}, {"",0}, {"degrees",0}},
    {"torad", "Vector Radian Conversion", ti_torad_start, ti_torad, 4, 1, 0, 1, {"real",0}, {"",0}, {"radians",0}},
    {"tr", "True Range", ti_tr_start, ti_tr, 2, 3, 0, 1, {"high","low","close",0}, {"",0}, {"tr",0}},
    {"trima", "Triangular Moving Average", ti_trima_start, ti_trima, 1, 1, 1, 1, {"real",0}, {"period",0}, {"trima",0}},
    {"trix", "Trix", ti_trix_start, ti_trix, 2, 1, 1, 1, {"real",0}, {"period",0}, {"trix",0}},
    {"trunc", "Vector Truncate", ti_trunc_start, ti_trunc, 4, 1, 0, 1, {"real",0}, {"",0}, {"trunc",0}},
    {"tsf", "Time Series Forecast", ti_tsf_start, ti_tsf, 1, 1, 1, 1, {"real",0}, {"period",0}, {"tsf",0}},
    {"typprice", "Typical Price", ti_typprice_start, ti_typprice, 1, 3, 0, 1, {"high","low","close",0}, {"",0}, {"typprice",0}},
    {"ultosc", "Ultimate Oscillator", ti_ultosc_start, ti_ultosc, 2, 3, 3, 1, {"high","low","close",0}, {"short period","medium period","long period",0}, {"ultosc",0}},
    {"var", "Variance Over Period", ti_var_start, ti_var, 3, 1, 1, 1, {"real",0}, {"period",0}, {"var",0}},
    {"vhf", "Vertical Horizontal Filter", ti_vhf_start, ti_vhf, 2, 1, 1, 1, {"real",0}, {"period",0}, {"vhf",0}},
    {"vidya", "Variable Index Dynamic Average", ti_vidya_start, ti_vidya, 1, 1, 3, 1, {"real",0}, {"short period","long period","alpha",0}, {"vidya",0}},
    {"volatility", "Annualized Historical Volatility", ti_volatility_start, ti_volatility, 2, 1, 1, 1, {"real",0}, {"period",0}, {"volatility",0}},
    {"vosc", "Volume Oscillator", ti_vosc_start, ti_vosc, 2, 1, 2, 1, {"volume",0}, {"short period","long period",0}, {"vosc",0}},
    {"vwma", "Volume Weighted Moving Average", ti_vwma_start, ti_vwma, 1, 2, 1, 1, {"close","volume",0}, {"period",0}, {"vwma",0}},
    {"wad", "Williams Accumulation/Distribution", ti_wad_start, ti_wad, 2, 3, 0, 1, {"high","low","close",0}, {"",0}, {"wad",0}},
    {"wcprice", "Weighted Close Price", ti_wcprice_start, ti_wcprice, 1, 3, 0, 1, {"high","low","close",0}, {"",0}, {"wcprice",0}},
    {"wilders", "Wilders Smoothing", ti_wilders_start, ti_wilders, 1, 1, 1, 1, {"real",0}, {"period",0}, {"wilders",0}},
    {"willr", "Williams %R", ti_willr_start, ti_willr, 2, 3, 1, 1, {"high","low","close",0}, {"period",0}, {"willr",0}},
    {"wma", "Weighted Moving Average", ti_wma_start, ti_wma, 1, 1, 1, 1, {"real",0}, {"period",0}, {"wma",0}},
    {"zlema", "Zero-Lag Exponential Moving Average", ti_zlema_start, ti_zlema, 1, 1, 1, 1, {"real",0}, {"period",0}, {"zlema",0}},
    {0,0,0,0,0,0,0,0,{0,0},{0,0},{0,0}}
};
const ti_indicator_info *ti_find_indicator(const char *name) {
    int imin = 0;
    int imax = sizeof(ti_indicators) / sizeof(ti_indicator_info) - 2;
    while (imax >= imin) {
        const int i = (imin + ((imax-imin)/2));
        const int c = strcmp(name, ti_indicators[i].name);
        if (c == 0) {
            return ti_indicators + i;
        } else if (c > 0) {
            imin = i + 1;
        } else {
            imax = i - 1;
        }
    }
    return 0;
}
int ti_find_indicator_valid(int8_t name[16])
{
    name[15] = 0;
    return ti_find_indicator((const char *)name) != 0;
}
