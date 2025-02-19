use postflop_solver::*;
use std::time::Instant;

fn main() {
    // 計測開始
    let start_time = Instant::now();

    // OOPとIPのレンジを文字列形式で定義
    // `Range` のドキュメントを参照してフォーマットの詳細を確認
    let oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s";
    let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+";

    let card_config = CardConfig {
        range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
        flop: flop_from_str("Td9d6h").unwrap(),
        turn: card_from_str("Qc").unwrap(),
        river: NOT_DEALT,
    };

    // ベットサイズの定義 -> ポットの60%、幾何学的サイズ、およびオールイン
    // レイズサイズ -> 前回のベットの2.5倍
    // `BetSizeOptions` のドキュメントを参照して詳細を確認
    let bet_sizes = BetSizeOptions::try_from(("60%, e, a", "2.5x")).unwrap();

    let tree_config = TreeConfig {
        initial_state: BoardState::Turn, // `card_config` と一致する必要がある
        starting_pot: 200,
        effective_stack: 900,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()], // [OOP, IP]
        turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        river_bet_sizes: [bet_sizes.clone(), bet_sizes],
        turn_donk_sizes: None, // デフォルトのベットサイズを使用
        river_donk_sizes: Some(DonkSizeOptions::try_from("50%").unwrap()),
        add_allin_threshold: 1.5, // (最大ベットサイズ) <= 1.5x ポットの場合にオールインを追加
        force_allin_threshold: 0.15, // (相手がコールした後のSPR) <= 0.15 の場合にオールインを強制
        merging_threshold: 0.1,
    };

    // ゲームツリーの構築
    // `ActionTree` は構築後に手動で編集可能
    let action_tree = ActionTree::new(tree_config).unwrap();
    let num_terminal_nodes = action_tree.get_num_terminal_nodes();

    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

    // プレイヤーのプライベートハンドを取得
    let oop_cards = game.private_cards(0);
    let oop_cards_str = holes_to_strings(oop_cards).unwrap();
    assert_eq!(
        &oop_cards_str[..10],
        &["5c4c", "Ac4c", "5d4d", "Ad4d", "5h4h", "Ah4h", "5s4s", "As4s", "6c5c", "7c5c"]
    );

    // メモリ使用量を確認
    let (mem_usage, mem_usage_compressed) = game.memory_usage();
    println!(
        "圧縮なしのメモリ使用量 (32ビット浮動小数点数): {:.2}GB",
        mem_usage as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!(
        "圧縮ありのメモリ使用量 (16ビット整数): {:.2}GB",
        mem_usage_compressed as f64 / (1024.0 * 1024.0 * 1024.0)
    );

    // 圧縮なしでメモリを確保 (32ビット浮動小数点数を使用)
    game.allocate_memory(false);

    // 圧縮ありでメモリを確保 (16ビット整数を使用)
    // game.allocate_memory(true);

    // ゲームの解を求める
    let max_num_iterations = 1000;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.005; // ポットの0.5%
    let exploitability = solve(&mut game, max_num_iterations, target_exploitability, true);
    // 計算量の導出
    let num_private_hands = oop_cards_str.len() as i32;
    println!("プレイヤーのハンド組み合わせ: {:?}", &num_private_hands);
    println!("終端ノード数: {:?}", &num_terminal_nodes);
    println!("可搾取量: {:?}", &exploitability);
    let complexity = compute_complexity(num_private_hands , num_terminal_nodes, exploitability as f64);
    println!("計算量: {:?}", complexity);

    panic!("停止処理");

    // println!("Exploitability: {:.2}", exploitability);

    

    // 手動でゲームの解を求める
    // for i in 0..max_num_iterations {
    //     solve_step(&game, i);
    //     if (i + 1) % 10 == 0 {
    //         let exploitability = compute_exploitability(&game);
    //         if exploitability <= target_exploitability {
    //             println!("Exploitability: {:.2}", exploitability);
    //             break;
    //         }
    //     }
    // }
    // finalize(&mut game);

    // 特定のハンドのエクイティとEVを取得
    game.cache_normalized_weights();
    let equity = game.equity(0); // `0` はOOPプレイヤーを意味する
    let ev = game.expected_values(0);
    println!("OOPの最初のハンドのエクイティ: {:.2}%", 100.0 * equity[0]);
    println!("OOPの最初のハンドのEV: {:.2}", ev[0]);

    // 全ハンドのエクイティとEVを取得
    let weights = game.normalized_weights(0);
    let average_equity = compute_average(&equity, weights);
    let average_ev = compute_average(&ev, weights);
    println!("平均エクイティ: {:.2}%", 100.0 * average_equity);
    println!("平均EV: {:.2}", average_ev);

    // OOPの利用可能なアクションを取得
    let actions = game.available_actions();
    assert_eq!(
        format!("{:?}", actions),
        "[Check, Bet(120), Bet(216), AllIn(900)]"
    );

    // `Bet(120)` をプレイ
    game.play(1);

    // IPの利用可能なアクションを取得
    let actions = game.available_actions();
    assert_eq!(format!("{:?}", actions), "[Fold, Call, Raise(300)]");

    // IPがナッツストレートでフォールドしないことを確認
    let ip_cards = game.private_cards(1);
    let strategy = game.strategy();
    assert_eq!(ip_cards.len(), 250);
    assert_eq!(strategy.len(), 750);

    let ksjs = holes_to_strings(ip_cards)
        .unwrap()
        .iter()
        .position(|s| s == "KsJs")
        .unwrap();

    // strategy[index] => Fold
    // strategy[index + ip_cards.len()] => Call
    // strategy[index + 2 * ip_cards.len()] => Raise(300)
    assert_eq!(strategy[ksjs], 0.0);
    assert!((strategy[ksjs] + strategy[ksjs + 250] + strategy[ksjs + 500] - 1.0).abs() < 1e-6);

    // `Call` をプレイ
    game.play(1);

    // 現在のノードがチャンスノード (リバーノード) であることを確認
    assert!(game.is_chance_node());

    // "7s" をディール可能であることを確認
    let card_7s = card_from_str("7s").unwrap();
    assert!(game.possible_cards() & (1 << card_7s) != 0);

    // "7s" をディール
    game.play(card_7s as usize);

    // ルートノードに戻る
    game.back_to_root();

    // 計測終了
    let duration = start_time.elapsed();
    println!("処理にかかった時間: {:?}", duration);
}
