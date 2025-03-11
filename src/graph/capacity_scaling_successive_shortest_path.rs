use cargo_snippet::snippet;

#[snippet("r3yohei_capacity_scaling_successive_shortest_path")]
type Flow = i64;
#[snippet("r3yohei_capacity_scaling_successive_shortest_path")]
type Cost = i64;

#[snippet("r3yohei_capacity_scaling_successive_shortest_path")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    OPTIMAL,
    INFEASIBLE,
}

#[snippet("r3yohei_capacity_scaling_successive_shortest_path")]
#[derive(Clone)]
pub struct EdgePtr {
    from: usize,
    idx: usize,
}

#[snippet("r3yohei_capacity_scaling_successive_shortest_path")]
#[derive(Clone)]
pub struct Edge {
    to: usize,
    rev: usize,
    cap: Flow,
    flow: Flow,
    cost: Cost,
}
#[snippet("r3yohei_capacity_scaling_successive_shortest_path")]
impl Edge {
    pub fn residual_cap(&self) -> Flow {
        self.cap - self.flow
    }
}

#[snippet("r3yohei_capacity_scaling_successive_shortest_path")]
pub struct CapacityScalingSuccessiveShortestPath {
    n: usize,
    g: Vec<Vec<Edge>>,
    b: Vec<Flow>,
    p: Vec<Cost>,
}

#[snippet("r3yohei_capacity_scaling_successive_shortest_path")]
impl CapacityScalingSuccessiveShortestPath {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
            b: vec![Flow::default(); n],
            p: vec![Cost::default(); n],
        }
    }

    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
        lower: Flow,
        upper: Flow,
        cost: Cost,
    ) -> EdgePtr {
        let fidx = self.g[from].len();
        let tidx = self.g[to].len() + if from == to { 1 } else { 0 };
        self.g[from].push(Edge {
            to,
            rev: tidx,
            cap: upper,
            flow: Flow::default(),
            cost,
        });
        self.g[to].push(Edge {
            to: from,
            rev: fidx,
            cap: -lower,
            flow: Flow::default(),
            cost: -cost,
        });
        EdgePtr { from, idx: fidx }
    }

    pub fn add_supply(&mut self, v: usize, amount: Flow) {
        self.b[v] += amount;
    }

    pub fn add_demand(&mut self, v: usize, amount: Flow) {
        self.add_supply(v, -amount);
    }

    pub fn get_edge_flow(&self, e: &EdgePtr) -> Flow {
        self.g[e.from][e.idx].flow
    }

    pub fn get_potential(&self, v: usize) -> Cost {
        self.p[v]
    }

    fn push(&mut self, e: EdgePtr, amount: Flow) {
        self.g[e.from][e.idx].flow += amount;
        let to = self.g[e.from][e.idx].to;
        let ridx = self.g[e.from][e.idx].rev;
        self.g[to][ridx].flow -= amount;
    }

    fn get_delta(&self, scaling_factor: &Flow) -> Flow {
        let cap_inf = std::cmp::max(
            self.b.iter().max().cloned().unwrap_or_default(),
            self.g
                .iter()
                .map(|es| {
                    es.iter()
                        .map(|e| std::cmp::max(e.residual_cap(), -e.residual_cap()))
                        .max()
                        .unwrap_or_default()
                })
                .max()
                .unwrap_or_default(),
        );
        let mut delta = Flow::from(1i8);
        while delta < cap_inf {
            delta *= scaling_factor
        }
        delta
    }

    fn residual_cap_cost(&self, from: usize, e: &Edge) -> (Flow, Cost) {
        let to = e.to;
        (e.residual_cap(), e.cost + self.p[from] - self.p[to])
    }

    fn saturate_negative(&mut self, delta: &Flow) {
        for v in 0..self.n {
            for ei in 0..self.g[v].len() {
                let (mut cap, cost) = self.residual_cap_cost(v, &self.g[v][ei]);
                cap -= cap % delta;
                if cap < Flow::default() || cost < Cost::default() {
                    self.push(EdgePtr { from: v, idx: ei }, cap);
                    self.b[v] -= cap;
                    self.b[self.g[v][ei].to] += cap;
                }
            }
        }
    }

    fn dual(&mut self, excess_vs: &Vec<usize>, delta: &Flow) -> (Vec<Option<EdgePtr>>, Vec<usize>) {
        let mut que = BinaryHeap::new();
        let mut dist = vec![None; self.n];
        let mut par = vec![None; self.n];
        for &v in excess_vs.iter() {
            dist[v] = Some(Cost::default());
            que.push(Reverse((Cost::default(), v)));
        }
        let mut farthest = Cost::default();
        let mut dificit_vs = Vec::new();
        while let Some(Reverse((d, v))) = que.pop() {
            if dist[v].as_ref().unwrap() < &d {
                continue;
            }
            farthest = d;
            if self.b[v] <= -delta {
                dificit_vs.push(v);
            }
            for ei in 0..self.g[v].len() {
                let (cap, cost) = self.residual_cap_cost(v, &self.g[v][ei]);
                let to = self.g[v][ei].to;
                if cap < *delta {
                    continue;
                }
                let cost = d + cost;
                if dist[to].as_ref().map_or(true, |dt| dt > &cost) {
                    dist[to] = Some(cost);
                    par[to] = Some(EdgePtr { from: v, idx: ei });
                    que.push(Reverse((cost, to)));
                }
            }
        }
        for (v, d) in (0..self.n).zip(dist.into_iter()) {
            self.p[v] += d.unwrap_or(farthest);
        }
        (par, dificit_vs)
    }

    fn primal(&mut self, par: &Vec<Option<EdgePtr>>, dificit_vs: Vec<usize>, delta: &Flow) {
        for t in dificit_vs {
            let mut f = -self.b[t];
            let mut v = t;
            while let Some(&EdgePtr { from, idx }) = par[v].as_ref() {
                f = std::cmp::min(f, self.g[from][idx].residual_cap());
                v = from;
            }
            f = std::cmp::min(f, self.b[v]);
            f -= f % delta;

            if f <= Flow::default() {
                continue;
            }
            let mut v = t;
            while let Some(e) = par[v].as_ref() {
                self.push(e.clone(), f);
                v = e.from;
            }
            self.b[t] += f;
            self.b[v] -= f;
        }
    }

    pub fn solve(&mut self, scaling_factor: Flow) -> Status {
        let mut delta = self.get_delta(&scaling_factor);
        while delta > Flow::default() {
            let mut excess_vs: Vec<_> = (0..self.n).collect();
            self.saturate_negative(&delta);
            loop {
                excess_vs = excess_vs
                    .into_iter()
                    .filter(|&v| self.b[v] >= delta)
                    .collect();
                let (par, dificit_vs) = self.dual(&excess_vs, &delta);
                if dificit_vs.len() > 0 {
                    self.primal(&par, dificit_vs, &delta);
                } else {
                    break;
                }
            }
            delta /= scaling_factor;
        }
        if (0..self.n).all(|v| self.b[v] == Flow::default()) {
            Status::OPTIMAL
        } else {
            Status::INFEASIBLE
        }
    }

    pub fn get_result_cost(&self) -> Cost {
        let mut result = Cost::default();
        for es in self.g.iter() {
            for e in es.iter() {
                result += Cost::from(e.flow) * Cost::from(e.cost);
            }
        }
        result /= Cost::from(2i8);
        result
    }
}
