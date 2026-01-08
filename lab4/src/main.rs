use std::time::Instant;

fn gen_segments(max_len: usize) -> Vec<String> {
    let mut segs: Vec<String> = Vec::new();
    if max_len == 0 {
        return segs;
    }

    let mut x = String::from("a");
    let mut y = String::from("a");

    while y.len() <= max_len {
        segs.push(y.clone());

        // x = x + "a" + x + "b"
        let mut next_x = String::with_capacity(x.len() * 2 + 2);
        next_x.push_str(&x);
        next_x.push('a');
        next_x.push_str(&x);
        next_x.push('b');

        // y = y + x
        let mut next_y = String::with_capacity(y.len() + next_x.len());
        next_y.push_str(&y);
        next_y.push_str(&next_x);

        x = next_x;
        y = next_y;
    }

    segs
}

fn go_from(pos: usize, x: &str, segs: &[String]) -> bool {
    if pos == x.len() {
        return true;
    }

    for s in segs {
        let seg_len = s.len();
        if pos + seg_len <= x.len() && &x[pos..pos + seg_len] == s.as_str() {
            if go_from(pos + seg_len, x, segs) {
                return true;
            }
        }
    }
    false
}

fn can_split_into_segments(x: &str, segs: &[String]) -> bool {
    go_from(0, x, segs)
}

fn simple_matches(s: &str) -> bool {
    let l = s.len();
    if l < 3 || (l - 3) % 2 != 0 {
        return false;
    }
    let n = (l - 3) / 2;

    if &s[n..n + 3] != "bab" {
        return false;
    }

    let left = &s[..n];
    let right = &s[n + 3..];

    if left != right {
        return false;
    }

    if left.is_empty() {
        return true;
    }

    let segs = gen_segments(left.len());
    can_split_into_segments(left, &segs)
}

fn parse_right_to_left(x: &str, segs: &[String]) -> bool {
    let mut pos = x.len();

    while pos > 0 {
        let mut amount_b: usize = 0;
        let bytes = x.as_bytes();

        let mut i: isize = pos as isize - 1;
        while i >= 0 && bytes[i as usize] == b'b' {
            amount_b += 1;
            i -= 1;
        }

        if amount_b >= segs.len() {
            return false;
        }

        let seg = &segs[amount_b];
        let seg_len = seg.len();
        if seg_len > pos {
            return false;
        }

        let start = pos - seg_len;
        if &x[start..pos] != seg.as_str() {
            return false;
        }

        pos = start;
    }

    true
}

fn optimized_matches(s: &str) -> bool {
    let l = s.len();
    if l < 3 || (l - 3) % 2 != 0 {
        return false;
    }
    let n = (l - 3) / 2;

    if &s[n..n + 3] != "bab" {
        return false;
    }

    let left = &s[..n];
    let right = &s[n + 3..];

    if left != right {
        return false;
    }

    if left.is_empty() {
        return true;
    }

    let segs = gen_segments(left.len());
    parse_right_to_left(left, &segs)
}

fn gen_all_strings(n: usize, cur: &mut String, out: &mut Vec<String>) {
    if cur.len() == n {
        out.push(cur.clone());
        return;
    }

    cur.push('a');
    gen_all_strings(n, cur, out);
    cur.pop();

    cur.push('b');
    gen_all_strings(n, cur, out);
    cur.pop();
}

fn main() {
    const MAX_LEN: usize = 27;
    const MIN_LEN: usize = 13;

    let mut slow_in: i128 = 0;
    let mut fast_in: i128 = 0;
    let mut slow_out: i128 = 0;
    let mut fast_out: i128 = 0;

    let mut cnt_in: i128 = 0;
    let mut cnt_out: i128 = 0;

    for l in MIN_LEN..=MAX_LEN {
        let mut all: Vec<String> = Vec::new();
        let mut cur = String::with_capacity(l);
        gen_all_strings(l, &mut cur, &mut all);

        for s in &all {
            let t0 = Instant::now();
            let slow = simple_matches(s);
            let slow_dur = t0.elapsed().as_nanos() as i128;

            let t1 = Instant::now();
            let fast = optimized_matches(s);
            let fast_dur = t1.elapsed().as_nanos() as i128;

            if slow != fast {
                println!("MISMATCH: {} slow={} fast={}", s, slow, fast);
                return;
            }

            if slow {
                cnt_in += 1;
                slow_in += slow_dur;
                fast_in += fast_dur;
            } else {
                cnt_out += 1;
                slow_out += slow_dur;
                fast_out += fast_dur;
            }
        }
    }

    let in_speedup = (slow_in as f64) / (fast_in as f64);
    let out_speedup = (slow_out as f64) / (fast_out as f64);

    println!(
        "IN   : cnt={}, slow={} ns, fast={} ns, speedup={:.2}x",
        cnt_in, slow_in, fast_in, in_speedup
    );
    println!(
        "OUT  : cnt={}, slow={} ns, fast={} ns, speedup={:.2}x",
        cnt_out, slow_out, fast_out, out_speedup
    );
}
