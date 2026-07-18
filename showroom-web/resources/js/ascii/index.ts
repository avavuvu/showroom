const asciiContainer = document.getElementById("ascii-background");
if (asciiContainer) {
    import("./background").then(({ initAsciiBackground }) => {
        initAsciiBackground(asciiContainer);
    });
}
