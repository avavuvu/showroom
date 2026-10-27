import type { CopyField } from "@setups"

export const copyField: CopyField = (_root, refs) => {
    let timer: ReturnType<typeof setTimeout> | undefined

    const write = async (text: string): Promise<boolean> => {
        try {
            await navigator.clipboard.writeText(text)
            return true
        } catch {
            refs.input.focus()
            refs.input.select()
            return document.execCommand("copy")
        }
    }

    return {
        select: () => refs.input.select(),

        copy: async () => {
            const button = refs.button
            if (!(await write(refs.input.value)) || !button) {
                return
            }
            button.textContent = "Copied"
            clearTimeout(timer)
            timer = setTimeout(() => {
                button.textContent = "Copy"
            }, 2_000)
        },

        disconnect: () => clearTimeout(timer),
    }
}
