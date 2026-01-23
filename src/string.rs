// this string sort is a radix sort
fn sort(strings: &mut [&str]) {
    let max = strings.iter().map(|s| s.len()).max().unwrap_or(0);
    for i in (0..max).rev() {
        let mut b = vec![Vec::new(); 257];
        for s in strings.drain(..) {
            b[s.as_bytes().get(i).map(|&c| c as usize + 1).unwrap_or(0)].push(s);
        }
        for mut v in b { strings.append(&mut v); }
    }
}