import type { AsciiBackground } from "@setups"
import {
    VERTEX_SHADER,
    FRAGMENT_SHADER,
    compileShader,
    createProgram,
    createFullscreenQuad,
    createVideoTexture,
    createAsciiAtlas,
} from "./lib/webgl"
import { getCharArray } from "./lib/ascii-charsets"
import { BRIGHTNESS, CHARSET, CHAR_WIDTH_RATIO, FONT_SIZE, SCALE } from "./lib/glyphs"

const prepareRenderer = (gl: WebGL2RenderingContext, chars: Array<string>) => {
    const vertexShader = compileShader(gl, VERTEX_SHADER, gl.VERTEX_SHADER)
    const fragmentShader = compileShader(gl, FRAGMENT_SHADER, gl.FRAGMENT_SHADER)
    if (!vertexShader || !fragmentShader) {
        return null
    }

    const program = createProgram(gl, vertexShader, fragmentShader)
    if (!program) {
        return null
    }

    gl.useProgram(program)
    createFullscreenQuad(gl, program)

    gl.activeTexture(gl.TEXTURE1)
    if (!createAsciiAtlas(gl, chars, FONT_SIZE)) {
        return null
    }
    gl.activeTexture(gl.TEXTURE0)
    if (!createVideoTexture(gl)) {
        return null
    }

    const getUniform = (name: string) => gl.getUniformLocation(program, name)

    gl.uniform1i(getUniform("u_video"), 0)
    gl.uniform1i(getUniform("u_asciiAtlas"), 1)
    gl.uniform1f(getUniform("u_numChars"), chars.length)
    gl.uniform1f(getUniform("u_brightness"), BRIGHTNESS)

    return { gridSize: getUniform("u_gridSize") }
}

export const asciiBackground: AsciiBackground = (root, { video, canvas }, { signal }) => {
    const gl = canvas.getContext("webgl2", {
        antialias: false,
        depth: false,
        stencil: false,
        powerPreference: "low-power",
        preserveDrawingBuffer: false,
    })
    const renderer = gl && prepareRenderer(gl, getCharArray(CHARSET))

    if (!gl || !renderer) {
        video.pause()
        gl?.getExtension("WEBGL_lose_context")?.loseContext()
        return
    }

    const charWidth = FONT_SIZE * CHAR_WIDTH_RATIO
    const reducedMotion = matchMedia("(prefers-reduced-motion: reduce)")

    let columns = 0
    let rows = 0
    let drawnTime = -1
    let animationFrame = 0
    let isOnscreen = true

    const draw = () => {
        if (video.readyState < video.HAVE_CURRENT_DATA) {
            return
        }
        drawnTime = video.currentTime
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, video)
        gl.generateMipmap(gl.TEXTURE_2D)
        gl.drawArrays(gl.TRIANGLES, 0, 6)
        root.toggleAttribute("data-live", true)
    }

    const schedule = () => {
        if (!animationFrame) {
            animationFrame = requestAnimationFrame(handleFrame)
        }
    }

    const handleFrame = () => {
        animationFrame = 0
        if (video.currentTime !== drawnTime) {
            draw()
        }
        if (!video.paused) {
            schedule()
        }
    }

    const resize = () => {
        const nextColumns = Math.max(1, Math.floor((root.clientWidth * SCALE) / charWidth))
        const nextRows = Math.max(1, Math.floor((root.clientHeight * SCALE) / FONT_SIZE))
        if (nextColumns === columns && nextRows === rows) {
            return
        }

        columns = nextColumns
        rows = nextRows
        canvas.width = columns * charWidth
        canvas.height = rows * FONT_SIZE
        gl.viewport(0, 0, canvas.width, canvas.height)
        gl.uniform2f(renderer.gridSize, columns, rows)
        draw()
    }

    const play = () => {
        video.play().catch((error: unknown) => {
            if (signal.aborted || !(error instanceof DOMException) || error.name !== "NotAllowedError") {
                return
            }
            document.addEventListener("click", update, { once: true, signal })
            document.addEventListener("touchstart", update, { once: true, passive: true, signal })
        })
    }

    const update = () => {
        if (isOnscreen && document.visibilityState === "visible" && !reducedMotion.matches) {
            play()
            schedule()
        } else {
            video.pause()
        }
    }

    video.addEventListener("loadeddata", draw, { signal })
    video.addEventListener("playing", schedule, { signal })
    document.addEventListener("visibilitychange", update, { signal })
    reducedMotion.addEventListener("change", update, { signal })

    const resizeObserver = new ResizeObserver(resize)
    resizeObserver.observe(root)

    const intersectionObserver = new IntersectionObserver((entries) => {
        isOnscreen = entries.at(-1)?.isIntersecting ?? isOnscreen
        update()
    })
    intersectionObserver.observe(root)

    resize()
    update()

    return {
        disconnect: () => {
            resizeObserver.disconnect()
            intersectionObserver.disconnect()
            cancelAnimationFrame(animationFrame)
            video.pause()
            gl.getExtension("WEBGL_lose_context")?.loseContext()
            root.removeAttribute("data-live")
        },
    }
}
