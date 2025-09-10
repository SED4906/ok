#include <stdarg.h>
#include <stddef.h>

int vsnprintf(char *s, size_t n, const char *format, ...) {
  va_list arg;
  va_start (arg, format);
  va_end(arg);
  return 0;
}
