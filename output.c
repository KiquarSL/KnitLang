#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
int32_t main() { 
    int32_t a = 67;
    int32_t b = 33;
    int32_t c = (a + b);
    const bool is = (c == 100);
    if (is) { 
    c = 0;
 }
    printf("%s\n", "Hello, world");
    return 0;
 }
