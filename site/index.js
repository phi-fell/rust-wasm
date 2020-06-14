import init, { greet } from './graemarch.js';

async function run() {
    await init();
    greet();
}

run();
