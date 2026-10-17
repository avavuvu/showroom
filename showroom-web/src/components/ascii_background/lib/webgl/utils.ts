import { GLYPH_COLOR, GLYPH_FONT, GLYPH_SCALE } from "../glyphs"

// Compiles a GLSL shader from source code
export const compileShader = (
    gl: WebGL2RenderingContext,
    source: string,
    type: number,
): WebGLShader | null => {
    const shader = gl.createShader(type)
    if (!shader) {
        console.error("Failed to create shader")
        return null
    }

    gl.shaderSource(shader, source)
    gl.compileShader(shader)

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error("Shader compile error:", gl.getShaderInfoLog(shader))
        gl.deleteShader(shader)
        return null
    }

    return shader
}

// Links vertex and fragment shaders into a program
export const createProgram = (
    gl: WebGL2RenderingContext,
    vertexShader: WebGLShader,
    fragmentShader: WebGLShader,
): WebGLProgram | null => {
    const program = gl.createProgram()
    if (!program) {
        console.error("Failed to create program")
        return null
    }

    gl.attachShader(program, vertexShader)
    gl.attachShader(program, fragmentShader)
    gl.linkProgram(program)

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        console.error("Program link error:", gl.getProgramInfoLog(program))
        gl.deleteProgram(program)
        return null
    }

    return program
}

// Creates a fullscreen quad (two triangles covering the viewport)
export const createFullscreenQuad = (
    gl: WebGL2RenderingContext,
    program: WebGLProgram,
): void => {
    // Vertex positions in clip space (-1 to 1)
    const positions = new Float32Array([
        -1,
        -1, // bottom-left
        1,
        -1, // bottom-right
        -1,
        1, // top-left
        -1,
        1, // top-left
        1,
        -1, // bottom-right
        1,
        1, // top-right
    ])

    // Texture coords (0 to 1), Y flipped because video origin is top-left
    const textureCoordinates = new Float32Array([0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0])

    // Position attribute
    const positionBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)
    gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW)
    const positionLocation = gl.getAttribLocation(program, "a_position")
    gl.enableVertexAttribArray(positionLocation)
    gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0)

    // Texture coordinate attribute
    const textureCoordinateBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, textureCoordinateBuffer)
    gl.bufferData(gl.ARRAY_BUFFER, textureCoordinates, gl.STATIC_DRAW)
    const textureCoordinateLocation = gl.getAttribLocation(program, "a_texCoord")
    gl.enableVertexAttribArray(textureCoordinateLocation)
    gl.vertexAttribPointer(textureCoordinateLocation, 2, gl.FLOAT, false, 0, 0)
}

// Creates a texture for video frames with mipmapping for quality
export const createVideoTexture = (
    gl: WebGL2RenderingContext,
): WebGLTexture | null => {
    const texture = gl.createTexture()
    if (!texture) {
        return null
    }

    gl.bindTexture(gl.TEXTURE_2D, texture)

    // Mipmapping gives better quality when sampling large areas
    gl.texParameteri(
        gl.TEXTURE_2D,
        gl.TEXTURE_MIN_FILTER,
        gl.LINEAR_MIPMAP_LINEAR,
    )
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)

    return texture
}

// Renders ASCII characters to a horizontal strip texture
export const createAsciiAtlas = (
    gl: WebGL2RenderingContext,
    chars: Array<string>,
    charSize: number = 64,
): WebGLTexture | null => {
    // Draw characters to an offscreen canvas
    const canvas = document.createElement("canvas")
    canvas.width = charSize * chars.length
    canvas.height = charSize

    const context = canvas.getContext("2d")
    if (!context) {
        return null
    }

    context.fillStyle = "#000"
    context.fillRect(0, 0, canvas.width, canvas.height)
    context.fillStyle = GLYPH_COLOR
    context.font = `${charSize * GLYPH_SCALE}px ${GLYPH_FONT}`
    context.textAlign = "center"
    context.textBaseline = "middle"

    // Draw each character centered in its cell
    for (let index = 0; index < chars.length; index++) {
        const x = index * charSize + charSize / 2
        const y = charSize / 2
        context.fillText(chars[index], x, y)
    }

    // Upload canvas to GPU
    const texture = gl.createTexture()
    if (!texture) {
        return null
    }

    gl.bindTexture(gl.TEXTURE_2D, texture)
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, canvas)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)

    return texture
}

// Helper to set uniforms with cached locations
export type UniformSetter = {
    set1f: (name: string, value: number) => void
    set2f: (name: string, x: number, y: number) => void
    set1i: (name: string, value: number) => void
}

export const createUniformSetter = (
    gl: WebGL2RenderingContext,
    program: WebGLProgram,
): UniformSetter => {
    const cache = new Map<string, WebGLUniformLocation | null>()

    const getLocation = (name: string) => {
        if (!cache.has(name)) {
            cache.set(name, gl.getUniformLocation(program, name))
        }
        return cache.get(name)!
    }

    return {
        set1f: (name, value) => gl.uniform1f(getLocation(name), value),
        set2f: (name, x, y) => gl.uniform2f(getLocation(name), x, y),
        set1i: (name, value) => gl.uniform1i(getLocation(name), value),
    }
}

// Calculates ASCII grid dimensions from video aspect ratio
export const calculateGridDimensions = (
    videoWidth: number,
    videoHeight: number,
    cols: number,
): { cols: number, rows: number } => {
    const aspectRatio = videoWidth / videoHeight
    // Divide by 2 because chars are ~2x taller than wide
    const rows = Math.round(cols / aspectRatio / 2)
    return { cols, rows }
}
