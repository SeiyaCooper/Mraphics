import init, * as MRAPHICS from "../wasm-pkg/mraphics.js";

export async function withCanvas(canvasId, { fullScreen = true } = {}) {
    await init();

    if (fullScreen) {
        let canvas = document.getElementById(canvasId);
        canvas.style.width = "100%";
        canvas.style.height = "100%";
        canvas.style.display = "block";

        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
    }

    return new MRAPHICS.Canvas(canvasId);
}

export * from "../wasm-pkg/mraphics.js";
