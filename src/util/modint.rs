use cargo_snippet::snippet;

#[snippet("r3yohei_ModInt")]
#[derive(Debug, Clone, Copy, Default)]
struct ModInt {
    value: usize,
}
#[snippet("r3yohei_ModInt")]
impl ModInt {
    const MOD: usize = 998244353;
    fn new(n: usize) -> Self {
        ModInt { value: n % Self::MOD }
    }
    fn zero() -> Self {
        ModInt { value: 0 }
    }
    fn one() -> Self {
        ModInt { value: 1 }
    }
    fn value(&self) -> usize {
        self.value
    }
    fn pow(&self, n: usize) -> Self {
        let mut p = *self;
        let mut ret = ModInt::one();
        let mut nn = n;
        while nn > 0 {
            if nn & 1 == 1 {
                ret *= p;
            }
            p *= p;
            nn >>= 1;
        }
        ret
    }
    fn inv(&self) -> Self {
        fn ext_gcd(a: usize, b: usize) -> (isize, isize, usize) {
            if a == 0 {
                return (0, 1, b);
            }
            let (x, y, g) = ext_gcd(b % a, a);
            (y - b as isize / a as isize * x, x, g)
        }

        ModInt::new((ext_gcd(self.value, Self::MOD).0 + Self::MOD as isize) as usize)
    }
}
#[snippet("r3yohei_ModInt")]
impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        ModInt::new(self.value + other.value)
    }
}
#[snippet("r3yohei_ModInt")]
impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        ModInt::new(Self::MOD + self.value - other.value)
    }
}
#[snippet("r3yohei_ModInt")]
impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: Self) -> Self {
        ModInt::new(self.value * other.value)
    }
}
#[snippet("r3yohei_ModInt")]
impl std::ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
#[snippet("r3yohei_ModInt")]
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
#[snippet("r3yohei_ModInt")]
impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
#[snippet("r3yohei_ModInt")]
impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
#[snippet("r3yohei_ModInt")]
impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}