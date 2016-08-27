/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut non_prime:Vec<bool> = vec![false; n as usize];
    if n <= 2 {
        return vec![2];
    }
    for i in 2..n {
        if !non_prime[i as usize] {
            let mut multiplier = i;
            while (i * multiplier) < n {
                non_prime[(i * multiplier) as usize] = true;
                multiplier += 1;
            }
            if multiplier == i {
                break;
            }
        }
    }

    let mut result:Vec<u32> = Vec::new();
    for i in 2..n {
        if !non_prime[i as usize] {
            result.push(i);
        }
    }
    result
}
