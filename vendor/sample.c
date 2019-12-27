//
// Created by rafaelcaricio on 26/12/2019.
//

#include <stdio.h>

#include "sample.h"

int JS_IsNumber(const XValue *v) {
    return 0;
}

XContext *X_NewContext() {
    XContext *ctx = NULL;
    return ctx;
}

XValue X_Run(XContext *ctx, const char *input, size_t input_len,
        const char *filename, int eval_flags) {

    printf("Hello com C code, %s!\n", input);

    return JS_EXCEPTION;
}

