import type { FruitsBackground } from "@setups"

const FRUIT_COUNT = 10
const AREA_PER_FRUIT = 10_000
const MAX_BACKING = 4096

type Drop = {
    image: HTMLImageElement
    x: number
    y: number
    depth: number
    rotation: number
    spin: number
}

const random = (min: number, max: number) => min + Math.random() * (max - min)

const loadImage = (index: number) =>
    new Promise<HTMLImageElement | null>((resolve) => {
        const image = new Image()
        image.onload = () => resolve(image)
        image.onerror = () => resolve(null)
        image.src = `/avatars/${index}.webp`
    })

export const fruitsBackground: FruitsBackground = (_root, { canvas }, { signal }) => {
    const context = canvas.getContext("2d")
    const host = canvas.parentElement
    if (!context || !host) {
        return { disconnect: () => {} }
    }

    const isStill = window.matchMedia("(prefers-reduced-motion: reduce)").matches

    let images: Array<HTMLImageElement> = []
    let drops: Array<Drop> = []
    let width = 0
    let height = 0
    let frame = 0
    let last = 0
    let isStopped = false

    const createDrop = (): Drop => {
        const depth = random(0.4, 1)
        return {
            image: images[Math.floor(Math.random() * images.length)],
            x: random(0, width),
            y: random(-80, height),
            depth,
            rotation: random(0, Math.PI * 2),
            spin: random(-0.8, 0.8),
        }
    }

    const resize = () => {
        const rect = host.getBoundingClientRect()
        if (rect.width === 0 || rect.height === 0) {
            return false
        }

        width = rect.width
        height = rect.height

        const ratio = Math.min(window.devicePixelRatio || 1, MAX_BACKING / Math.max(width, height))
        canvas.style.width = `${width}px`
        canvas.style.height = `${height}px`
        canvas.width = Math.round(width * ratio)
        canvas.height = Math.round(height * ratio)
        context.setTransform(ratio, 0, 0, ratio, 0, 0)
        return true
    }

    const draw = () => {
        context.clearRect(0, 0, width, height)
        for (const drop of drops) {
            const size = 100 * drop.depth
            const drawnWidth = (size * drop.image.naturalWidth) / drop.image.naturalHeight
            context.save()
            context.translate(drop.x, drop.y)
            context.rotate(drop.rotation)
            context.drawImage(drop.image, -drawnWidth / 2, -size / 2, drawnWidth, size)
            context.restore()
        }
    }

    const tick = (now: number) => {
        const delta = Math.min((now - last) / 1000, 0.05)
        last = now

        for (const drop of drops) {
            const size = 80 * drop.depth
            drop.y += (80 + 180 * drop.depth) * delta
            drop.rotation += drop.spin * delta

            if (drop.y - size > height) {
                drop.y = -size
                drop.x = random(0, width)
            }
        }

        draw()
        frame = requestAnimationFrame(tick)
    }

    const observer = new ResizeObserver(() => {
        if (!resize()) {
            return
        }

        const target = Math.round((width * height) / AREA_PER_FRUIT)
        while (drops.length < target) {
            drops.push(createDrop())
        }
        drops = drops.slice(0, target).sort((a, b) => a.depth - b.depth)

        if (isStill) {
            draw()
        }
    })

    const disconnect = () => {
        isStopped = true
        cancelAnimationFrame(frame)
        observer.disconnect()
        signal.removeEventListener("abort", disconnect)
        context.clearRect(0, 0, width, height)
    }

    signal.addEventListener("abort", disconnect, { once: true })

    Promise.all(Array.from({ length: FRUIT_COUNT }, (_, index) => loadImage(index))).then((loaded) => {
        images = loaded.filter((image): image is HTMLImageElement => image !== null)
        if (isStopped || images.length === 0) {
            return
        }

        observer.observe(host)
        if (!isStill) {
            frame = requestAnimationFrame(tick)
        }
    })

    return { disconnect }
}
