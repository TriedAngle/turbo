<div align="center">
  <h1> Turbo </h1>
  <p>the Text Format that is just turbo</p>
</div>

# Overview
Examples can be found in `/examples` 
## Text
- Cursive: `**<text>**`
- Underlined: `__<text>__`
- Bold: `*<text>*`
- Supertext: `^<text>^`
- Subtext: `_<text>_`
- Striked: `~<text>~`
- Code: ``` `<text>` ```

### Backslash
- `\` + any char will add that char to the text (required for text modifier)
- `\{<text>}` same as `\` but with any amount of chars
- `\` at the end of the line will create a line break

### Links
created with: `[<alias>](<address>)`\
if alias is empty, the address will be displayed

All of these can be nested.
### Planned:
- Add: `$` for inline KaTex
- Add: nestable marker / highlighting with color support
- Add: Image Support

## Lists
- lists are created with `-`
- lists are nestable
- list suppoert multiline, the first line will always be separate though.
- the first line of a list item can have a check `- [ ]` or `- [x]`
- numbering can be added with `1.` => `- 1. <text>`
- besides numbering the following are also supported: 
  - `a..z` => lowercase alphabet
  - `A..Z` => uppercase alphabet
  - `i..` => lowercase roman
  - `I..` => uppercase roman
  - unordered with disc, circle and square

### Planned:
- Add: Custom List Values
- Add: No List Marking

## Code Blocks
- constructed like this:
  ```
  ::: <lang/type>
  <code>
  :::
  ```
- Syntax Highlighting
- KaTex (LaTeX Math) => `math` or `katex`
- Mermaid (Graphs) => `mermaid`

### Planned:
- Add: Tables from code blocks by a format similar to json, but without the `"` => `table`
- Better Support List nesting, by trimming early whitespace

## Layouting:
- Headings: `#` define heading size (1 largest)
- Headings also support multiline text, and can appear in lists

- horizontal line: `---` or more `-` in a otherwise empty line

- text and other structures are separated by empty lines

### Planned:
- Finish Cross File Linking and File Inclusion
- Grid-like layouting
- Videos
- Variables (Markdown like and References, similar to BibTeX)
