import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
    static targets = ["email", "emailError", "password", "passwordToggle"];

    declare readonly emailTarget: HTMLInputElement;
    declare readonly emailErrorTarget: HTMLElement;
    declare readonly passwordTarget: HTMLInputElement;
    declare readonly passwordToggleTarget: HTMLButtonElement;

    validateEmail() {
        const { value } = this.emailTarget;
        let error = "";

        if (!value) {
            error = "Email is required";
        } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) {
            error = "Enter a valid email";
        }

        this.emailErrorTarget.textContent = error;
        this.emailErrorTarget.hidden = !error;
    }

    togglePassword() {
        const isPassword = this.passwordTarget.type === "password";
        this.passwordTarget.type = isPassword ? "text" : "password";
        this.passwordToggleTarget.textContent = isPassword ? "Hide" : "Show";
    }
}
