#ifndef MSBT_BINDINGS_H
#define MSBT_BINDINGS_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
unsigned char *cxx_string_to_binary(const char *text, size_t *length);
const char* cxx_binary_to_string(const uint8_t* binary, size_t length);
void free_cxx_string(char* str);

void free_cxx_binary(unsigned char* binary);

#ifdef __cplusplus
}
#endif

#endif  // MSBT_BINDINGS_H
