#include <emscripten/emscripten.h>

// https://developer.mozilla.org/en-US/docs/WebAssembly/C_to_wasm
extern "C"
{
#include <stdio.h>

    const int EMSCRIPTEN_KEEPALIVE answer()
    {
        return 42;
    }
}
