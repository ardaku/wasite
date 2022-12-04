# Environment Variables

The following environment variables **must** be exported by Wasite-compliant
terminal emulators (writes can be ignored to deny permissions without forfeiting
compliance):

## R: `LINES` (Since v0.1.0)
The number of lines in the terminal view.

## R: `COLUMNS` (Since v0.1.0)
The number of columns in the terminal view.

## R/W: `LINE` (Since v0.1.0)
The line number of the cursor position (non-zero)

## R/W: `COLUMN` (Since v0.1.0)
The column number of the cursor position (non-zero)

## R/W: `SCREEN` (Since v0.1.0)
The screen number within the terminal.  Default is `0`.

Non-default (alternate) screens are only the size of the display; They do not
scroll.

## R/W: `TITLE` (Since v0.1.0)
The title of the window.

## R/W: `USER` (Since v0.1.0)
The username of the current user.

## R/W: `HOSTNAME` (Since v0.1.0)
The hostname of the host device.

## R/W: `NAME` (Since v0.1.0)
The pretty name of the host device.

## R/W: `TZ` (Since v0.1.0)
The IANA TZDB identifier for the timezone (ex: `America/New_York`)

## R/W: `LANGS` (Since v0.1.0)
The display language using two letter language code (ISO 639-1) followed by an
underscore and a two letter region code (ISO 3166-1 alpha-2).  (ex: `en_US`)

Multiple languages/dialects can be specified in order of preference, separated
by semicolons.

## R/W: `RAW` (Since v0.1.0)
This mode can be switched on and off.  Default is `0`; `1` to enable.

Enabling changes:
 - Stdout/Stdin is not written to the screen
 - Stdin is not buffered until newline
 - All keyboard shortcuts are intercepted
 - Stdout does not move the cursor on newline characters
