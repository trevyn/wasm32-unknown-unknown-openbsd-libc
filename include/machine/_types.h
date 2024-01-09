/* clang -target wasm32-unknown-unknown -x c /dev/null -dM -E */

typedef __INTPTR_TYPE__ __intptr_t;
typedef __PTRDIFF_TYPE__ __ptrdiff_t;
typedef __WCHAR_TYPE__ __wchar_t;
typedef __WINT_TYPE__ __wint_t;
typedef __INT8_TYPE__ __int8_t;
typedef __UINT8_TYPE__ __uint8_t;
typedef __INT16_TYPE__ __int16_t;
typedef __UINT16_TYPE__ __uint16_t;
typedef __INT32_TYPE__ __int32_t;
typedef __UINT32_TYPE__ __uint32_t;
typedef __INT64_TYPE__ __int64_t;
typedef __UINT64_TYPE__ __uint64_t;

/* Register size */
typedef long __register_t;

/* VM system types */
typedef unsigned long __vaddr_t;
typedef unsigned long __paddr_t;
typedef unsigned long __vsize_t;
typedef unsigned long __psize_t;

typedef unsigned long __size_t;
typedef long __ssize_t;

typedef float __float_t;
typedef double __double_t;

struct __va_list_tag;
typedef struct __va_list_tag *__va_list;
