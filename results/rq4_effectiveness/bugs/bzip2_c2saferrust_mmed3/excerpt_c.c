
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
