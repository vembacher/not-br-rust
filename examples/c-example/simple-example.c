#include <stdio.h>
#include "notbr.h"

int main()
{
    printf("--- Example Markdown ---\n");
    char *input = "The quick brown fox jumps over the lazy dog.\n";
    char *output = process_text(input, 1, 50, NotBrMarkdown);
    printf("%s", output);
    not_br_free(output);
    printf("\n");

    printf("--- Example HTML ---\n");
    char *input_html = "The quick brown fox jumps over the lazy dog.\n";
    char *output_html = process_text(input_html, 1, 50, NotBrHtml);
    printf("%s", output_html);
    not_br_free(output_html);
}