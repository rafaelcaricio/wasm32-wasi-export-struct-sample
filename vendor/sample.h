//
// Created by rafaelcaricio on 26/12/2019.
//

#ifndef EXPORT_STRUCT_SAMPLE_H
#define EXPORT_STRUCT_SAMPLE_H

#include <stddef.h>

typedef union XValueUnion {
    double float64;
    void *ptr;
} XValueUnion;

typedef struct XValue {
    XValueUnion u;
    double tag;
} XValue;

XValue X_Run(const char *input, size_t input_len);

#endif //EXPORT_STRUCT_SAMPLE_H
