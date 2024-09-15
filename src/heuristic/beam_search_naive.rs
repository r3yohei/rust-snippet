#![allow(non_snake_case, unused)]

use proconio::{*, marker::*, source::line::LineSource};
use rustc_hash::FxHashSet;
use std::io::*;
use std::{collections::*, fmt::format};
use std::{cmp::*, vec};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use itertools::Itertools;
use superslice::Ext;

const INF: i64 = 1 << 60;
const SEED: u128 = 8_192;
const TL: f64 = 1.989;
const BEAM_DEPTH: usize = 1000;
const BEAM_WIDTH: usize = 1000;

type Operation = char;


/// 現在時刻を返す
pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 1.5
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}


/// 最小･最大の交換
/// A.chmax(B)のようにすることで，もしA<BならA=Bとしてtrueを返し，そうでなければAのまま保持してfalseを返す
pub trait ChangeMinMax {
    fn chmin(&mut self, x: Self) -> bool;
    fn chmax(&mut self, x: Self) -> bool;
}
impl<T: PartialOrd> ChangeMinMax for T {
    fn chmin(&mut self, x: Self) -> bool {
        *self > x && {
            *self = x;
            true
        }
    }
    fn chmax(&mut self, x: Self) -> bool {
        *self < x && {
            *self = x;
            true
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    N: usize,
    board: Vec<Vec<i64>>,
    zobrist_hash: Vec<Vec<u128>>, // (i,j)を結んだかどうか
}
impl Input {
    fn new() -> Self {
        input! {
            N: usize,
            board: [[i64; N]; N],
        }
        let mut zobrist_hash = vec![vec![0; N]; N];
        let mut rng = Pcg64Mcg::new(SEED);
        for i in 0..N {
            for j in 0.._N {
                zobrist_hash[i][j] = rng.gen();
            }
        }
        Self {
            N,
            board,
            zobrist_hash,
        }
    }
}


#[derive(Clone, Debug)]
struct State {
    board: Vec<Vec<i64>>,
    visited: Vec<Vec<bool>>,
    hash: u128,
}
impl State {
    fn new(input: &Input) -> Self {
        Self {
            board: input.board.clone(),
            visited: vec![vec![false; input.N]; input.N],
            hash: 0,
        }
    }
    fn enum_cands(&self, input: &Input) -> Vec<Operation> {
        let mut cands = vec![];
        todo!("合法手を列挙する処理");
        cands
    }
    #[inline]
    fn operate(&mut self, input: &Input, op: Operation) {
        todo!("状態とハッシュを更新する処理");
    }
    #[inline]
    fn revert(&mut self, input: &Input, op: Operation) {
        todo!("状態とハッシュを戻す処理");
    }
    #[inline]
    fn delta_score(&self, op: Operation -> i64 {
        todo!("スコア差分計算");
    }
}


#[derive(Clone, Debug)]
struct Action {
    order: Vec<Operation>,
    score: i64,
}
impl Action {
    fn new() -> Self {
        Self {
            order: vec![],
            score: 0,
        }
    }
    fn yield_next(&self, op: Operation, delta_score: i64) -> Self {
        let mut next_order = self.order.clone();
        next_order.push(op);
        Self {
            order: next_order,
            score: self.score + delta_score,
        }
    }
}
impl Ord for Action {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}
impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for Action {}
impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

fn beam_search(input: &Input, mut state: &mut State) -> Action {
    let mut crt_beam = BinaryHeap::new();
    crt_beam.push(Action::new());
    let mut hash_set = FxHashSet::default();
    for t in 0..BEAM_DEPTH {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..BEAM_WIDTH {
            if crt_beam.is_empty() {
                break;
            }
            let crt_action = crt_beam.peek().unwrap();
            for &op in &crt_action.order {
                state.operate(input, op);
            }
            hash_set.insert(state.hash);
            let operations = state.enum_cands(&input);
            for op in operations {
                state.operate(input, op);
                if hash_set.contains(&state.hash) {
                    state.revert(input, op);
                    continue;
                }
                hash_set.insert(state.hash);
                let delta_score = state.delta_score(op);
                let next_action = crt_action.yield_next(op, delta_score);
                next_beam.push(next_action);
                state.revert(input, op);
            }
        }

        assert!(!next_beam.is_empty());

        crt_beam = next_beam;
        hash_set.clear();
    }
    
    crt_beam.peek().unwrap().clone()
}


fn main() {
    get_time();
    let input = Input::new();
    let mut state = State::new(&input);
    let best_action = beam_search(&input, &mut state);
    
    eprintln!("time: {}", get_time());
}
