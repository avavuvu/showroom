import type { ThemePreview } from "@setups"

export const themePreview: ThemePreview = (form) => {
    const style = document.getElementById("theme")

    let timer: ReturnType<typeof setTimeout> | undefined
    let request: AbortController | undefined

    const refresh = async () => {
        if (!style) {
            return
        }
        request?.abort()
        const controller = (request = new AbortController())
        try {
            const response = await fetch(`${form.action}/preview`, {
                method: "POST",
                body: new URLSearchParams(new FormData(form) as unknown as Array<Array<string>>),
                signal: controller.signal,
            })
            if (response.ok) {
                style.textContent = await response.text()
            }
        } catch {}
    }

    return {
        schedule: () => {
            clearTimeout(timer)
            timer = setTimeout(refresh, 100)
        },

        disconnect: () => {
            clearTimeout(timer)
            request?.abort()
        },
    }
}
