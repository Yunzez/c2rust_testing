#include <stdint.h>
#include <string.h>

#define False 0

int ends_in_bz2_valid(int8_t value[16]);

typedef char Char; typedef int Bool; typedef int Int32;
static Bool endsInBz2 ( Char* name )
{
   Int32 n = strlen ( name );
   if (n <= 4) return False;
   return
      (name[n-4] == '.' &&
       name[n-3] == 'b' &&
       name[n-2] == 'z' &&
       name[n-1] == '2');
}

int ends_in_bz2_valid(int8_t value[16]) { value[15] = 0; return endsInBz2((Char *)value); }
