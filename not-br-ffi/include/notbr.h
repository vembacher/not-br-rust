#ifndef NOT_BR_RS_NOT_BRR_H
#define NOT_BR_RS_NOT_BRR_H

enum OutputType
{
    NotBrHTML = 1,
    NotBrMarkdown = 2,
};

char *process_text(char *input, int frequency, int bold_percentage, int input_type);

void not_br_free(char *output);

#endif //NOT_BR_RS_NOT_BRR_H
