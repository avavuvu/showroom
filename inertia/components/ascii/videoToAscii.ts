import { ref, computed, watch, onMounted, onBeforeUnmount, type Ref } from 'vue';
import { getCharArray, DEFAULT_CHARSET } from "~/components/ascii/lib/ascii-charsets"
import {
    FRAGMENT_SHADER,
    VERTEX_SHADER,

    compileShader,
    createProgram,
    createFullscreenQuad,
    createVideoTexture,
    createAsciiAtlas,
    calculateGridDimensions,
    CHAR_WIDTH_RATIO,
    type UseVideoToAsciiOptions,
    type AsciiContext,
    type AsciiStats,
    type UniformSetter,
    type UniformLocations,
} from "~/components/ascii/lib/webgl"

export type { UseVideoToAsciiOptions, AsciiContext, AsciiStats };

const MAX_TRAIL_LENGTH = 24;
const MAX_RIPPLES = 8;

// Composable Implementation
export function useVideoToAscii(options: UseVideoToAsciiOptions = {}) {
    const {
        fontSize,
        numColumns,
        colored = true,
        blend = 0,
        highlight = 0,
        brightness = 1.0,
        charset = DEFAULT_CHARSET,
        maxWidth,
        enableSpacebarToggle = false,
        onStats,
    } = options;

    // Template refs (equivalent to DOM refs in React)
    const containerRef: Ref<HTMLDivElement | null> = ref(null);
    const videoRef: Ref<HTMLVideoElement | null> = ref(null);
    const canvasRef: Ref<HTMLCanvasElement | null> = ref(null);

    // WebGL refs
    const glRef: Ref<WebGL2RenderingContext | null> = ref(null);
    const programRef: Ref<WebGLProgram | null> = ref(null);
    const videoTextureRef: Ref<WebGLTexture | null> = ref(null);
    const atlasTextureRef: Ref<WebGLTexture | null> = ref(null);
    const animationRef = ref(0);

    // Feature hooks register their uniform setters here
    const uniformSettersRef: Ref<Map<string, UniformSetter>> = ref(new Map());
    const uniformLocationsRef: Ref<UniformLocations | null> = ref(null);

    // Benchmark/stats refs
    const frameCountRef = ref(0);
    const frameTimesRef: Ref<number[]> = ref([]);
    const lastFpsTimeRef = ref(performance.now());

    // Reactive state (equivalent to useState)
    const dimensions = ref({ cols: 80, rows: 24 });
    const stats = ref<AsciiStats>({ fps: 0, frameTime: 0 });
    const isReady = ref(false);
    const isPlaying = ref(false);

    // Computed values (equivalent to useMemo)
    const defaultWidth = typeof window !== 'undefined' ? window.innerWidth : 900;
    const containerWidth = maxWidth || defaultWidth;

    const calculatedFontSize = computed(() =>
        numColumns ? containerWidth / (numColumns * CHAR_WIDTH_RATIO) : fontSize || 10
    );

    const calculatedMaxWidth = computed(() =>
        numColumns
            ? numColumns * calculatedFontSize.value * CHAR_WIDTH_RATIO
            : maxWidth || 900
    );

    const charWidth = computed(() => calculatedFontSize.value * CHAR_WIDTH_RATIO);
    const cols = computed(() =>
        numColumns || Math.floor(calculatedMaxWidth.value / charWidth.value)
    );

    const chars = computed(() => getCharArray(charset));

    // Methods
    const registerUniformSetter = (id: string, setter: UniformSetter) => {
        uniformSettersRef.value.set(id, setter);
    };

    const unregisterUniformSetter = (id: string) => {
        uniformSettersRef.value.delete(id);
    };

    const cacheUniformLocations = (
        gl: WebGL2RenderingContext,
        program: WebGLProgram
    ): UniformLocations => {
        const get = (name: string) => gl.getUniformLocation(program, name);

        return {
            // Core uniforms
            u_video: get('u_video'),
            u_asciiAtlas: get('u_asciiAtlas'),
            u_resolution: get('u_resolution'),
            u_charSize: get('u_charSize'),
            u_gridSize: get('u_gridSize'),
            u_numChars: get('u_numChars'),
            u_colored: get('u_colored'),
            u_blend: get('u_blend'),
            u_highlight: get('u_highlight'),
            u_brightness: get('u_brightness'),

            // Mouse uniforms
            u_mouse: get('u_mouse'),
            u_mouseRadius: get('u_mouseRadius'),
            u_trailLength: get('u_trailLength'),
            u_trail: Array.from({ length: MAX_TRAIL_LENGTH }, (_, i) =>
                get(`u_trail[${i}]`)
            ),

            // Ripple uniforms
            u_time: get('u_time'),
            u_rippleEnabled: get('u_rippleEnabled'),
            u_rippleSpeed: get('u_rippleSpeed'),
            u_ripples: Array.from({ length: MAX_RIPPLES }, (_, i) =>
                get(`u_ripples[${i}]`)
            ),

            // Audio uniforms
            u_audioLevel: get('u_audioLevel'),
            u_audioReactivity: get('u_audioReactivity'),
            u_audioSensitivity: get('u_audioSensitivity'),
        };
    };

    const initWebGL = (): boolean => {
        const canvas = canvasRef.value;
        const video = videoRef.value;
        const container = containerRef.value;

        if (!canvas || !video || !video.videoWidth) return false;

        let finalFontSize = calculatedFontSize.value;
        let finalCols = cols.value;

        if (numColumns && container) {
            const actualWidth = container.clientWidth || defaultWidth;
            finalFontSize = actualWidth / (numColumns * CHAR_WIDTH_RATIO);
            finalCols = numColumns;
        }

        const grid = calculateGridDimensions(
            video.videoWidth,
            video.videoHeight,
            finalCols
        );
        dimensions.value = grid;

        const finalCharWidth = finalFontSize * CHAR_WIDTH_RATIO;
        const pixelWidth = grid.cols * finalCharWidth;
        const pixelHeight = grid.rows * finalFontSize;

        canvas.width = pixelWidth;
        canvas.height = pixelHeight;

        const gl = canvas.getContext('webgl2', {
            antialias: false,
            preserveDrawingBuffer: false,
        });

        if (!gl) {
            console.error('WebGL2 not supported');
            return false;
        }

        glRef.value = gl;

        const vertexShader = compileShader(gl, VERTEX_SHADER, gl.VERTEX_SHADER);
        const fragmentShader = compileShader(gl, FRAGMENT_SHADER, gl.FRAGMENT_SHADER);

        if (!vertexShader || !fragmentShader) return false;

        const program = createProgram(gl, vertexShader, fragmentShader);
        if (!program) return false;

        programRef.value = program;
        gl.useProgram(program);

        createFullscreenQuad(gl, program);

        videoTextureRef.value = createVideoTexture(gl);

        const finalFontSizeForAtlas =
            numColumns && container
                ? (container.clientWidth || defaultWidth) / (numColumns * CHAR_WIDTH_RATIO)
                : calculatedFontSize.value;

        atlasTextureRef.value = createAsciiAtlas(gl, chars.value, finalFontSizeForAtlas);

        const locations = cacheUniformLocations(gl, program);
        uniformLocationsRef.value = locations;

        gl.uniform1i(locations.u_video, 0);
        gl.uniform1i(locations.u_asciiAtlas, 1);

        gl.uniform2f(locations.u_resolution, pixelWidth, pixelHeight);
        gl.uniform2f(locations.u_charSize, finalCharWidth, finalFontSize);
        gl.uniform2f(locations.u_gridSize, finalCols, grid.rows);
        gl.uniform1f(locations.u_numChars, chars.value.length);
        gl.uniform1f(locations.u_brightness, brightness);

        gl.uniform2f(locations.u_mouse, -1, -1);
        gl.uniform1f(locations.u_mouseRadius, 0);
        gl.uniform1i(locations.u_trailLength, 0);
        gl.uniform1f(locations.u_rippleEnabled, 0);
        gl.uniform1f(locations.u_audioLevel, 0);
        gl.uniform1f(locations.u_audioReactivity, 0);
        gl.uniform1f(locations.u_audioSensitivity, 0);

        gl.viewport(0, 0, pixelWidth, pixelHeight);

        isReady.value = true;
        return true;
    };

    const render = () => {
        const gl = glRef.value;
        const video = videoRef.value;
        const program = programRef.value;
        const locations = uniformLocationsRef.value;

        if (!gl || !video || !program || !locations || video.paused || video.ended)
            return;

        const frameStart = performance.now();

        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, videoTextureRef.value);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, video);
        gl.generateMipmap(gl.TEXTURE_2D);

        gl.activeTexture(gl.TEXTURE1);
        gl.bindTexture(gl.TEXTURE_2D, atlasTextureRef.value);

        gl.uniform1i(locations.u_colored, colored ? 1 : 0);
        gl.uniform1f(locations.u_blend, blend / 100);
        gl.uniform1f(locations.u_highlight, highlight / 100);
        gl.uniform1f(locations.u_brightness, brightness);

        for (const setter of uniformSettersRef.value.values()) {
            setter(gl, program, locations);
        }

        gl.drawArrays(gl.TRIANGLES, 0, 6);

        const frameEnd = performance.now();
        frameCountRef.value++;
        frameTimesRef.value.push(frameEnd - frameStart);
        if (frameTimesRef.value.length > 60) frameTimesRef.value.shift();

        const now = performance.now();
        if (now - lastFpsTimeRef.value >= 1000) {
            const avgFrameTime =
                frameTimesRef.value.reduce((a, b) => a + b, 0) / frameTimesRef.value.length;
            const newStats = {
                fps: frameCountRef.value,
                frameTime: avgFrameTime,
            };
            stats.value = newStats;
            onStats?.(newStats);
            frameCountRef.value = 0;
            lastFpsTimeRef.value = now;
        }

        animationRef.value = requestAnimationFrame(render);
    };

    // Video event handlers
    const handleLoadedMetadata = () => {
        initWebGL();
    };

    const handlePlay = () => {
        isPlaying.value = true;
        animationRef.value = requestAnimationFrame(render);
    };

    const handlePause = () => {
        isPlaying.value = false;
        cancelAnimationFrame(animationRef.value);
    };

    const handleEnded = () => {
        isPlaying.value = false;
        cancelAnimationFrame(animationRef.value);
    };

    // Playback controls
    const play = () => {
        videoRef.value?.play();
    };

    const pause = () => {
        videoRef.value?.pause();
    };

    const toggle = () => {
        const video = videoRef.value;
        if (!video) return;

        if (video.paused) {
            video.play();
        } else {
            video.pause();
        }
    };

    // Keyboard handler
    const handleKeyDown = (e: KeyboardEvent) => {
        if (e.code === 'Space' && e.target === document.body) {
            e.preventDefault();
            toggle();
        }
    };

    // Resize observer
    let resizeObserver: ResizeObserver | null = null;

    // Lifecycle hooks
    onMounted(() => {
        const video = videoRef.value;
        if (!video) return;

        video.addEventListener('loadedmetadata', handleLoadedMetadata);
        video.addEventListener('play', handlePlay);
        video.addEventListener('pause', handlePause);
        video.addEventListener('ended', handleEnded);

        if (video.readyState >= 1) {
            handleLoadedMetadata();
        }

        // Spacebar toggle
        if (enableSpacebarToggle) {
            window.addEventListener('keydown', handleKeyDown);
        }

        // Resize observer
        if (numColumns && containerRef.value) {
            resizeObserver = new ResizeObserver(() => {
                if (videoRef.value && videoRef.value.readyState >= 1) {
                    const wasPlaying = !videoRef.value.paused;
                    if (initWebGL() && wasPlaying) {
                        requestAnimationFrame(() => {
                            render();
                        });
                    }
                }
            });
            resizeObserver.observe(containerRef.value);
        }
    });

    onBeforeUnmount(() => {
        const video = videoRef.value;
        if (video) {
            video.removeEventListener('loadedmetadata', handleLoadedMetadata);
            video.removeEventListener('play', handlePlay);
            video.removeEventListener('pause', handlePause);
            video.removeEventListener('ended', handleEnded);
        }

        if (enableSpacebarToggle) {
            window.removeEventListener('keydown', handleKeyDown);
        }

        if (resizeObserver) {
            resizeObserver.disconnect();
        }

        const gl = glRef.value;
        if (gl) {
            if (videoTextureRef.value) gl.deleteTexture(videoTextureRef.value);
            if (atlasTextureRef.value) gl.deleteTexture(atlasTextureRef.value);
            if (programRef.value) gl.deleteProgram(programRef.value);
        }

        cancelAnimationFrame(animationRef.value);
    });

    // Watch for config changes (equivalent to useEffect with dependencies)
    watch(
        [calculatedFontSize, cols, chars, () => brightness],
        () => {
            if (videoRef.value && videoRef.value.readyState >= 1) {
                initWebGL();
            }
        },
        { deep: true }
    );

    return {
        containerRef,
        videoRef,
        canvasRef,
        glRef,
        programRef,
        uniformLocationsRef,
        registerUniformSetter,
        unregisterUniformSetter,
        dimensions,
        stats,
        isReady,
        isPlaying,
        play,
        pause,
        toggle,
    };
}