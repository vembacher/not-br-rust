#include <stdio.h>
#include "notbr.h"

int main()
{
    char *input = "The quick brown fox jumps over the lazy dog.\n";
    char *output = process_text(input, 1, 50, NotBrMarkdown);
    printf("%s", output);
    not_br_free(output);
}