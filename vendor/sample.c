//
// Created by rafaelcaricio on 26/12/2019.
//

#include <stdio.h>

#include "sample.h"

XContext *X_NewContext() {
    XContext *ctx = NULL;
    return ctx;
}


XValue X_Run(XContext *ctx, const char *input, size_t input_len,
        const char *filename, int eval_flags) {

    printf("Hello com C code, %s!\n", input);

    return JS_EXCEPTION;
}

