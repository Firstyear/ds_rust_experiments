
#include <stdio.h>

void external_symbol(void);

void __attribute__ ((weak, alias("external_symbol")))
_external_symbol(void) {
    printf("Hello From C!\n");
}

int
main(int argc, char **argv) {
    printf("Testing External Symbol\n");
    external_symbol();
    return 0;
}


