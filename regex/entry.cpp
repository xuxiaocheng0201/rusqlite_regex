#include "meta.h"

SQLITE_EXTENSION_INIT1

#define create_function(name, nArg) \
    rc = sqlite3_create_function_v2(db, #name, nArg, SQLITE_UTF8 | SQLITE_DETERMINISTIC, nullptr, &(name), nullptr, nullptr, nullptr); \
    if (rc != SQLITE_OK) { \
        const char *msg = sqlite3_errmsg(db); \
        if (msg == nullptr) \
            *pzErrMsg = sqlite3_mprintf("Failed to create function %s (%d).", #name, rc); \
        else \
            *pzErrMsg = sqlite3_mprintf("Failed to create function %s: (%d) %s", #name, rc, msg); \
        return rc; \
    }


int sqlite3_rusqliteregex_init(sqlite3 *db, char **pzErrMsg, const sqlite3_api_routines *pApi) {
    int rc = SQLITE_OK;
    SQLITE_EXTENSION_INIT2(pApi)

    create_function(regex_version, 0);

    return rc;
}

#undef create_function
