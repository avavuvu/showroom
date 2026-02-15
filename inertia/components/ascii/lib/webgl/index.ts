// Shaders
export { VERTEX_SHADER } from "./shaders/vertex";
export { FRAGMENT_SHADER } from "./shaders/fragment";

// Utilities
export {
  compileShader,
  createProgram,
  createFullscreenQuad,
  createVideoTexture,
  createAsciiAtlas,
  calculateGridDimensions,
  createUniformSetter,
} from "./utils";

// Types and constants
export {
  CHAR_WIDTH_RATIO,
  type AsciiStats,
  type GridDimensions,
  type UniformSetter,
  type UniformLocations,
  type UseVideoToAsciiOptions,
  type UseAsciiMouseEffectOptions,
  type UseAsciiRippleOptions,
  type UseAsciiAudioOptions,
  type AsciiContext,
  type MouseEffectHandlers,
  type RippleHandlers,
  type VideoToAsciiProps,
  type VideoToAsciiWebGLProps,
  type BenchmarkStats,
  type WebGLResources,
  type Ripple,
} from "./types";
