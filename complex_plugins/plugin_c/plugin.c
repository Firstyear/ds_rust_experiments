
#include <stdio.h>
#include <inttypes.h>

void register_pre_operation(void *plugin_register);

int32_t
plugin_init(void *plugin_register) {
    printf("plugin_c: init\n");
    register_pre_operation(plugin_register);
    return 0;
}

