import init, { run_app } from './pkg/yewchat.js';

async function main() {
    await init();
    run_app();
}

main();
