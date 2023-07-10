use proconio::{marker::*, source::line::LineSource, *};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashSet;
use std::cmp::Reverse;

const N: usize = 30;
const BEAM_WIDTH: usize = 1_000;
const BEAM_DEPTH: usize = 1_000;
const SEED: u128 = 8_192;
const INF: i64 = 1_000_000_000;
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
    zobrist_hash: Vec<Vec<u128>>,
}
impl Input {
    fn new() -> Self {
        input! {
            board: [i64; N],
        }

        let mut rng = Pcg64Mcg::new(SEED);
        let mut zobrist_hash = vec![0; N];
        for i in 0..N {
            zobrist_hash[i] = rng.gen();
        }

        Self {
            board,
            zobrist_hash,
        }
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

    // Stateへの操作
    #[inline]
    fn apply(&mut self, op: Operation) {
        todo!();
    }

    // Stateへの逆操作
    #[inline]
    fn revert(&mut self, op: Operation) {
        todo!();
    }

    // 評価関数
    #[inline]
    fn score(&self) -> i64 {
        todo!();
    }

    // Stateのhash値
    // inputの情報からzobrist hashを前計算しておき，状態更新とともにhash値も更新する
    #[inline]
    fn hash(&self) -> u128 {
        todo!();
    }
}

#[derive(Clone)]
struct Cand {
    op: Operation,
    parent: usize,
    turn: usize,
    score: usize,
    hash: u128,
}
impl Cand {
    fn to_node(&self) -> Node {
        Node::new(self.op, self.parent, self.turn, self.score, self.hash)
    }
}

struct Node {
    op: Operation,
    parent: usize,
    child: usize,
    prev: usize,
    next: usize,
    turn: usize,
    score: usize,
    hash: u128,
}
impl Node {
    fn new(op: Operation, parent: usize, turn: usize, score: usize, hash: u128) -> Self {
        Self {
            op,
            parent,
            child: !0,
            prev: !0,
            next: !0,
            turn,
            score,
            hash,
        }
    }
}

// 現在のノードをビーム幅の分保持する構造体
struct Beam {
    state: State,
    latest: usize,
    nodes: Vec<Node>, // nodes[latest..]が最新のノード
    cur_node: usize,
}
impl Beam {
    fn new(state: State, node: Node) -> Self {
        let mut nodes = vec![];
        nodes.push(node);

        Self {
            state,
            latest: 0,
            nodes,
            cur_node: 0,
        }
    }

    #[inline]
    fn add_node(&mut self, cand: &Cand) {
        let next = self.nodes[cand.parent].child;
        if next != !0 {
            self.nodes[next].prev = self.nodes.len();
        }
        self.nodes[cand.parent].child = self.nodes.len();

        self.nodes.push(Node {
            next,
            ..cand.to_node()
        });
    }

    #[inline]
    fn del_node(&mut self, mut idx: usize) {
        loop {
            let Node {
                prev, next, parent, ..
            } = self.nodes[idx];
            assert_ne!(parent, !0);
            if prev & next == !0 {
                idx = parent;
                continue;
            }

            if prev != !0 {
                self.nodes[prev].next = next;
            } else {
                self.nodes[parent].child = next;
            }
            if next != !0 {
                self.nodes[next].prev = prev;
            }

            break;
        }
    }

    fn restore(&self, mut idx: usize) -> Vec<Operation> {
        let mut ret = vec![];

        loop {
            // 親が存在しないところ(最初のダミーノード)まで遡って手番を再現する
            let Node { op, parent, .. } = self.nodes[idx];
            if parent == !0 {
                break;
            }
            ret.push(op);
            idx = parent;
        }

        ret.reverse();
        ret
    }

    #[inline]
    fn update<'a, I: Iterator<Item = &'a Cand>>(&mut self, cands: I) {
        let len = self.nodes.len();
        for cand in cands {
            self.add_node(cand);
        }

        for i in self.latest..len {
            if self.nodes[i].child == !0 {
                self.del_node(i);
            }
        }
        self.latest = len;
    }

    #[inline]
    fn dfs(&mut self, input: &Input, cands: &mut Vec<Cand>, single: bool) {
        if self.nodes[self.cur_node].child == !0 {
            // 子がいなければ，1手進めて作る
            self.append_cands(input, self.cur_node, cands);
            return;
        }

        let node = self.cur_node;
        let mut child = self.nodes[node].child;
        // ある深さまで1本道ならそこより上に戻るのは無駄なので検知する
        let next_single = single & (self.nodes[child].next == !0);

        loop {
            self.cur_node = child;
            let node = &self.nodes[child];
            self.state.apply(node.op);
            self.dfs(input, cands, next_single);

            // 1本道でなければロールバックが必要
            if !next_single {
                let node = &self.nodes[child];
                self.state.revert(node.op);
            }
            child = self.nodes[child].next;
            if child == !0 {
                break;
            }
        }

        if !next_single {
            self.cur_node = node;
        }
    }

    #[inline]
    fn enum_cands(&mut self, input: &Input, cands: &mut Vec<Cand>) {
        self.dfs(input, cands, true);
    }

    #[inline]
    fn append_cands(&mut self, input: &Input, idx: usize, cands: &mut Vec<Cand>) {
        // 現在のノードから1手進めたノードを，候補に格納する
        let node = &self.nodes[idx];
        let crt_score = node.score;
        // 各合法手について
        for o in 0..NUM_OP {
            // 操作を行い，次状態を確認する
            let next_turn = node.turn + 1;
            self.state.apply(&input, OP[o]);
            // スコア差分を更新する
            let score_diff = self.state.score();
            // ロールバック
            self.state.revert(&input, OP[o]);
            // hashの更新
            // 例えば，位置の変更に対して下記のように更新する
            let crt_pos = 0;
            let next_pos = 1;
            let hash = node.hash ^ input.zobrist_hash[crt_pos] ^ input.zobrist_hash[next_pos];

            // この操作を候補に保存
            let cand = Cand {
                op: OP[o],
                parent: idx,
                turn: next_turn,
                score: crt_score + score_diff,
                hash,
            };
            cands.push(cand);
        }
    }
}

fn beam(input: &Input) -> Vec<Operation> {
    let first_op = 'X'; // 最初のノードに入れる，ダミーの操作
    let mut beam = {
        let state = State::new(input);
        let node = Node::new(first_op, !0, 0, state.score(input), state.hash(input));
        Beam::new(state, node)
    };
    let mut cands: Vec<Cand> = vec![];
    let mut first = true;

    let mut set = FxHashSet::default();
    let best;

    let mut iter = 0;

    loop {
        // ゲームが終了しているものの中で，なるべくスコアが高いものを選ぶ
        if let Some(idx) = (0..cands.len()).find(|&i| cands[i].turn == BEAM_DEPTH) {
            best = cands[idx].clone();
            break;
        }
        iter += 1;

        if !first {
            // ソートのkeyの昇順･降順に注意
            cands.sort_unstable_by_key(|a| Reverse(a.score));
            set.clear();

            // hashが被ってない場合のみ残し，上位ビーム幅個残す
            let it = cands
                .iter()
                .filter(|cand| set.insert(cand.hash))
                .take(BEAM_WIDTH);
            beam.update(it);
        }
        first = false;

        cands.clear();
        beam.enum_cands(input, &mut cands);
        assert_ne!(cands.len(), 0);
    }

    eprintln!("iter: {}", iter);
    eprintln!("beam_score: {}", best.score);
    let mut ret = beam.restore(best.parent);
    ret.push(best.op);

    ret
}

fn main() {
    get_time();
    let input = Input::new();
    let ans = beam(&input);

    for &ansi in &ans {
        println!("{}", ansi);
    }

    eprintln!("time: {}", get_time());
}