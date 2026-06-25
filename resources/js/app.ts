import { Application, type ControllerConstructor } from "@hotwired/stimulus";

const application = Application.start();

const modules = import.meta.glob<{ default: ControllerConstructor }>(
    "./controllers/*_controller.ts",
    { eager: true },
);

for (const path in modules) {
    const identifier = path
        .replace("./controllers/", "")
        .replace("_controller.ts", "")
        .replaceAll("_", "-");

    application.register(identifier, modules[path].default);
}
