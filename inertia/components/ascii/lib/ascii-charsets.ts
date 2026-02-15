/**
 * ASCII Character Set Definitions
 *
 * Character sets are ordered from dark (low brightness) to light (high brightness).
 * The shader maps pixel brightness to character index, so the first character
 * represents the darkest pixels and the last represents the brightest.
 *
 * To add a new character set:
 * 1. Add an entry to ASCII_CHARSETS with a unique key
 * 2. Order characters from dark → light (spaces/dots first, dense chars last)
 * 3. The key becomes available in CharsetKey type automatically
 */

export const ASCII_CHARSETS = {
  /** Classic 10-character gradient - good balance of detail and performance */
  standard: {
    name: "Standard",
    chars: " .:-=+*#%@",
  },

  showroom: {
    name: "Showroom",
    chars: ".sowhROM",
  },

  /** Unicode block characters - chunky retro aesthetic */
  blocks: {
    name: "Blocks",
    chars: " ░▒▓█",
  },

  /** Minimal 5-character set - high contrast, fast rendering */
  minimal: {
    name: "Minimal",
    chars: " .oO@",
  },

  /** Binary on/off - pure silhouette mode */
  binary: {
    name: "Binary",
    chars: " █",
  },

  /** 70-character gradient - maximum detail, best for high resolution */
  detailed: {
    name: "Detailed",
    chars:
      " .'`^\",:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$",
  },

  /** Dot-based - pointillist aesthetic */
  dots: {
    name: "Dots",
    chars: " ·•●",
  },

  /** Directional arrows - experimental */
  arrows: {
    name: "Arrows",
    chars: " ←↙↓↘→↗↑↖",
  },

  /** Moon phases - decorative gradient */
  emoji: {
    name: "Emoji",
    chars: "  ❤️▒▓🌑🌒🌓🌔🌕",
  },
} as const;

/** Type-safe key for selecting character sets */
export type CharsetKey = keyof typeof ASCII_CHARSETS;

/** Default character set used when none is specified */
export const DEFAULT_CHARSET: CharsetKey = "standard";

/**
 * Get the character array for a given charset key.
 * Uses spread operator to correctly handle multi-byte unicode characters.
 */
export function getCharArray(charset: CharsetKey): string[] {
  return [...ASCII_CHARSETS[charset].chars];
}

/**
 * Get the display name for a charset
 */
export function getCharsetName(charset: CharsetKey): string {
  return ASCII_CHARSETS[charset].name;
}
