import init, { start } from './graemarch.js';

async function run() {
    await init();
    start();
}

run();
