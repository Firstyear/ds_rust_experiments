
#include "ehasx.h"
#include <stdio.h>

int
main(int argc, char **argv) {
    // Test a simple exposed symbol.
    int32_t i = ehasx_test();
    printf("%d\n", i);
    // Test a struct function
    i = anon_struct_fn();
    printf("%d\n", i);
    // Get a rust struct pointer
    struct testehasx *sptr = testehasx_new();
    i = testehasx_struct_fn(sptr);
    printf("%d\n", i);
    testehasx_destroy(sptr);

    return 0;
}

