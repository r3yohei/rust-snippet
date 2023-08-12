use cargo_snippet::snippet;

/// 最小･最大の交換
/// A.chmax(B)のようにすることで，もしA<BならA=Bとしてtrueを返し，そうでなければAのまま保持してfalseを返す
#[snippet("r3yohei_ChangeMinMax")]
pub trait ChangeMinMax {
    fn chmin(&mut self, x: Self) -> bool;
    fn chmax(&mut self, x: Self) -> bool;
}
#[snippet("r3yohei_ChangeMinMax")]
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
