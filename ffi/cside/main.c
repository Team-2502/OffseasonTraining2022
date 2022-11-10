#include <dlfcn.h> // dlopen, RTLD_LAZY, dlsym


void *rustlib;

void (*extc_export)();

int main() {
    rustlib = dlopen("../rustside/target/debug/librustside.dylib", RTLD_LAZY);

    *(void **)(&extc_export) = dlsym(rustlib, "c_export");

    extc_export();

}
