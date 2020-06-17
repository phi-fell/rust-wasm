import init, { start_game } from './gotg_client.js';

async function run() {
    await init();
    start_game();
}

run();
