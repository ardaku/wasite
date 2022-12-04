# Standard Input
Standard input takes UTF-8 text for user input, but can also transfer other
formats (via piping).  If raw mode is enabled, then ANSI escape sequences
(matching closely what GNOME terminal produces for maximum compatibility) for
additional events will be sent:

## Non-Character Keys Escape Sequences (Since v0.1.0)
ANSI escape sequences always start with `\x1B[`.

 - Omit for Escape (just `\x1B`)
 - `A`: Up
 - `B`: Down
 - `C`: Right
 - `D`: Left
 - `F`: End
 - `H`: Home
 - `OP`: F1
 - `OQ`: F2
 - `OR`: F3
 - `OS`: F4
 - `Z`: Shift+Tab
 - `0;c;lM`: Mouse left click press `c` (column) and `l` (line)
 - `0;c;lm`: Mouse left click release `c` (column) and `l` (line)
 - `1mA`: Up (`m` modifier)
 - `1mB`: Down (`m` modifier)
 - `1mC`: Right (`m` modifier)
 - `1mD`: Left (`m` modifier)
 - `1mF`: End (`m` modifier)
 - `1mH`: Home (`m` modifier)
 - `1mP`: F1 (`m` modifier)
 - `1mQ`: F2 (`m` modifier)
 - `1mR`: F3 (`m` modifier)
 - `1mS`: F4 (`m` modifier)
 - `1mZ`: Shift+Tab (`m` modifier)
 - `1;c;lM`: Mouse middle click press `c` (column) and `l` (line)
 - `1;c;lm`: Mouse middle click release `c` (column) and `l` (line)
 - `2m~`: Insert (`m` modifier)
 - `2;c;lM`: Mouse right click press `c` (column) and `l` (line)
 - `2;c;lm`: Mouse right click release `c` (column) and `l` (line)
 - `3m~`: Delete (`m` modifier)
 - `5m~`: Page Up (`m` modifier)
 - `6m~`: Page Down (`m` modifier)
 - `15m~`: F5 (`m` modifier)
 - `17m~`: F6 (`m` modifier)
 - `18m~`: F7 (`m` modifier)
 - `19m~`: F8 (`m` modifier)
 - `20m~`: F9 (`m` modifier)
 - `21m~`: F10 (`m` modifier)
 - `23m~`: F11 (`m` modifier)
 - `24m~`: F12 (`m` modifier)
 - `35;c;lM`: Mouse move `c` (column) and `l` (line)

## Modifiers (Since v0.1.0)
Modifiers always start with `;`

 - Omit for None (empty)
 - `;2`: Shift
 - `;3`: Alt
 - `;4`: Shift + Alt
 - `;5`: Control
 - `;6`: Shift + Control
 - `;7`: Alt + Control
 - `;8`: Shift + Alt + Control
