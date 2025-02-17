use postflop_solver::*;

fn main() {
    // 計測開始
    let start_time = Instant::now();

    // see `basic.rs` for the explanation of the following code

    let oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s";
    let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+";

    let card_config = CardConfig {
        range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
        flop: flop_from_str("Td9d6h").unwrap(),
        turn: card_from_str("Qc").unwrap(),
        river: NOT_DEALT,
    };

    let bet_sizes = BetSizeOptions::try_from(("60%, e, a", "2.5x")).unwrap();

    let tree_config = TreeConfig {
        initial_state: BoardState::Turn,
        starting_pot: 200,
        effective_stack: 900,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        river_bet_sizes: [bet_sizes.clone(), bet_sizes],
        turn_donk_sizes: None,
        river_donk_sizes: Some(DonkSizeOptions::try_from("50%").unwrap()),
        add_allin_threshold: 1.5,
        force_allin_threshold: 0.15,
        merging_threshold: 0.1,
    };

    let action_tree = ActionTree::new(tree_config).unwrap();
    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();
    game.allocate_memory(false);

    let max_num_iterations = 1000;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.005;
    solve(&mut game, max_num_iterations, target_exploitability, true);

    // save the solved game tree to a file
    // 4th argument is zstd compression level (1-22); requires `zstd` feature to use
    save_data_to_file(&game, "memo string", "filename.bin", Some(10)).unwrap();

    // S3バケットへfilename.binを保存したい

    // S3バケットへ保存したファイル(filename.bin)を読み込みたい

    // load the solved game tree from a file
    // 2nd argument is the maximum memory usage in bytes
    let (mut game2, _memo_string): (PostFlopGame, _) =
        load_data_from_file("filename.bin", Some(10)).unwrap();

    // check if the loaded game tree is the same as the original one
    game.cache_normalized_weights();
    game2.cache_normalized_weights();
    assert_eq!(game.equity(0), game2.equity(0));

    // delete the file
    std::fs::remove_file("filename.bin").unwrap();

    // 計測終了
    let duration = start_time.elapsed();
    println!("処理にかかった時間: {:?}", duration);
}
