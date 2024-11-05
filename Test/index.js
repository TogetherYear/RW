import init, { Run, ConvertImage } from '../Build/index.js';

await init(new Object());

const input = document.querySelector('.Input');

input.addEventListener('change', (e) => {
    const file = e.target.files[0];
    const reader = new FileReader();
    reader.readAsArrayBuffer(file);
    reader.onload = () => {
        const uint8Array = new Uint8Array(reader.result);
        let s = ConvertImage(uint8Array);
        console.log(s);
    };
});
