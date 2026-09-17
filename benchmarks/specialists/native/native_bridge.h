#ifndef SNAPTOKENS_SPECIALISTS_NATIVE_BRIDGE_H
#define SNAPTOKENS_SPECIALISTS_NATIVE_BRIDGE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
#define ST_NOEXCEPT noexcept
extern "C" {
#else
#define ST_NOEXCEPT
#endif

// Borrows one rank entry only for the duration of a constructor call.
typedef struct NativeRankEntry {
    const uint8_t* data;
    size_t len;
    uint32_t rank;
} NativeRankEntry;

// Borrows one special token only for the duration of a constructor call.
typedef struct NativeSpecialEntry {
    const uint8_t* data;
    size_t len;
    uint32_t id;
} NativeSpecialEntry;

// Owns a type-erased C++ vector until the matching release call.
typedef struct NativeBuffer {
    const void* data;
    size_t len;
    void* owner;
} NativeBuffer;

// Constructs TokenDagger's public CoreBPE from borrowed exact model data.
void* st_tokendagger_create(const uint8_t* pattern, size_t pattern_len,
                            const NativeRankEntry* ranks, size_t rank_count,
                            const NativeSpecialEntry* specials, size_t special_count,
                            NativeBuffer* error) ST_NOEXCEPT;
// Encodes one input through TokenDagger's public scalar API.
int st_tokendagger_encode(void* handle, const uint8_t* text, size_t text_len, NativeBuffer* output,
                          NativeBuffer* error) ST_NOEXCEPT;
// Releases an ID vector returned by st_tokendagger_encode.
void st_tokendagger_release_output(NativeBuffer output) ST_NOEXCEPT;
// Releases an exception string returned by either bridge call.
void st_tokendagger_release_error(NativeBuffer error) ST_NOEXCEPT;
// Destroys one persistent TokenDagger encoder.
void st_tokendagger_destroy(void* handle) ST_NOEXCEPT;
// Reports the linked PCRE2 runtime version and JIT capability.
int st_tokendagger_pcre2_info(char* version, size_t version_capacity, uint32_t* jit_available,
                              uint32_t* header_major, uint32_t* header_minor) ST_NOEXCEPT;

#ifdef __cplusplus
}
#endif

#undef ST_NOEXCEPT

#endif
