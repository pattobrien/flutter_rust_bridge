#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_struct_with_comments {
  int32_t field_with_comments;
} wire_struct_with_comments;

void wire_StructWithComments_instance_method(int64_t port_, struct wire_struct_with_comments *that);

void wire_StructWithComments_static_method(int64_t port_);

void wire_function_with_comments_slash_star_star(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line(int64_t port_);

void wire_function_with_comments_triple_slash_single_line(int64_t port_);

void wire_simple_adder(int64_t port_, int32_t a, int32_t b);

WireSyncReturn wire_simple_adder_sync(int32_t a, int32_t b);

struct wire_struct_with_comments *new_box_autoadd_struct_with_comments(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_StructWithComments_instance_method);
    dummy_var ^= ((int64_t) (void*) wire_StructWithComments_static_method);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_sync);
    return dummy_var;
}
