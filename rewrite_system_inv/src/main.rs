use rand::seq::SliceRandom;

#[derive(Clone)]
struct RewriteRule {
    pattern: &'static str,
    replacement: &'static str,
}

static SRS_EXTENDED: &[RewriteRule] = &[
    RewriteRule { pattern: "aa", replacement: "" },
    RewriteRule { pattern: "bc", replacement: "bb" },
    RewriteRule { pattern: "abb", replacement: "bb" },
    RewriteRule { pattern: "acb", replacement: "cc" },
    RewriteRule { pattern: "acc", replacement: "cb" },
    RewriteRule { pattern: "bba", replacement: "bb" },
    RewriteRule { pattern: "cbb", replacement: "bbb" },
    RewriteRule { pattern: "ccb", replacement: "bbb" },
    RewriteRule { pattern: "ccc", replacement: "bbb" },
    RewriteRule { pattern: "bbbb", replacement: "bb" },
];

const ALPHABET: &[char] = &['a', 'b', 'c'];
const MAX_STEPS: usize = 20;
const WORD_LENGTH: usize = 9;


fn random_word() -> String {
    let mut rng = rand::thread_rng();
    (0..WORD_LENGTH)
        .map(|_| *ALPHABET.choose(&mut rng).unwrap())
        .collect()
}

fn find_positions(word: &str, pattern: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    if pattern.len() > word.len() {
        return positions;
    }
    for i in 0..=word.len() - pattern.len() {
        if &word[i..i + pattern.len()] == pattern {
            positions.push(i);
        }
    }
    positions
}

fn apply_random_rule(word: &str, rules: &[RewriteRule]) -> Option<String> {
    let mut rng = rand::thread_rng();
    let mut applicable = Vec::new();

    for rule in rules {
        let positions = find_positions(word, rule.pattern);
        for &pos in &positions {
            applicable.push((rule, pos));
        }
    }

    if applicable.is_empty() {
        return None;
    }

    let (rule, pos) = applicable.choose(&mut rng).unwrap();
    let mut new_word = String::new();
    new_word.push_str(&word[..*pos]);
    new_word.push_str(rule.replacement);
    new_word.push_str(&word[*pos + rule.pattern.len()..]);

    Some(new_word)
}

fn count_letters(s: &str) -> (usize, usize, usize) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for ch in s.chars() {
        match ch {
            'a' => a += 1,
            'b' => b += 1,
            'c' => c += 1,
            _ => {}
        }
    }
    (a, b, c)
}

fn parity(x: usize) -> usize {
    x % 2
}

fn only_as(s: &str) -> bool {
    s.chars().all(|ch| ch == 'a')
}

fn check_invariants(chain: &[String]) -> bool {
    let mut ok = true;

    let (_, b0, c0) = count_letters(&chain[0]);
    let start_parity = parity(b0 + c0);
    let start_has_bc = b0 > 0 || c0 > 0;

    for i in 1..chain.len() {
        let (_, b, c) = count_letters(&chain[i]);

        // 1. Четность (b + c) сохраняется
        if parity(b + c) != start_parity {
            println!("Нарушена четность на шаге {}: {}", i, chain[i]);
            ok = false;
        }

        // 2. Если в начале были b или c, не должно быть слова только из 'a' или пустого
        if start_has_bc && (only_as(&chain[i]) || chain[i].is_empty()) {
            println!("Получено только 'a' на шаге {}: {}", i, chain[i]);
            ok = false;
        }
    }

    ok
}

fn main() {
    let start = random_word();
    println!("Начальное слово: {}", start);

    let mut chain = vec![start.clone()];
    let mut current = start.clone();

    for _ in 0..MAX_STEPS {
        if let Some(next) = apply_random_rule(&current, SRS_EXTENDED) {
            chain.push(next.clone());
            current = next;
        } else {
            break;
        }
    }

    println!("\nЦепочка переписываний:");
    for (i, w) in chain.iter().enumerate() {
        println!("{:2}: {}", i, w);
    }

    println!();
    if check_invariants(&chain) {
        println!("OK");
    } else {
        println!("NOT OK");
    }
}
