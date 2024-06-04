#ifndef GUEST_FUNCTION_DETAILS_VERIFIER_H
#define GUEST_FUNCTION_DETAILS_VERIFIER_H

/* Generated by flatcc 0.6.2 FlatBuffers schema compiler for C by dvide.com */

#ifndef GUEST_FUNCTION_DETAILS_READER_H
#include "guest_function_details_reader.h"
#endif
#include "flatcc/flatcc_verifier.h"
#ifndef GUEST_FUNCTION_DEFINITION_VERIFIER_H
#include "guest_function_definition_verifier.h"
#endif
#include "flatcc/flatcc_prologue.h"

static int Hyperlight_Generated_GuestFunctionDetails_verify_table(flatcc_table_verifier_descriptor_t *td);

static int Hyperlight_Generated_GuestFunctionDetails_verify_table(flatcc_table_verifier_descriptor_t *td)
{
    int ret;
    if ((ret = flatcc_verify_table_vector_field(td, 0, 1, &Hyperlight_Generated_GuestFunctionDefinition_verify_table) /* functions */)) return ret;
    return flatcc_verify_ok;
}

static inline int Hyperlight_Generated_GuestFunctionDetails_verify_as_root(const void *buf, size_t bufsiz)
{
    return flatcc_verify_table_as_root(buf, bufsiz, Hyperlight_Generated_GuestFunctionDetails_identifier, &Hyperlight_Generated_GuestFunctionDetails_verify_table);
}

static inline int Hyperlight_Generated_GuestFunctionDetails_verify_as_root_with_size(const void *buf, size_t bufsiz)
{
    return flatcc_verify_table_as_root_with_size(buf, bufsiz, Hyperlight_Generated_GuestFunctionDetails_identifier, &Hyperlight_Generated_GuestFunctionDetails_verify_table);
}

static inline int Hyperlight_Generated_GuestFunctionDetails_verify_as_typed_root(const void *buf, size_t bufsiz)
{
    return flatcc_verify_table_as_root(buf, bufsiz, Hyperlight_Generated_GuestFunctionDetails_type_identifier, &Hyperlight_Generated_GuestFunctionDetails_verify_table);
}

static inline int Hyperlight_Generated_GuestFunctionDetails_verify_as_typed_root_with_size(const void *buf, size_t bufsiz)
{
    return flatcc_verify_table_as_root_with_size(buf, bufsiz, Hyperlight_Generated_GuestFunctionDetails_type_identifier, &Hyperlight_Generated_GuestFunctionDetails_verify_table);
}

static inline int Hyperlight_Generated_GuestFunctionDetails_verify_as_root_with_identifier(const void *buf, size_t bufsiz, const char *fid)
{
    return flatcc_verify_table_as_root(buf, bufsiz, fid, &Hyperlight_Generated_GuestFunctionDetails_verify_table);
}

static inline int Hyperlight_Generated_GuestFunctionDetails_verify_as_root_with_identifier_and_size(const void *buf, size_t bufsiz, const char *fid)
{
    return flatcc_verify_table_as_root_with_size(buf, bufsiz, fid, &Hyperlight_Generated_GuestFunctionDetails_verify_table);
}

static inline int Hyperlight_Generated_GuestFunctionDetails_verify_as_root_with_type_hash(const void *buf, size_t bufsiz, flatbuffers_thash_t thash)
{
    return flatcc_verify_table_as_typed_root(buf, bufsiz, thash, &Hyperlight_Generated_GuestFunctionDetails_verify_table);
}

static inline int Hyperlight_Generated_GuestFunctionDetails_verify_as_root_with_type_hash_and_size(const void *buf, size_t bufsiz, flatbuffers_thash_t thash)
{
    return flatcc_verify_table_as_typed_root_with_size(buf, bufsiz, thash, &Hyperlight_Generated_GuestFunctionDetails_verify_table);
}

#include "flatcc/flatcc_epilogue.h"
#endif /* GUEST_FUNCTION_DETAILS_VERIFIER_H */
