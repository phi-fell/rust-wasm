import init, { start_game } from './graemarch.js';

async function run() {
    await init();
    start_game();
}

run();
