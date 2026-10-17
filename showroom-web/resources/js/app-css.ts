import.meta.glob(["../../src/views/layouts/**/*.css", "!../../src/**/*.email.css"], { eager: true })
import.meta.glob(
    [
        "../../src/**/*.css",
        "!../../src/views/layouts/**",
        "!../../src/**/*.email.css",
        "!../../src/components/editor/**",
        "!../../src/blocks/*/editor/**",
    ],
    { eager: true },
)
