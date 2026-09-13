#include <stdint.h>

#define TI_REAL double

int ti_adx_start_valid(double options[1]);

int ti_adx_start(TI_REAL const *options) {
    return ((int)options[0]-1) * 2;
}

int ti_adx_start_valid(double options[1]) { return ti_adx_start(options); }
