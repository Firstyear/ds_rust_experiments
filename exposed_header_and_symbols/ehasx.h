
#include <inttypes.h>

int32_t ehasx_test();
int32_t anon_struct_fn();

// This is an anonymous struct type, for us to use pointers.
struct testehasx;

struct testehasx* testehasx_new();
int32_t testehasx_struct_fn(struct testehasx *ptr);
void testehasx_destroy(struct testehasx *ptr);



