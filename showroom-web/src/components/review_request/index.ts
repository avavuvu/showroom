import type { ReviewRequest } from "@setups"

export const reviewRequest: ReviewRequest = (_root, refs) => {
    const names = () => [...refs.rows.querySelectorAll<HTMLInputElement>('input[name="reviewer"]')]

    return {
        addRow: () => {
            const row = refs.rowTemplate.content.firstElementChild?.cloneNode(true)
            if (row instanceof HTMLElement) {
                refs.rows.append(row)
                row.querySelector("input")?.focus()
            }
        },

        removeRow: (_event, button) => {
            const row = button.closest(".reviewer-row")
            if (names().length > 1) {
                row?.remove()
                return
            }
            const input = row?.querySelector("input")
            if (input) {
                input.value = ""
            }
        },
    }
}
