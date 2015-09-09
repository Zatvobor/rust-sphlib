#include <stddef.h>
#include <sys/types.h>
#include <stdlib.h>

/* Adequate size s==len*2 + 1 must be alloced to use this variant */
void __bin2hex(char *s, const unsigned char *p, size_t len)
{
  int i;
  static const char hex[16] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'};

  for (i = 0; i < (int)len; i++) {
    *s++ = hex[p[i] >> 4];
    *s++ = hex[p[i] & 0xF];
  }
  *s++ = '\0';
}

/* Returns a malloced array string of a binary value of arbitrary length. The
 * array is rounded up to a 4 byte size to appease architectures that need
 * aligned array  sizes */
char *bin2hex(const unsigned char *p, size_t len)
{
  ssize_t slen;
  char *s;

  slen = len * 2 + 1;
  if (slen % 4)
    slen += 4 - (slen % 4);
  s = (char *)calloc(slen, 1);
  // FIXME
  // if (unlikely(!s))
  //   printf("Failed to calloc");

  __bin2hex(s, p, len);

  return s;
}
