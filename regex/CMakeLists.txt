cmake_minimum_required(VERSION 3.29)
project(regex)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)
include_directories(contrib)
include_directories(extension)

add_library(regex STATIC
        contrib/sqlite3.c
        contrib/sqlite3.h
        contrib/sqlite3ext.h
        entry.cpp
        extension/utils.h
        extension/rust.h
        extension/meta.h
        extension/meta.cpp
        extension/regex.cpp
        extension/regex.h
)
