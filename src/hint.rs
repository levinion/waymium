use anyhow::{Result, bail};

pub fn get_hint(n: usize, total: usize, charset: impl AsRef<str>) -> Result<String> {
    let charset: Vec<char> = charset.as_ref().chars().collect();
    let base = charset.len();
    if base < 2 && total > base {
        bail!("Charset too small for multiple hints");
    }

    let mut depth = 1;
    let mut capacity = base;
    while capacity < total {
        depth += 1;
        capacity *= base;
    }

    let mut result = Vec::new();
    let mut curr = n;
    for _ in 0..depth {
        result.push(charset[curr % base]);
        curr /= base;
    }

    Ok(result.into_iter().rev().collect())
}
