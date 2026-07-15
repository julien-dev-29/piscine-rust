pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (
        c,
        (c as f64).exp(),
        (c as f64).abs().ln(),
    )
}

pub fn str_function(a: String) -> (String, String) {
    let exp_string = a
        .split_whitespace()
        .map(|s| {
            let n: f64 = s.parse().unwrap();
            n.exp().to_string()
        })
        .collect::<Vec<_>>()
        .join(" ");

    (a, exp_string)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let logs = b
        .iter()
        .map(|&x| (x as f64).abs().ln())
        .collect();

    (b, logs)
}

#[cfg(test)]
mod tests {
    use super::*;


}
