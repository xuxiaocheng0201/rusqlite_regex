#ifndef UTILS_H
#define UTILS_H

#include "sqlite3ext.h"

#define check_n_arg(name, n) do { \
    if (nVal != n) { \
        const char *err = sqlite3_mprintf("The arguments count of %s function is wrong. expected %d but got %d.", name, n, nVal); \
        sqlite3_result_error(pCtx, err, -1); \
        sqlite3_free((void *) err); \
        return; \
    } \
} while (0)


extern void (*free_rust_string)(void*);

#define regex_compile(pattern) \
    const auto compiler = rust_regex_compile(pattern); \
    if (compiler -> error != nullptr) { \
        const char *err = sqlite3_mprintf("Regex compile error: %s", compiler -> error); \
        sqlite3_result_error(pCtx, err, -1); \
        sqlite3_free((void *) err); \
        rust_regex_free_compile(compiler); \
        return; \
    } \
    if (compiler -> re == nullptr) { \
        sqlite3_result_error(pCtx, "internal error: neither regex nor error returned by rust_regex_compile.", -1); \
        rust_regex_free_compile(compiler); \
        return; \
    } \
    const auto re = compiler -> re
# define regex_free() \
    rust_regex_free_compile(compiler)

#endif //UTILS_H
