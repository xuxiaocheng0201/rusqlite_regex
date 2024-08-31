#include "utils.h"
#include "meta.h"
#include "rust.h"

SQLITE_EXTENSION_INIT3

void regex_version(sqlite3_context *pCtx, const int nVal, sqlite3_value **) {
    check_n_arg("regex_version", 0);
    const char *version = rust_regex_version();
    sqlite3_result_text(pCtx, version, -1, free_rust_string);
}
