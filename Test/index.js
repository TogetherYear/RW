import init, { Demo } from '../Build/index.js';

await init(new Object());

Demo().then(() => {
    console.log('Wasm');
});
