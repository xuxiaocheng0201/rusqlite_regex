#include "utils.h"
#include "rust.h"

void free_rust_string_(void *ptr) {
    rust_free_string(static_cast<const char *>(ptr));
}

void (*free_rust_string)(void*) = free_rust_string_;
