#!/bin/sh

clang -c -fPIC -o allocator.o allocator.c
ar rcs liballocator.a allocator.o
