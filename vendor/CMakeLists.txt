cmake_minimum_required(VERSION 2.8.12)

set(CMAKE_C_FLAGS "$ENV{CFLAGS} -Wall -Wextra -Wno-sign-compare -Wno-missing-field-initializers -Wundef -Wuninitialized -Wunused -Wno-unused-parameter -Wwrite-strings -Wchar-subscripts -funsigned-char")

add_definitions(
        -D__x86_64__
)

project(qjs)

add_library(quickjs STATIC
        cutils.c
        libregexp.c
        libunicode.c
        quickjs.c
        quickjs-libc.c
        )
