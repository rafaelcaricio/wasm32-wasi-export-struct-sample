//
// Created by rafaelcaricio on 26/12/2019.
//

#ifndef EXPORT_STRUCT_SAMPLE_H
#define EXPORT_STRUCT_SAMPLE_H

#include <stddef.h>
#include <stdint.h>

enum {
    JS_TAG_EXCEPTION   = 6,
};

#define JS_MKVAL(tag, val) (XValue){ (XValueUnion){ .float64 = val }, tag }
#define JS_EXCEPTION JS_MKVAL(JS_TAG_EXCEPTION, 0)

typedef struct XMallocState {
    size_t malloc_count;
    size_t malloc_size;
    size_t malloc_limit;
    void *opaque; /* user opaque */
} XMallocState;

typedef struct XContext XContext;

typedef union XValueUnion {
    int32_t int32;
    double float64;
    void *ptr;
} XValueUnion;

typedef struct XValue {
    XValueUnion u;
    int64_t tag;
} XValue;

int JS_IsNumber(const XValue *v);

XContext *X_NewContext();

XValue X_Run(XContext *ctx, const char *input, size_t input_len,
                const char *filename, int eval_flags);

#endif //EXPORT_STRUCT_SAMPLE_H
