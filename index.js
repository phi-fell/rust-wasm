import init, { greet } from './pkg/graemarch.js';

async function run() {
    await init();
    greet();
}

run();
