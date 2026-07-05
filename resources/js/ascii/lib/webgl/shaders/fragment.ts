export const FRAGMENT_SHADER = `#version 300 es
precision highp float;

// Textures
uniform sampler2D u_video;
uniform sampler2D u_asciiAtlas;

// Dimensions
uniform vec2 u_resolution;
uniform vec2 u_charSize;
uniform vec2 u_gridSize;
uniform float u_numChars;

// Rendering options
uniform bool u_colored;
uniform float u_blend;
uniform float u_highlight;
uniform float u_brightness;

// Audio
uniform float u_audioLevel;
uniform float u_audioReactivity;
uniform float u_audioSensitivity;

// Mouse
uniform vec2 u_mouse;
uniform float u_mouseRadius;
uniform vec2 u_trail[24];
uniform int u_trailLength;

// Ripple
uniform vec4 u_ripples[8];
uniform float u_time;
uniform float u_rippleEnabled;
uniform float u_rippleSpeed;

in vec2 v_texCoord;
out vec4 fragColor;

void main() {
  // Figure out which ASCII cell this pixel is in
  vec2 cellCoord = floor(v_texCoord * u_gridSize);
  vec2 thisCell = cellCoord;
  
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
  
  // Pick the color - video colors or green terminal aesthetic
  vec3 baseColor = vec3(1.0, 1.0, 1.0);
  
  
  // White background, black text
  vec3 bgColor = vec3(1.0, 1.0, 1.0);
  vec3 textColor = vec3(0.0, 0.0, 0.0);
  vec3 finalColor = mix(bgColor, textColor, charColor.r);
  
  // Blend with original video if requested
  
  fragColor = vec4(finalColor, 1.0);
}
`