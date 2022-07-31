#ifndef notbr_h
#define notbr_h

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum OutputType {
  NotBrHtml = 1,
  NotBrMarkdown = 2,
} OutputType;

char *process_text(const char *input,
                   int frequency,
                   int bold_percentage,
                   enum OutputType output_type);

void not_br_free(char *output);

#endif /* notbr_h */
