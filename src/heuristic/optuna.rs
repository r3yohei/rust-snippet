/// グローバルなどで定義しているハイパーパラメータをコマンドライン引数に置き換えておき，
/// 外部からoptunaでよしなに渡してシミュレーションする
/// 最もスコアの良いハイパーパラメータを外部で自動で保存する
/// 以下はRustファイル内での単なる設定例

fn main() {
    get_time();
    // optuna用コマンドライン引数の設定見本
    let args: Vec<String> = std::env::args().collect();
    eprintln!("args: {:?}", args);
    let T0: f64 = args[1].parse::<f64>().unwrap();
    let T1: f64 = args[2].parse::<f64>().unwrap();
    let MULTI_START: f64 = args[3].parse::<f64>().unwrap();
    let PROB_2OPT: usize = args[4].parse::<usize>().unwrap();
    let PROB_3OPT: usize = args[5].parse::<usize>().unwrap();
    let PROB_INSERT: usize = args[6].parse::<usize>().unwrap();
    let PROB_MOVE_STATION: usize = args[7].parse::<usize>().unwrap();
    let PROB_INSERT_STATION: usize = args[8].parse::<usize>().unwrap();
    let MAX_DELTA: f64 = args[9].parse::<f64>().unwrap();
    let MIN_DELTA: f64 = args[10].parse::<f64>().unwrap();
    let KICK_THRESHOLD: usize = args[11].parse::<usize>().unwrap();
    let PROB_DOUBLE_BRIDGE: f64 = args[12].parse::<f64>().unwrap();
    let CALC_TEMP_FREQUENCY: usize = args[13].parse::<usize>().unwrap();
    let NUM_STATION: usize = args[14].parse::<usize>().unwrap();

    // 以下普通にシミュレートするプログラム
    todo!();
}