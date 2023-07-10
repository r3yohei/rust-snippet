#![allow(non_snake_case, unused)]

use itertools::*;
use proconio::{marker::*, source::line::LineSource, *};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use std::io::*;
use std::{cmp::*, vec};
use std::{collections::*, fmt::format};

const N: usize = 100;
const SEED: u128 = 8_192;
const INF: i64 = 1_000_000_000;
const TL: f64 = 1.989;
const T0: f64 = 10e5; // 焼きなまし初期温度
const T1: f64 = 10e2; // 焼きなまし終温度
const MULTIPOINT: f64 = 3.0; // 多点スタートする回数
const CALC_TEMP_FREQUENCY: usize = 30; // 温度を再計算する頻度
const KICK_THRESHOLD: usize = 100; // スコア改善がこの回数見られなければkickを入れる
const NUM_OP: usize = 4;
const OP: [char; NUM_OP] = ['U', 'D', 'L', 'R'];

type Operation = char;

fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
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
struct Input {
    board: Vec<i64>,
}
impl Input {
    fn new() -> Self {
        input! {
            board: [i64; N],
        }

        Self { board }
    }
}

#[derive(Clone)]
struct State {
    board: Vec<i64>,
}
impl State {
    fn new(input: &Input) -> Self {
        Self {
            board: input.board.clone(),
        }
    }

    fn apply(&mut self, op: Operation) {
        todo!();
    }

    fn revert(&mut self, op: Operation) {
        todo!();
    }

    fn score(&self, input: &Input) -> i64 {
        todo!();
    }

    fn print(&self, input: &Input) {
        todo!();
    }
}

fn simulated_annealing(input: &Input) {
    let mut best_state = State::new();
    let mut best_score = best_state.score(&input);

    let mut total_iter = 0;
    let mut mutipoint = 0.0;
    let mut accepted_count = 0;
    let mut update_best_count = 0;
    while get_time() < TL {
        // 多点スタートで振り出しに戻る箇所
        mutipoint += 1.0;
        // 乱数シードも変更する
        let mut rng = Pcg64Mcg::new(SEED + mutipoint as u64);
        // stateを初期のものに変更する
        let mut crt_state = State::new();
        let mut crt_score = crt_state.score(&input);

        // 各多点のイテレーションは，TL / MULTI_START秒ずつ割り当てられる
        while get_time() < TL * mutipoint / MULTIPOINT {
            total_iter += 1;
            let t = get_time() / (TL * mutipoint / MULTIPOINT);
            let T = T0.powf(1.0 - t) * T1.powf(t);

            // なんらか2点選んで操作とか
            let s0 = rng.gen_range(0, N - 1);
            let s1 = rng.gen_range(s1, N);

            crt_state.apply();
            let next_score = crt_state.score(&input);

            if crt_score < next_score
                || rng.gen_bool(f64::exp(-(crt_score - next_score) as f64 / T))
            {
                // 改善解か，悪化でも許容範囲内なら採用する
                crt_score = next_score;
                accepted_count += 1;
            } else {
                // そうでないならリバートする
                crt_state.revert();
            }

            // スコアがベストを更新するなら，その構造体を保存する
            if best_score < crt_score {
                best_state = crt_state.clone();
                best_score = crt_score;
                update_best_count += 1;
            }
        }
        eprintln!("=== multipoint simulated annealing ===");
        eprintln!("multipoint: {}", mutipoint);
        eprintln!("total iter: {}", total_iter);
        eprintln!("accepted: {}", accepted_count);
        eprintln!("update best: {}", update_best_count);
        eprintln!("score: {}", best_score);
        eprintln!("time: {}", get_time());
        eprintln!();
    }

    // 最後に最も良い回答を出力
    best_state.print();
}

fn main() {
    get_time();
    let input = Input::new();
    simulated_annealing(&input);
}