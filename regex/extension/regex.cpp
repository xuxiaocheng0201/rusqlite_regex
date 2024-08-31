#include "utils.h"
#include "regex.h"
#include "rust.h"

SQLITE_EXTENSION_INIT3

void regex_is_match(sqlite3_context *pCtx, const int nVal, sqlite3_value **apVal) {
    check_n_arg(regex_is_match, 2);
    const auto pattern = reinterpret_cast<const char *>(sqlite3_value_text(apVal[0]));
    const auto haystack = reinterpret_cast<const char *>(sqlite3_value_text(apVal[1]));
    regex_compile(pattern);
    const auto matched = rust_regex_is_match(re, haystack);
    sqlite3_result_int(pCtx, static_cast<int>(matched));
    regex_free();
}
