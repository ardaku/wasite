# Environment Variables

The following environment variables **must** be exported by Wasite-compliant
terminal emulators (writes can be ignored to deny permissions without forfeiting
compliance):

## `USER` (Since v0.1.0)

The username of the current user.

## `HOSTNAME` (Since v0.1.0)

The hostname of the host device.

## `NAME` (Since v0.1.0)

The pretty name of the host device.

## `TZ` (Since v0.1.0)

The IANA TZDB identifier for the timezone (ex: `America/New_York`)

## `LANGS` (Since v1.0.0)

The display language using two letter language code (ISO 639-1) followed by a
forward slash and a two letter region code (ISO 3166-1 alpha-2).  (ex: `en/US`)

Multiple languages/dialects can be specified in order of preference, separated
by colons.

Key / value pairs can specify languages for specific contexts:

 - `Collation`: Sorting
 - `CharClass`: How characters should be classified
 - `Monetary`: Kind of money
 - `Message`: Messages
 - `Numeric`: Thousands separator, number system
 - `Time`: Displaying date and time

So, for a preference of American English, falling back to Mexican Spanish if not
available, and a preference of pound sterlings for money:

```
Monetary=en/GB:en/US:es/MX
```
