#ifndef UTILS_H
#define UTILS_H

#include "sqlite3ext.h"

#define check_n_arg(name, n) \
    if (nVal != n) { \
        const char *err = sqlite3_mprintf("The arguments count of %s function is wrong. expected %d but got %d.", name, n, nVal); \
        sqlite3_result_error(pCtx, err, -1); \
        sqlite3_free((void *) err); \
        return; \
    }

#include "rust.h"

inline void free_rust_string_(void *ptr) {
    free_string(static_cast<const char *>(ptr));
}

void (*free_rust_string)(void*) = free_rust_string_;

#endif //UTILS_H
