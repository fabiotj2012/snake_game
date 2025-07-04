// src/index.js

// Import the wasm module and the structs we exposed
import init, { Game, Direction } from '../pkg/snake_game.js';

async function run() {
    // Initialize the WebAssembly module
    const wasm = await init();

    const GRID_SIZE = 20; // 20x20 grid
    const PIXEL_SCALE = 20; // Each grid cell will be 20x20 pixels

    const canvas = document.getElementById('game-canvas');
    const ctx = canvas.getContext('2d');

    canvas.width = GRID_SIZE * PIXEL_SCALE;
    canvas.height = GRID_SIZE * PIXEL_SCALE;

    // Create a new game instance from our Rust code
    // Note the `new` keyword, as we defined a constructor in wasm_bindgen
    const game = new Game(GRID_SIZE, GRID_SIZE);

    // Map key codes to our Rust enum directions
    document.addEventListener('keydown', (event) => {
        switch (event.key) {
            case 'ArrowUp':
                game.change_snake_direction(Direction.Up);
                break;
            case 'ArrowDown':
                game.change_snake_direction(Direction.Down);
                break;
            case 'ArrowLeft':
                game.change_snake_direction(Direction.Left);
                break;
            case 'ArrowRight':
                game.change_snake_direction(Direction.Right);
                break;
            case ' ': // Spacebar
                if (!game.game_started() || game.game_over()) {
                    game.start_game();
                }
                break;
        }
    });

    function draw() {
        // Clear the canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Draw the food
        // The `food()` getter returns a copy of the Point struct
        const food = game.food();
        ctx.fillStyle = 'red';
        ctx.fillRect(food.x * PIXEL_SCALE, food.y * PIXEL_SCALE, PIXEL_SCALE, PIXEL_SCALE);

        // Draw the snake
        // To read the snake's body, we get the pointer and length, THEN the memory
        const snakeBodyPtr = game.get_body_ptr();
        const snakeBodyLen = game.get_body_len();
        const wasmMemory = new Uint8Array(wasm.memory.buffer);
        
        // Each Point (x, y) has 2 * i32 = 8 bytes.
        const snakeCells = new Int32Array(wasmMemory.buffer, snakeBodyPtr, snakeBodyLen * 2);

        ctx.fillStyle = 'lime';
        for (let i = 0; i < snakeBodyLen; i++) {
            const x = snakeCells[i * 2];
            const y = snakeCells[i * 2 + 1];
            ctx.fillRect(x * PIXEL_SCALE, y * PIXEL_SCALE, PIXEL_SCALE, PIXEL_SCALE);
        }

        // Draw score
        ctx.fillStyle = 'white';
        ctx.font = '16px Arial';
        ctx.textAlign = 'right'; // Align text to the right
        ctx.fillText(`Score: ${game.score()}`, canvas.width - 10, 25); // Position at top-right

        // Draw start/game over message
        if (!game.game_started() || game.game_over()) {
            let message;
            if (!game.game_started()) {
                message = "Press SPACE to Start";
            } else {
                message = `Game Over! Score: ${game.score()}\nPress SPACE to Restart`;
            }

            ctx.fillStyle = 'white';
            ctx.font = '30px Arial';
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';

            const lines = message.split('\n');
            const lineHeight = 30; // Based on font size
            const startY = canvas.height / 2 - (lines.length - 1) * lineHeight / 2;

            lines.forEach((line, index) => {
                ctx.fillText(line, canvas.width / 2, startY + index * lineHeight);
            });
        }
    }

    function gameLoop() {
        // Only tick if the game is started and not over
        if (game.game_started() && !game.game_over()) {
            game.tick();
        }
        // Draw the new state
        draw();

        // Call the next frame
        setTimeout(() => {
            requestAnimationFrame(gameLoop);
        }, 1000 / 10); // Controls game speed (10 fps)
    }

    // Start the game loop
    requestAnimationFrame(gameLoop);
}

run().catch(console.error);