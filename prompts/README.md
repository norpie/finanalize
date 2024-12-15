# Prompting

```tree
.
├── example.prompt
├── example.prompt.final
├── example.prompt.json
├── example.prompt.out
├── example.prompt.struct
├── example.prompt.templ
└── README.md
```

> This document explains how the prompting system works.

## Dependencies

- [https://handlebarsjs.com/](https://handlebarsjs.com/) - A templating engine.

## Files

### example.prompt.templ

The process starts with a template file. This file contains the text that will be used to generate the prompt. The template file is a regular text file with placeholders that will be replaced with some input when rendered.

### example.prompt.json

This JSON file contains the data that will be used to fill the template. The JSON file is a dictionary where the keys are the placeholders in the template and the values are the data that will be used to fill the placeholders.

### example.prompt

This file is the result of rendering the template with the data. The placeholders in the template are replaced with the data from the JSON file.

### example.prompt.out

This file contains the tokens the LLM should output when prompted with `example.prompt`.

### example.prompt.struct

This file contains a structure which needs to be added in front of the tokens in `example.prompt.out` to make it a valid input for the parser.

### example.prompt.final

This is the resulting file after the LLM has been prompted with `example.prompt`.

## Writing a Prompt

Not all of these files are written by a dev. The only files that need to be written are:

- `example.prompt.templ`
- `example.prompt.json` (this is only an example which the dev can use to write the rust struct)
- `example.prompt.struct`
