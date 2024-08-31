#ifndef REGEX_H
#define REGEX_H

#include "sqlite3ext.h"

extern "C" void regex_is_match(sqlite3_context *pCtx, int nVal, sqlite3_value **apVal);

#endif //REGEX_H
