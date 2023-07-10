use cargo_snippet::snippet;

/// 素数判定
/// O(√n)
/// 1~√nまででnを割り切るものがあれば素数でない
#[snippet("r3yohei_is_prime")]
fn is_prime(n: usize) -> bool {
    let rootn = (n as f64).sqrt().floor() as usize;
    for i in 2..=rootn {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// 素因数分解
/// is_prime()に依存
#[snippet("r3yohei_prime_factorization")]
#[snippet(include = "r3yohei_is_prime")]
fn prime_factorization(mut n: usize) -> Vec<usize> {
    let mut factor = vec![];
    if is_prime(n) {
        factor.push(n);
        factor
    } else {
        let rootn = (n as f64).sqrt().floor() as usize;
        for i in 2..=rootn {
            // 例えば2で割れるだけ割り，素因数をfactorへ追加
            // こうすることで，次に4を見たときにwhileに入らない
            while n % i == 0 {
                n /= i;
                factor.push(i);
            }
        }
        // forを抜けて残った数が1以外なら，それも素因数なので追加
        if n != 1 {
            factor.push(n);
        }
        factor
    }
}

/// エラトステネスの篩
/// n以下の素数を列挙する
/// O(nloglogn)
#[snippet("r3yohei_sieve_of_eratosthenes")]
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n+1];
    let mut prime = vec![];
    for i in 2..(n as usize +1) {
        if is_prime[i] {
            prime.push(i);
            // iの倍数を全て消す
            for j in (i*2..n+1).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    prime
}

/// L以上R以下の素数のエラトステネスの篩による列挙
/// is_prime[x]: x+Lが素数かどうか
#[snippet("r3yohei_sieve_of_eratosthenes_from_l_to_r")]
fn sieve_of_eratosthenes_from_l_to_r(n: usize, l: usize, r: usize) -> Vec<usize> {
    let mut is_prime = vec![true; r-l+1];
    if l == 1 {
        is_prime[0] = false;
    }

    let rootn = (n as f64).sqrt().floor() as usize;
    for i in 2..=rootn {
        // L以上で最小のiの倍数
        let min_value = ((l+i-1) / i) * i;
        // L以上R以下のiの倍数をすべて消す
        for j in (min_value..=r).step_by(i) {
            // i自身を直接falseにしない
            if j == i {continue;}
            is_prime[j-l] = false;
        }
    }

    let mut prime = vec![];
    for i in 0..r-l+1 {
        if is_prime[i] {
            prime.push(i+l);
        }
    }
    prime
}