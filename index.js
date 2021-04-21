import init, { Fire, ColorMode } from "./wasm_playground.js";

async function run() {
    const i = await init();

    const width = 256;
    const height = 112;

    const canvas = document.getElementById("fire-canvas");
    canvas.height = height;
    canvas.width = width;

    const ctx = canvas.getContext('2d');

    const fire = Fire.new(width, height);
    const image = new ImageData(new Uint8ClampedArray(i.memory.buffer, fire.texture(), 4 * width * height), width, height);

    const height_slider = document.getElementById("height_param");
    const spread_slider = document.getElementById("spread_param");

    var color = document.querySelector('input[name="color"]:checked').value;
    document.getElementsByName("color").forEach(function (e) { e.oninput = function () { color = this.value } });

    const renderLoop = () => {
        fire.tick(spread_slider.value, height_slider.value, ColorMode[color]);
        ctx.putImageData(image, 0, 0);
        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
}

run();