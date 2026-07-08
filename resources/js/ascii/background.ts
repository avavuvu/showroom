import {
    VERTEX_SHADER,
    FRAGMENT_SHADER,
    compileShader,
    createProgram,
    createFullscreenQuad,
    createVideoTexture,
    createAsciiAtlas,
} from "./lib/webgl";
import { getCharArray } from "./lib/ascii-charsets";

const FONT_SIZE = 16;
const CHAR_WIDTH_RATIO = 1;

export function initAsciiBackground(container: HTMLElement): () => void {
    const chars = getCharArray("showroom");
    const charWidth = FONT_SIZE * CHAR_WIDTH_RATIO;

    // Video is declared in the server-rendered HTML with autoplay/muted/loop/playsinline
    const video = document.getElementById(
        "ascii-video",
    ) as HTMLVideoElement | null;
    if (!video) return () => {};

    const canvas = document.createElement("canvas");
    container.append(canvas);

    let gl: WebGL2RenderingContext | null = null;
    let videoTexture: WebGLTexture | null = null;
    let atlasTexture: WebGLTexture | null = null;
    let raf = 0;

    const teardown = () => {
        if (gl) {
            if (videoTexture) gl.deleteTexture(videoTexture);
            if (atlasTexture) gl.deleteTexture(atlasTexture);
            gl = null;
        }
    };

    const init = () => {
        if (!video.videoWidth) return;

        teardown();

        const cols = Math.floor(window.innerWidth / charWidth) * 1.5;
        const rows = Math.floor(window.innerHeight / FONT_SIZE) * 1.5;
        const w = cols * charWidth;
        const h = rows * FONT_SIZE;

        canvas.width = w;
        canvas.height = h;

        gl = canvas.getContext("webgl2", {
            antialias: false,
            preserveDrawingBuffer: false,
        });
        if (!gl) return;

        const vert = compileShader(gl, VERTEX_SHADER, gl.VERTEX_SHADER);
        const frag = compileShader(gl, FRAGMENT_SHADER, gl.FRAGMENT_SHADER);
        if (!vert || !frag) return;

        const program = createProgram(gl, vert, frag);
        if (!program) return;

        gl.useProgram(program);
        createFullscreenQuad(gl, program);

        videoTexture = createVideoTexture(gl);
        atlasTexture = createAsciiAtlas(gl, chars, FONT_SIZE);

        const u = (name: string) => gl!.getUniformLocation(program!, name);

        gl.uniform1i(u("u_video"), 0);
        gl.uniform1i(u("u_asciiAtlas"), 1);
        gl.uniform2f(u("u_resolution"), w, h);
        gl.uniform2f(u("u_charSize"), charWidth, FONT_SIZE);
        gl.uniform2f(u("u_gridSize"), cols, rows);
        gl.uniform1f(u("u_numChars"), chars.length);
        gl.uniform1i(u("u_colored"), 0);
        gl.uniform1f(u("u_blend"), 0);
        gl.uniform1f(u("u_highlight"), 0);
        gl.uniform1f(u("u_brightness"), 1.0);
        gl.uniform2f(u("u_mouse"), -1, -1);
        gl.uniform1f(u("u_mouseRadius"), 0);
        gl.uniform1i(u("u_trailLength"), 0);
        gl.uniform1f(u("u_rippleEnabled"), 0);
        gl.uniform1f(u("u_audioLevel"), 0);
        gl.uniform1f(u("u_audioReactivity"), 0);
        gl.uniform1f(u("u_audioSensitivity"), 0);

        gl.viewport(0, 0, w, h);
    };

    const render = () => {
        if (gl && video.readyState >= 2) {
            gl.activeTexture(gl.TEXTURE0);
            gl.bindTexture(gl.TEXTURE_2D, videoTexture);
            gl.texImage2D(
                gl.TEXTURE_2D,
                0,
                gl.RGBA,
                gl.RGBA,
                gl.UNSIGNED_BYTE,
                video,
            );
            gl.generateMipmap(gl.TEXTURE_2D);

            gl.activeTexture(gl.TEXTURE1);
            gl.bindTexture(gl.TEXTURE_2D, atlasTexture);

            gl.drawArrays(gl.TRIANGLES, 0, 6);
        }

        raf = requestAnimationFrame(render);
    };

    const onResize = () => init();
    window.addEventListener("resize", onResize);

    const startRender = () => {
        if (!raf) raf = requestAnimationFrame(render);
    };

    const tryPlay = () => {
        video
            .play()
            .then(startRender)
            .catch(() => {
                document.addEventListener(
                    "click",
                    () =>
                        video
                            .play()
                            .then(startRender)
                            .catch(() => {}),
                    { once: true },
                );
                document.addEventListener(
                    "touchstart",
                    () =>
                        video
                            .play()
                            .then(startRender)
                            .catch(() => {}),
                    { once: true },
                );
            });
    };

    if (video.readyState >= 1) {
        init();
        tryPlay();
    } else {
        video.addEventListener("loadedmetadata", () => {
            init();
            tryPlay();
        });
    }

    return () => {
        window.removeEventListener("resize", onResize);
        cancelAnimationFrame(raf);
        video.pause();
        teardown();
        canvas.remove();
        // Don't remove the video — it lives in the server-rendered HTML
    };
}
