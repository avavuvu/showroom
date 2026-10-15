import { getCharArray } from "../showroom-web/src/components/ascii_background/lib/ascii-charsets";
import {
    BRIGHTNESS,
    CHARSET,
    CHAR_WIDTH_RATIO,
    FONT_SIZE,
    GLYPH_COLOR,
    GLYPH_FONT,
    GLYPH_SCALE,
    SCALE,
} from "../showroom-web/src/components/ascii_background/lib/glyphs";

const VIEWPORTS = {
    landscape: [1680, 945],
    portrait: [390, 844],
} as const;

const [
    input = "showroom-web/resources/static/assets/flower-loop.webm",
    output = "showroom-web/resources/static/assets/flower-loop-ascii",
    frameArg = "0",
] = Bun.argv.slice(2);

const frame = Number.parseInt(frameArg, 10);
if (!Number.isInteger(frame) || frame < 0) throw new Error(`frame must be a non-negative integer, got "${frameArg}"`);

const run = (command: string[]) => {
    const result = Bun.spawnSync(command, { stdout: "pipe", stderr: "pipe" });
    if (!result.success) throw new Error(`${command[0]} failed: ${result.stderr.toString()}`);
    return result.stdout;
};

const [width, height] = run([
    "ffprobe", "-v", "error", "-select_streams", "v:0",
    "-show_entries", "stream=width,height", "-of", "csv=p=0", input,
]).toString().trim().split(",").map(Number);

const pixels = run([
    "ffmpeg", "-v", "error", "-i", input,
    "-vf", `select=eq(n\\,${frame}),scale=in_color_matrix=auto:in_range=auto,format=rgb24`,
    "-frames:v", "1", "-f", "rawvideo", "-",
]);

if (pixels.length !== width * height * 3) throw new Error(`no frame ${frame} in ${input}`);

const luma = (x: number, y: number) => {
    const i = (Math.min(height - 1, Math.max(0, y)) * width + Math.min(width - 1, Math.max(0, x))) * 3;
    return (0.299 * pixels[i] + 0.587 * pixels[i + 1] + 0.114 * pixels[i + 2]) / 255;
};

const sample = (u: number, v: number) => {
    const x = u * width - 0.5;
    const y = v * height - 0.5;
    const x0 = Math.floor(x);
    const y0 = Math.floor(y);
    const fx = x - x0;
    const fy = y - y0;
    const top = luma(x0, y0) * (1 - fx) + luma(x0 + 1, y0) * fx;
    const bottom = luma(x0, y0 + 1) * (1 - fx) + luma(x0 + 1, y0 + 1) * fx;
    return top * (1 - fy) + bottom * fy;
};

const adjust = (brightness: number) => {
    const adjusted = BRIGHTNESS <= 1 ? brightness * BRIGHTNESS : 1 - (1 - brightness) / BRIGHTNESS;
    return Math.min(1, Math.max(0, adjusted));
};

const escape = (text: string) =>
    text.replace(/[&<>"]/g, (char) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[char]!);

const chars = getCharArray(CHARSET);

const render = (viewportWidth: number, viewportHeight: number) => {
    const cols = Math.max(1, Math.floor((viewportWidth * SCALE) / (FONT_SIZE * CHAR_WIDTH_RATIO)));
    const rows = Math.max(1, Math.floor((viewportHeight * SCALE) / FONT_SIZE));
    const xs = Array.from({ length: cols }, (_, col) => col * FONT_SIZE + FONT_SIZE / 2).join(" ");

    const lines = Array.from({ length: rows }, (_, row) => {
        const text = Array.from({ length: cols }, (_, col) => {
            const brightness = adjust(sample((col + 0.5) / cols, (row + 0.5) / rows));
            return chars[Math.floor((1 - brightness) * (chars.length - 0.001))];
        }).join("");
        return `<text x="${xs}" y="${row * FONT_SIZE + FONT_SIZE / 2}">${escape(text)}</text>`;
    });

    return [
        `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${cols * FONT_SIZE} ${rows * FONT_SIZE}" preserveAspectRatio="none">`,
        `<rect width="100%" height="100%" fill="#fff"/>`,
        `<g font-family="${GLYPH_FONT}" font-size="${FONT_SIZE * GLYPH_SCALE}" fill="${GLYPH_COLOR}" text-anchor="middle" dominant-baseline="central" style="white-space:pre">`,
        ...lines,
        `</g>`,
        `</svg>`,
        ``,
    ].join("\n");
};

for (const [name, [viewportWidth, viewportHeight]] of Object.entries(VIEWPORTS)) {
    const svg = render(viewportWidth, viewportHeight);
    const path = `${output}-${name}.svg`;
    await Bun.write(path, svg);
    console.log(`${path}: ${svg.length} bytes, ${Bun.gzipSync(svg).length} gzipped`);
}
