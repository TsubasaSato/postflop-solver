use postflop_solver::*;
use std::time::Instant;

fn main() {
    // 計測開始
    let start_time = Instant::now();

    let oop_range = "AA:0.945,KK:0.79,QQ:0.87,JJ:0.83,TT:0.765,99:0.645,88:0.595,77-66:0.575,55-44:0.655,33:0.66,22:0.7,AQs+,AJs:0.995,ATs,A9s:0.795,A8s:0.83,A7s:0.625,A6s:0.575,A5s:0.595,A4s:0.625,A3s:0.41,A2s:0.665,AKo:0.79,AQo:0.86,AJo:0.755,ATo:0.745,A9o:0.97,A8o:0.885,A7o:0.76,A6o:0.705,A5o:0.805,A4o:0.61,A3o:0.5,KQs,KJs:0.92,KTs:0.94,K9s:0.735,K8s:0.6,K7s:0.735,K6s:0.895,K5s:0.8,K4s:0.83,K3s-K2s:0.885,KQo:0.65,KJo:0.71,KTo:0.76,K9o:0.75,K8o:0.605,K7o:0.145,K6o:0.005,QJs:0.565,QTs:0.51,Q9s:0.41,Q8s:0.875,Q7s:0.785,Q6s:0.865,Q5s:0.805,Q4s:0.725,Q3s:0.77,Q2s:0.78,QJo:0.85,QTo:0.8,Q9o:0.64,Q8o:0.1450004,JTs:0.735,J9s:0.59,J8s:0.79,J7s:0.82,J6s:0.785,J5s:0.76,J4s:0.67,J3s:0.45,J2s:0.18,JTo:0.74,J9o:0.705,J8o:0.33,T9s:0.595,T8s:0.545,T7s:0.835,T6s:0.77,T5s:0.37,T4s:0.115,T9o:0.71,T8o:0.595,98s:0.63,97s:0.995,96s:0.78,95s:0.295,98o:0.605,87s:0.78,86s:0.825,85s:0.77,87o:0.305,76s:0.885,75s:0.77,74s:0.475,65s:0.81,64s:0.78,54s:0.73,53s:0.745,43s:0.01";
    let ip_range = "TT:0.31,99:0.915,88:0.98,77,66:0.98,55-22,AJs:0.085,ATs-A8s,A7s:0.805,A6s,A3s:0.875,A2s:0.835,AQo:0.35,AJo:0.89,ATo:0.995,A9o-A8o,A7o:0.805,A6o:0.455,A5o:0.93,A4o:0.71,A3o:0.16,KQs:0.195,KJs:0.675,KTs:0.75,K9s:0.74,K8s-K7s,K6s:0.71,K5s:0.89,K4s:0.925,K3s:0.86,K2s:0.925,KQo:0.945,KJo:0.915,KTo:0.895,K9o:0.85,K8o:0.55,K7o:0.12000001,K6o:0.1,QJs:0.575,QTs:0.705,Q9s-Q2s,QTo+,Q9o:0.555,JTs:0.37,J9s:0.64,J8s,J7s:0.96,J6s-J4s,J3s:0.895,J2s:0.6,JTo,J9o:0.68,J8o:0.11,T9s:0.265,T8s:0.545,T7s-T6s,T5s:0.96,T4s:0.58,T3s:0.565,T2s:0.2,T9o,T8o:0.65,98s:0.385,97s:0.945,96s-95s,98o:0.79,87s:0.355,86s:0.965,85s,84s:0.465,87o:0.7,76s:0.37,75s-74s,76o:0.57,65s:0.41,64s-63s,65o:0.305,54s:0.43,53s:0.815,52s,54o:0.155,43s:0.67,42s,32s:0.95";

    let card_config = CardConfig {
        range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
        flop: flop_from_str("KhJd8c").unwrap(),
        turn: NOT_DEALT,
        river: NOT_DEALT,
    };

    let bet_sizes = BetSizeOptions::try_from(("33%, 50%, 75%, 125%", "50%, 100%")).unwrap();

    let tree_config = TreeConfig {
        initial_state: BoardState::Flop,
        starting_pot: 60,
        effective_stack: 970,
        rake_rate: 0.05,
        rake_cap: 80.0,
        flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        river_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        turn_donk_sizes: Some(DonkSizeOptions::try_from("33%").unwrap()),
        river_donk_sizes: Some(DonkSizeOptions::try_from("33%").unwrap()),
        add_allin_threshold: 1.5,
        force_allin_threshold: 0.15,
        merging_threshold: 0.0,
    };

    let action_tree = ActionTree::new(tree_config).unwrap();
    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

    // check memory usage
    let (mem_usage, mem_usage_compressed) = game.memory_usage();
    println!(
        "Memory usage without compression (32-bit float): {:.2}GB",
        mem_usage as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!(
        "Memory usage with compression (16-bit integer): {:.2}GB",
        mem_usage_compressed as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    // game.allocate_memory(true);

    // let max_num_iterations = 16_777_216f32;
    // let target_exploitability = game.tree_config().starting_pot as f32 * 0.001;
    // let exploitability = solve(&mut game, max_num_iterations, target_exploitability, true);
    // println!("Exploitability: {:.2}", exploitability);

    // game.cache_normalized_weights();
    // let equity = game.equity(0);
    // let ev = game.expected_values(0);
    // println!("Equity of oop_hands[0]: {:.2}%", 100.0 * equity[0]);
    // println!("EV of oop_hands[0]: {:.2}", ev[0]);

    // let weights = game.normalized_weights(0);
    // let average_equity = compute_average(&equity, weights);
    // let average_ev = compute_average(&ev, weights);
    // println!("Average equity: {:.2}%", 100.0 * average_equity);
    // println!("Average EV: {:.2}", average_ev);

    // 計測終了
    let duration = start_time.elapsed();
    println!("処理にかかった時間: {:?}", duration);
}
