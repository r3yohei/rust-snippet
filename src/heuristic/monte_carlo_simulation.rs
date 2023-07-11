use cargo_snippet::snippet;
use proconio::{marker::*, source::line::LineSource, *};
use std::io::*;
use std::{collections::*, fmt::format};
use std::{cmp::*, vec};
use rand_pcg::Pcg64Mcg;

const N: usize = 36;
const TURN: usize = 35; // 操作回数
const SEED: u128 = 8_192;
const PLAYOUT_EPOCH: usize = 100; // 期待値計算のためにプレイアウトする回数
const TL: f64 = 1.98; // 全体の制限時間
const NUM_OP: usize = 4;
const OP: [char; NUM_OP] = ['U', 'D', 'L', 'R'];

type Input = char;
type Operation = char;

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        // ローカル環境とジャッジ環境の実行速度差はget_timeで吸収しておくと便利
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 1.5
        }
        #[cfg(not(feature = "local"))]
        {
            (ms - STIME)
        }
    }
}

#[derive(Clone)]
struct State {
    turn: usize,
    board: Vec<i64>,
}
impl State {
    fn new(input: &Input) -> Self {
        Self {
            turn: 0,
            board: input.board.clone(),
        }
    }

    fn apply(&mut self, op: Operation, input: Input) {
        self.turn += 1;
        todo!();
    }

    fn score(&self, input: &Input) -> i64 {
        todo!();
    }
}

// 現在の状態をtとして，t+1~最後まで遊んだ時のスコアの期待値が最も高いt+1番目の手を返す
// 現在受け取った入力で行う合法手を全て試し，それより先の入力をt+1~TURN個勝手に決めてプレイしまくる
// 最後までプレイアウトした時のスコアの期待値が一番高いものをt番目の手とする
fn playout(mut rng: &mut Pcg64Mcg, state: &State, input: Input) -> usize {
    // t番目の手とスコアの期待値のmap
    let mut map: HashMap<usize, i64> = HashMap::new();
    // t番目の手を固定して，t+1~36までを何回か遊ぶ
    for &op in &OP {
        // 各シミュレーションでのスコアを格納する(後で平均を出す)
        let mut scores = vec![];
        
        for _ in 0..PLAYOUT_EPOCH {
            // まずはt番目は固定した手を実行
            let mut state_tmp = state.clone();
            state_tmp.apply(op, input);

            // 残りターンの入力を適当に生成する
            let mut sim_inputs = vec![];
            for i in (1..=(TURN-state_tmp.turn)).rev() {
                sim_inputs.push(rng.gen_range(1, i+1));
            }

            // 適当に手を実行することを繰り返す
            // 強いルールベースが望ましいが，ランダムでもOK
            for (i, t) in (state_tmp.turn..=TURN).enumerate() {
                let sim_input = sim_inputs[i];
                let sim_op_idx = rng.gen_range(0, NUM_OP);
                state_tmp.apply(OP[sim_op_idx], sim_input);
            }

            // 終わったらスコア計算
            let score = state_tmp.score();
            scores.push(score);
        }
        let mean = scores.iter().sum::<i64>() / scores.len() as i64;
        map.insert(p, mean);
    }
    // mapをソートして最大を取得
    let (p_max, _) = map.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    *p_max
}

fn monte_carlo_simulation() {
    let mut rng = Pcg64Mcg::new(SEED);
    let mut state = State::new();
    
    for t in 0..TURN {
        // 入力を受け取る
        let mut stdin = LineSource::new(BufReader::new(stdin()));
        input! {
            from &mut stdin,
            input: Input,
        }
        // ランダムプレイアウトして，t番目の手を決める
        let op = playout(&mut rng, &state, input);
        state.apply(op, input);
        
        // 回答を出力し，フラッシュする
        println!("{}", op);
        stdout().flush().unwrap();
    }

    eprintln!("=== Monte Carlo Simulation ===");
    eprintln!("time: {}", get_time());
}

fn main() {
    get_time();
    monte_carlo_simulation();
}
