cmake_minimum_required(VERSION 2.8.12)

project(qjs)

add_library(quickjs STATIC
        cutils.c
        libregexp.c
        libunicode.c
        quickjs.c
        quickjs-libc.c
        )
