#include <stdio.h>
#include <string.h>
typedef unsigned char png_byte; typedef png_byte *png_bytep; typedef unsigned long png_alloc_size_t;
/* verbatim from benchmark/pairs/rq4/optipng_c2saferrust/source/libpng/pngwutil.c:251-289 */
static void
optimize_cmf(png_bytep data, png_alloc_size_t data_size)
{
   if (data_size <= 16384)
   {
      unsigned int z_cmf = data[0];
      if ((z_cmf & 0x0f) == 8 && (z_cmf & 0xf0) <= 0x70)
      {
         unsigned int z_cinfo;
         unsigned int half_z_window_size;
         z_cinfo = z_cmf >> 4;
         half_z_window_size = 1U << (z_cinfo + 7);
         if (data_size <= half_z_window_size)
         {
            unsigned int tmp;
            do
            {
               half_z_window_size >>= 1;
               --z_cinfo;
            }
            while (z_cinfo > 0 && data_size <= half_z_window_size);
            z_cmf = (z_cmf & 0x0f) | (z_cinfo << 4);
            data[0] = (png_byte)z_cmf;
            tmp = data[1] & 0xe0;
            tmp += 0x1f - ((z_cmf << 8) + tmp) % 0x1f;
            data[1] = (png_byte)tmp;
         }
      }
   }
}
int main(int argc, char **argv) {
    unsigned char d[2] = {0x08, 0x1d};           /* CM=8, CINFO=0, FCHECK valid: 0x081d % 31 == 0 */
    unsigned long sizes[] = {1, 64, 100, 128};
    for (int i = 0; i < 4; i++) { unsigned char x[2]; memcpy(x, d, 2); optimize_cmf(x, sizes[i]); printf("C   data_size=%3lu -> %02x %02x\n", sizes[i], x[0], x[1]); }
    return 0;
}
