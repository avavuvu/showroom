import type { Dropdown } from "@setups"

export const dropdown: Dropdown = (details) => ({
    close: (event) => {
        if (!details.open) {
            return
        }
        event.preventDefault()
        details.open = false
        details.querySelector("summary")?.focus()
    },

    outside: (event) => {
        if (details.open && !details.contains(event.target as Node)) {
            details.open = false
        }
    },
})
