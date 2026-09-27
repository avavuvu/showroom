export class SaveStatus extends HTMLElement {
    private update = (event: Event): void => {
        this.textContent = (event as CustomEvent<string>).detail;
    };

    connectedCallback(): void {
        window.addEventListener("save-status", this.update);
    }

    disconnectedCallback(): void {
        window.removeEventListener("save-status", this.update);
    }
}

if (!customElements.get("sr-save-status")) {
    customElements.define("sr-save-status", SaveStatus);
}
