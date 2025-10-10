use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

/// Правило переписывания: заменяет `pattern` на `replacement`
#[derive(Clone)]
struct RewriteRule {
    pattern: &'static str,
    replacement: &'static str,
}

/// Основной набор правил
static SRS_PRIMARY: &[RewriteRule] = &[
    RewriteRule { pattern: "aa", replacement: "" },
    RewriteRule { pattern: "bb", replacement: "cccc" },
    RewriteRule { pattern: "cc", replacement: "acb" },
    RewriteRule { pattern: "abc", replacement: "aabbcc" },
    RewriteRule { pattern: "baabaac", replacement: "cbba" },
];

static SRS_EXTENDED: &[RewriteRule] = &[
    RewriteRule { pattern: "aa", replacement: "" },
    RewriteRule { pattern: "bb", replacement: "cccc" },
    RewriteRule { pattern: "cc", replacement: "acb" },
    RewriteRule { pattern: "abc", replacement: "bbcc" },
    RewriteRule { pattern: "baabaac", replacement: "cbba" },
    RewriteRule { pattern: "cacb", replacement: "acbc" },
    RewriteRule { pattern: "cbacbacb", replacement: "bc" },
    RewriteRule { pattern: "acbcbacbc", replacement: "bacbacb" },
    RewriteRule { pattern: "cbacbca", replacement: "cbacbc" },
    RewriteRule { pattern: "cbcbacbc", replacement: "abacbacb" },
    RewriteRule { pattern: "bacbc", replacement: "acbcb" },
    RewriteRule { pattern: "acbcabacbacb", replacement: "cabacbacb" },
];

static ALPHABET: &[char] = &['a', 'b', 'c'];

const MAX_REPLACEMENTS: usize = 5;
const WORD_LENGTH: usize = 9;

fn generate_word() -> String {
    let mut rng = thread_rng();
    (0..WORD_LENGTH)
        .map(|_| *ALPHABET.choose(&mut rng).unwrap())
        .collect()
}

fn match_positions(text: &str, pattern: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    let bytes = text.as_bytes();
    let pat_bytes = pattern.as_bytes();

    if pat_bytes.len() > bytes.len() {
        return matches;
    }

    for i in 0..=bytes.len() - pat_bytes.len() {
        if &bytes[i..i + pat_bytes.len()] == pat_bytes {
            matches.push(i);
        }
    }

    matches
}

fn rewrite_randomly(mut input: String, rules: &[RewriteRule]) -> String {
    let mut rng = thread_rng();

    for _ in 0..MAX_REPLACEMENTS {
        let mut candidates = Vec::new();
        for rule in rules {
            let positions = match_positions(&input, rule.pattern);
            if !positions.is_empty() {
                candidates.push((rule, positions));
            }
        }

        if candidates.is_empty() {
            continue;
        }

        let (rule, positions) = candidates.choose(&mut rng).unwrap();
        let pos = *positions.choose(&mut rng).unwrap();

        let mut updated = String::new();
        updated.push_str(&input[..pos]);
        updated.push_str(rule.replacement);
        updated.push_str(&input[pos + rule.pattern.len()..]);
        input = updated;
    }

    input
}

fn generate_variations(start: &str, rules: &[RewriteRule], max_depth: usize) -> HashSet<String> {
    let mut seen = HashSet::new();
    let mut queue = vec![start.to_string()];
    seen.insert(start.to_string());

    for _ in 0..max_depth {
        if queue.is_empty() {
            break;
        }

        let mut next_level = Vec::new();
        for word in queue {
            for new_word in apply_rules(&word, rules) {
                if seen.insert(new_word.clone()) {
                    next_level.push(new_word);
                }
            }
        }
        queue = next_level;
    }

    seen
}

fn apply_rules(word: &str, rules: &[RewriteRule]) -> HashSet<String> {
    let mut results = HashSet::new();
    let bytes = word.as_bytes();

    for rule in rules {
        let pat_bytes = rule.pattern.as_bytes();
        if pat_bytes.len() > bytes.len() {
            continue;
        }

        for i in 0..=bytes.len() - pat_bytes.len() {
            if &bytes[i..i + pat_bytes.len()] == pat_bytes {
                let mut new_word = String::new();
                new_word.push_str(&word[..i]);
                new_word.push_str(rule.replacement);
                new_word.push_str(&word[i + rule.pattern.len()..]);
                results.insert(new_word);
            }
        }
    }

    results
}

fn have_common_variation(start: &str, rules: &[RewriteRule], known_set: &HashSet<String>) -> bool {
    if known_set.contains(start) {
        println!("Common word found: {}", start);
        return true;
    }

    let mut seen = HashSet::new();
    let mut queue = vec![start.to_string()];
    seen.insert(start.to_string());

    while !queue.is_empty() {
        let mut next = Vec::new();
        for current in queue {
            for new_word in apply_rules(&current, rules) {
                if known_set.contains(&new_word) {
                    println!("Common word found: {}", new_word);
                    return true;
                }
                if seen.insert(new_word.clone()) {
                    next.push(new_word);
                }
            }
        }
        queue = next;
    }

    false
}

fn main() {
    let initial = generate_word();
    println!("Generated word: {}", initial);

    let rewritten = rewrite_randomly(initial.clone(), SRS_PRIMARY);
    println!("After random rewrites: {}", rewritten);

    let generated_set = generate_variations(&initial, SRS_EXTENDED, 20);
    let found_common = have_common_variation(&rewritten, SRS_EXTENDED, &generated_set);

    if found_common {
        println!("OK");
    } else {
        println!("Not OK");
    }
}
