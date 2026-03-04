pub fn get_hint(n: usize, charset: impl AsRef<str>) -> String {
    let charset = charset.as_ref();
    let base = charset.len();

    if n == 0 {
        return charset.chars().next().unwrap().to_string();
    }

    let mut result = String::new();
    let mut curr = n;

    while curr > 0 {
        let rem = curr % base;
        result.push(charset.chars().nth(rem).unwrap());
        curr /= base;
    }

    result.chars().rev().collect()
}
