use cargo_snippet::snippet;
use std::collections::VecDeque;

#[snippet("r3yohei_removability_checker")]
struct RemovabilityChecker {
    window: usize,
    removability: Vec<bool>,
}
#[snippet("r3yohei_removability_checker")]
impl RemovabilityChecker {
    fn new(window: usize) -> Self {
        let mut removability = vec![false; 1 << (window * window)];
        for pattern in 0..(1 << (window * window)) {
            // 3x3領域を1次元配列に直したとき，ある領域に色があるかないかの2^9通りある
            // それらについて，中央を除いた時に連結かどうかを事前に判定しておく
            let drop_center = pattern & !(1 << ((window * window) / 2));
            removability[pattern] = Self::bfs(window, drop_center as u32);
        }
        Self { window, removability }
    }
    fn bfs(window: usize, pattern: u32) -> bool {
        // patternの後ろの0の数を指定しておけば，1が立っているところをスタート地点にできる
        let s = pattern.trailing_zeros() as usize;
        let mut deque = VecDeque::new();
        let mut visited = 0;
        deque.push_back(s);
        visited |= 1 << s;
        while let Some(crt) = deque.pop_front() {
            if crt % window < window - 1 {
                // 右
                let next = crt + 1;
                if (visited & (1 << next)) == 0 && 0 < (pattern & (1 << next)) {
                    // 未訪問かつ当該パターンで通行可能なら進む
                    visited |= 1 << next;
                    deque.push_back(next);
                }
            }
            if 0 < crt % window {
                // 左
                let next = crt - 1;
                if (visited & (1 << next)) == 0 && 0 < (pattern & (1 << next)) {
                    // 未訪問かつ当該パターンで通行可能なら進む
                    visited |= 1 << next;
                    deque.push_back(next);
                }
            }
            if crt + window < window * window {
                // 下
                let next = crt + window;
                if (visited & (1 << next)) == 0 && 0 < (pattern & (1 << next)) {
                    // 未訪問かつ当該パターンで通行可能なら進む
                    visited |= 1 << next;
                    deque.push_back(next);
                }
            }
            if window <= crt {
                // 上
                let next = crt - window;
                if (visited & (1 << next)) == 0 && 0 < (pattern & (1 << next)) {
                    // 未訪問かつ当該パターンで通行可能なら進む
                    visited |= 1 << next;
                    deque.push_back(next);
                }
            }
        }
        // 全て到達可能かどうか
        pattern == visited
    }
}