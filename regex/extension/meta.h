#ifndef REGEX_SQLITE_H
#define REGEX_SQLITE_H

#include "sqlite3ext.h"

extern "C" void regex_version(sqlite3_context *pCtx, int nVal, sqlite3_value **apVal);

#endif  // REGEX_SQLITE_H
