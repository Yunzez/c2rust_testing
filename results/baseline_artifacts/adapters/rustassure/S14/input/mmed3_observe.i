typedef unsigned char UChar;
static
__inline__
UChar mmed3 ( UChar a, UChar b, UChar c )
{
   UChar t;
   if (a > b) { t = a; a = b; b = t; };
   if (b > c) {
      b = c;
      if (a > b) b = a;
   }
   return b;
}
UChar mmed3_observe(UChar a, UChar b, UChar c)
{
    return mmed3(a, b, c);
}
