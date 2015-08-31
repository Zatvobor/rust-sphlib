#include <stddef.h>
#include <sys/types.h>
#include <stdlib.h>

void __bin2hex(char *s, const unsigned char *p, size_t len);
char *bin2hex(const unsigned char *p, size_t len);
