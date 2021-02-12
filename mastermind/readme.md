# Mastermind

Minimax solver for mastermind game

https://en.wikipedia.org/wiki/Mastermind_(board_game)

## Build and run

    cargo run

# Example

    # Create game
    game = Mastermind::new_game(vec:[1,3,4,0],6,3);

    # Find result of guess
    eval_guess(game.hidden_colours,vec:[0,2,1,0],5);

    # Get recommended best move
    game.get_best_guess();



