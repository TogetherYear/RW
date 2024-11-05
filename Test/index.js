import init, { ConvertPngImageFileToBase64 } from '../Build/index.js';

await init(new Object());

const input = document.querySelector('.Input');

input.addEventListener('change', (e) => {
    const file = e.target.files[0];
    const reader = new FileReader();
    reader.readAsArrayBuffer(file);
    reader.onload = () => {
        const pngBuffer = new Uint8Array(reader.result);
        ConvertPngImageFileToBase64(pngBuffer)
            .then((res) => {
                console.log(res);
            })
            .catch((err) => {
                console.error('err:', err);
            });
    };
});
