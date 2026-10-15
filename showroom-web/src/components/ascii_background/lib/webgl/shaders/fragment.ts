export const FRAGMENT_SHADER = `#version 300 es
precision highp float;

// Textures
uniform sampler2D u_video;
uniform sampler2D u_asciiAtlas;

// Dimensions
uniform vec2 u_gridSize;
uniform float u_numChars;

uniform float u_brightness;

in vec2 v_texCoord;
out vec4 fragColor;

void main() {
  // Figure out which ASCII cell this pixel is in
  vec2 cellCoord = floor(v_texCoord * u_gridSize);

  // Sample video at cell center (mipmaps handle averaging)
  vec2 cellCenter = (cellCoord + 0.5) / u_gridSize;
  vec4 videoColor = texture(u_video, cellCenter);

  // Perceived brightness using human eye sensitivity weights
  float brightness = dot(videoColor.rgb, vec3(0.299, 0.587, 0.114));

  // Apply brightness multiplier
  // brightness < 1.0: darkens (multiply)
  // brightness > 1.0: brightens (compress dark values toward 1.0)
  float adjustedBrightness;
  if (u_brightness <= 1.0) {
    adjustedBrightness = brightness * u_brightness;
  } else {
    // For brightness > 1.0, compress the range: dark values get pushed up
    // Formula: 1.0 - (1.0 - brightness) / u_brightness
    // This makes dark values brighter while keeping bright values near 1.0
    adjustedBrightness = 1.0 - (1.0 - brightness) / u_brightness;
  }
  adjustedBrightness = clamp(adjustedBrightness, 0.0, 1.0);

  // Map brightness to character index (Inverted for dark-on-light: 0 = lightest/sparse, N = darkest/dense)
  float charIndex = floor((1.0 - adjustedBrightness) * (u_numChars - 0.001));

  // Find the character in the atlas (horizontal strip of pre-rendered chars)
  float atlasX = charIndex / u_numChars;
  vec2 cellPos = fract(v_texCoord * u_gridSize);
  vec2 atlasCoord = vec2(atlasX + cellPos.x / u_numChars, cellPos.y);
  vec4 charColor = texture(u_asciiAtlas, atlasCoord);

  // White background, atlas-coloured text
  vec3 bgColor = vec3(1.0, 1.0, 1.0);
  float charMask = dot(charColor.rgb, vec3(0.299, 0.587, 0.114));
  vec3 finalColor = mix(bgColor, charColor.rgb, smoothstep(0.05, 0.6, charMask));

  fragColor = vec4(finalColor, 1.0);
}
`;
