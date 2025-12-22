import init, { Fire, ColorMode } from "./wasm_playground.js";

async function run() {
    const wasm = await init();

    const fire = new Fire();
    const image = new ImageData(new Uint8ClampedArray(wasm.memory.buffer, fire.texture, 4 * fire.len), fire.width, fire.height);

    const canvas = document.getElementById("fire-canvas");
    canvas.width = fire.width;
    canvas.height = fire.height;

    const ctx = canvas.getContext('2d');

    var color = document.querySelector('input[name="color"]:checked').value;
    document.getElementsByName("color").forEach(function (e) { e.oninput = function () { color = this.value } });

    const renderLoop = () => {
        fire.tick(ColorMode[color]);
        ctx.putImageData(image, 0, 0);
        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
}

run();