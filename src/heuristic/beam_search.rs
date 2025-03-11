#![allow(non_snake_case, unused)]

use itertools::Itertools;
use proconio::{marker::*, source::line::LineSource, *};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashSet;
use std::io::*;
use std::{cmp::*, vec};
use std::{collections::*, fmt::format};
use superslice::Ext;

const INF: i64 = 1 << 60;
const SEED: u128 = 8_192;
const TL: f64 = 1.989;
const BEAM_DEPTH: usize = 1000;
const BEAM_WIDTH: usize = 1000;

type Op = usize;

#[derive(Clone, Debug)]
struct Entry<K, V> {
    k: K,
    v: V,
}

impl<K: PartialOrd, V> Ord for Entry<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<K: PartialOrd, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.k.partial_cmp(&other.k)
    }
}

impl<K: PartialEq, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.k.eq(&other.k)
    }
}

impl<K: PartialEq, V> Eq for Entry<K, V> {}

/// K が小さいトップn個を保持
#[derive(Clone, Debug)]
pub struct BoundedSortedList<K: PartialOrd + Copy, V: Clone> {
    que: BinaryHeap<Entry<K, V>>,
    size: usize,
}

impl<K: PartialOrd + Copy, V: Clone> BoundedSortedList<K, V> {
    pub fn new(size: usize) -> Self {
        Self {
            que: BinaryHeap::with_capacity(size),
            size,
        }
    }
    pub fn can_insert(&self, k: K) -> bool {
        self.que.len() < self.size || self.que.peek().unwrap().k > k
    }
    pub fn insert(&mut self, k: K, v: V) {
        if self.que.len() < self.size {
            self.que.push(Entry { k, v });
        } else if let Some(mut top) = self.que.peek_mut() {
            if top.k > k {
                top.k = k;
                top.v = v;
            }
        }
    }
    pub fn list(&self) -> Vec<(K, V)> {
        let v = self.que.clone().into_sorted_vec();
        v.into_iter().map(|e| (e.k, e.v)).collect()
    }
    pub fn len(&self) -> usize {
        self.que.len()
    }
}

#[derive(Clone, Debug)]
struct Cand {
    op: Op,
    from: usize, // どの状態に対する操作か
}

#[derive(Clone, Debug)]
struct State {}
impl State {
    fn new() -> Self {
        todo!()
    }
    fn is_done(&self) -> bool {
        todo!()
    }
    fn enum_cands(
        &self,
        input: &Input,
        mut cands: &mut BoundedSortedList<usize, Cand>,
        from: usize,
    ) {
        todo!()
    }
}

fn beam_search() -> Vec<Op> {
    let state = State::new();
    let mut crt_beam = vec![state];
    let mut prev = vec![]; // 経路復元
    for t in 0..BEAM_DEPTH {
        let mut cands = BoundedSortedList::new(BEAM_WIDTH);
        for b in 0..crt_beam.len() {
            let crt_state = &crt_beam[b];
            crt_state.enum_cands(input, &mut cands, b);
        }

        let mut next_beam = vec![];
        let mut hash_set = FxHashSet::default();
        for (score, cand) in cands.list() {
            // 盤面・ハッシュの更新をここに書く

            if !hash_set.insert(hash) {
                continue;
            }

            prev.push((cand.op, crt_beam[cand.from].id));

            let next_state = State {
                C: crt_C,
                score,
                hash,
                id: prev.len() - 1,
            };
            next_beam.push(next_state);
        }
        crt_beam = next_beam;

        if crt_beam.iter().any(|state| state.is_done()) {
            break;
        }
    }

    // 経路復元
    let mut out = vec![];
    let mut best_id = !0;
    let mut best_score = INF;
    for state in crt_beam.iter() {
        if state.is_done() && best_score.chmin(state.score) {
            best_id = state.id;
        }
    }
    let mut id = best_id;
    while id != !0 {
        let (op, next_id) = prev[id];
        out.push(op);
        id = next_id;
    }
    out.reverse();

    out
}
