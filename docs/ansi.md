# Standard Output

Sequences start with the escape character followed by a left bracket `\x1B[` and
end with `m`.  The style options give you 8 colors for both background and
foreground (bold may not change intensity).  And 4 style attributes (thickness,
italic, underline, inverted).

The following ANSI escape sequences must be supported by Wasite-compliant
terminal emulators:

## `\x1B[0m` (Since v0.1.0)
Disable all style attribute flags (doesn't include colors)

## `\x1B[1m` (Since v0.1.0)
Set thickness style attribute flag to bold

## `\x1B[2m` (Since v0.1.0)
Set thickness style attribute flag to thin

## `\x1B[3m` (Since v0.1.0)
Enable italic style attribute flag

## `\x1B[4m` (Since v0.1.0)
Enable underlined style attribute flag

## `\x1B[7m` (Since v0.1.0)
Enable color inversion style attribute flag

-----

## `\x1B[22m` (Since v0.1.0)
Set thickness style attribute flag to regular

## `\x1B[23m` (Since v0.1.0)
Disable italic style attribute flag

## `\x1B[24m` (Since v0.1.0)
Disable underlined style attribute flag

## `\x1B[27m` (Since v0.1.0)
Disable color inversion style attribute flag

-----

## `\x1B[30m` (Since v0.1.0)
Set foreground color to black

## `\x1B[31m` (Since v0.1.0)
Set foreground color to red

## `\x1B[32m` (Since v0.1.0)
Set foreground color to green

## `\x1B[33m` (Since v0.1.0)
Set foreground color to yellow

## `\x1B[34m` (Since v0.1.0)
Set foreground color to blue

## `\x1B[35m` (Since v0.1.0)
Set foreground color to magenta

## `\x1B[36m` (Since v0.1.0)
Set foreground color to cyan

## `\x1B[37m` (Since v0.1.0)
Set foreground color to white

## `\x1B[38;5;nm` (Since v0.1.0)
Set foreground to `n` from
[Color table](https://www.ditig.com/256-colors-cheat-sheet).

## `\x1B[38;2;r;g;bm` (Since v0.1.0)
Set 24-bit foreground color to `r`, `g`, `b` (decimal 0-255)

## `\x1B[39m` (Since v0.1.0)
Reset foreground color to default

-----

## `\x1B[40m` (Since v0.1.0)
Set background color to black

## `\x1B[41m` (Since v0.1.0)
Set background color to red

## `\x1B[42m` (Since v0.1.0)
Set background color to green

## `\x1B[43m` (Since v0.1.0)
Set background color to yellow

## `\x1B[44m` (Since v0.1.0)
Set background color to blue

## `\x1B[45m` (Since v0.1.0)
Set background color to magenta

## `\x1B[46m` (Since v0.1.0)
Set background color to cyan

## `\x1B[47m` (Since v0.1.0)
Set background color to white

## `\x1B[48;5;nm` (Since v0.1.0)
Set background to `n` from
[Color table](https://www.ditig.com/256-colors-cheat-sheet).

## `\x1B[48;2;r;g;bm` (Since v0.1.0)
Set 24-bit background color to `r`, `g`, `b` (decimal 0-255)

## `\x1B[49m` (Since v0.1.0)
Reset background color to default

-----

## `\x1B[90m` (Since v0.1.0)
Set foreground color to bright black

## `\x1B[91m` (Since v0.1.0)
Set foreground color to bright red

## `\x1B[92m` (Since v0.1.0)
Set foreground color to bright green

## `\x1B[93m` (Since v0.1.0)
Set foreground color to bright yellow

## `\x1B[94m` (Since v0.1.0)
Set foreground color to bright blue

## `\x1B[95m` (Since v0.1.0)
Set foreground color to bright magenta

## `\x1B[96m` (Since v0.1.0)
Set foreground color to bright cyan

## `\x1B[97m` (Since v0.1.0)
Set foreground color to bright white

-----

## `\x1B[100m` (Since v0.1.0)
Set background color to bright black

## `\x1B[101m` (Since v0.1.0)
Set background color to bright red

## `\x1B[102m` (Since v0.1.0)
Set background color to bright green

## `\x1B[103m` (Since v0.1.0)
Set background color to bright yellow

## `\x1B[104m` (Since v0.1.0)
Set background color to bright blue

## `\x1B[105m` (Since v0.1.0)
Set background color to bright magenta

## `\x1B[106m` (Since v0.1.0)
Set background color to bright cyan

## `\x1B[107m` (Since v0.1.0)
Set background color to bright white

-----

# Unicode special codepoints

## NUL `\x00` (Since v0.1.0)
Clear screen

## BEL `\x07` (Since v0.1.0)
Visual alert

## BS `\x08` (Since v0.1.0)
Move cursor back one column

## TAB `\x09` (Since v0.1.0)
Move cursor to the next tab stop (always every 8 columns)

## LF `\x0A` (Since v0.1.0)
Clear until end of line, move cursor to the beginning of the next line

## VT `\x0B` (Since v0.1.0)
Move cursor to the beginning of the next line without clearing

## FF `\x0C` (Since v0.1.0)
Move cursor to next line, keeping same column

## CR `\x0D` (Since v0.1.0)
Return cursor to start of line
