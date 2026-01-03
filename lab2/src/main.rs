use rand::Rng;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::process;

struct DFA {
    start: usize,
    next: Vec<[usize; 2]>, // next[state][0] = 'a', next[state][1] = 'b'
    accepting: Vec<bool>,
}

impl DFA {
    fn build_min_dfa() -> Self {
        let mut next = vec![[0usize; 2]; 45];
        let mut accepting = vec![false; 45];

        let mut set = |s: usize, a_next: usize, b_next: usize| {
            next[s][0] = a_next;
            next[s][1] = b_next;
        };

        accepting[35] = true;
        accepting[36] = true;
        accepting[37] = true;
        accepting[38] = true;
        accepting[39] = true;
        accepting[40] = true;
        accepting[41] = true;
        accepting[42] = true;
        accepting[43] = true;
        accepting[44] = true;

        set(0, 0, 0);
        set(1, 2, 3);
        set(2, 5, 6);
        set(3, 7, 4);
        set(4, 0, 11);
        set(5, 0, 10);
        set(6, 1, 11);
        set(7, 0, 12);
        set(8, 15, 30);
        set(9, 16, 32);
        set(10, 0, 31);
        set(11, 0, 30);
        set(12, 2, 32);
        set(13, 15, 28);
        set(14, 15, 29);
        set(15, 0, 23);
        set(16, 5, 26);
        set(17, 23, 0);
        set(18, 24, 11);
        set(19, 23, 11);
        set(20, 15, 34);
        set(21, 23, 30);
        set(22, 25, 32);
        set(23, 0, 35);
        set(24, 2, 37);
        set(25, 5, 38);
        set(26, 1, 41);
        set(27, 15, 43);
        set(28, 35, 31);
        set(29, 36, 32);
        set(30, 35, 0);
        set(31, 36, 3);
        set(32, 39, 4);
        set(33, 44, 0);
        set(34, 35, 35);
        set(35, 8, 17);
        set(36, 13, 18);
        set(37, 14, 19);
        set(38, 9, 21);
        set(39, 8, 22);
        set(40, 20, 33);
        set(41, 8, 33);
        set(42, 27, 17);
        set(43, 40, 17);
        set(44, 8, 42);

        DFA {
            start: 1,
            next,
            accepting,
        }
    }

    fn run(&self, word: &str) -> (isize, bool) {
        let mut state = self.start;

        for ch in word.bytes() {
            state = match ch {
                b'a' => self.next[state][0],
                b'b' => self.next[state][1],
                _ => return (-1, false),
            };
        }

        (state as isize, self.accepting[state])
    }
}

struct NFA {
    start: usize,
    accepting: HashSet<usize>,
    trans_a: Vec<Vec<usize>>,
    trans_b: Vec<Vec<usize>>,
}

impl NFA {
    fn build_nfa() -> Self {
        let mut trans_a = vec![Vec::<usize>::new(); 18];
        let mut trans_b = vec![Vec::<usize>::new(); 18];

        let add = |m: &mut Vec<Vec<usize>>, from: usize, to: usize| {
            m[from].push(to);
        };

        add(&mut trans_a, 0, 0);
        add(&mut trans_b, 0, 0);
        add(&mut trans_a, 1, 2);
        add(&mut trans_b, 1, 9);
        add(&mut trans_a, 2, 3);
        add(&mut trans_b, 2, 6);
        add(&mut trans_b, 3, 4);
        add(&mut trans_a, 3, 0);
        add(&mut trans_b, 4, 5);
        add(&mut trans_b, 4, 1);
        add(&mut trans_a, 4, 0);
        add(&mut trans_a, 5, 8);
        add(&mut trans_b, 5, 0);
        add(&mut trans_b, 6, 7);
        add(&mut trans_a, 6, 1);
        add(&mut trans_b, 7, 5);
        add(&mut trans_a, 7, 0);
        add(&mut trans_a, 9, 10);
        add(&mut trans_b, 9, 11);
        add(&mut trans_b, 10, 7);
        add(&mut trans_b, 10, 1);
        add(&mut trans_a, 10, 0);
        add(&mut trans_b, 11, 7);
        add(&mut trans_a, 11, 0);
        add(&mut trans_a, 8, 12);
        add(&mut trans_b, 8, 16);
        add(&mut trans_a, 12, 13);
        add(&mut trans_b, 12, 15);
        add(&mut trans_b, 13, 14);
        add(&mut trans_a, 13, 0);
        add(&mut trans_b, 14, 8);
        add(&mut trans_a, 14, 0);
        add(&mut trans_a, 15, 8);
        add(&mut trans_b, 15, 0);
        add(&mut trans_a, 16, 17);
        add(&mut trans_b, 16, 0);
        add(&mut trans_b, 17, 8);
        add(&mut trans_a, 17, 0);

        let accepting: HashSet<usize> = [8].into_iter().collect();

        NFA {
            start: 1,
            accepting,
            trans_a,
            trans_b,
        }
    }

    fn run(&self, word: &str) -> bool {
        let mut cur: HashSet<usize> = [self.start].into_iter().collect();

        let step = |cur: &HashSet<usize>, ch: u8, trans_a: &Vec<Vec<usize>>, trans_b: &Vec<Vec<usize>>| {
            let mut next = HashSet::new();
            for &s in cur {
                let outs = match ch {
                    b'a' => &trans_a[s],
                    b'b' => &trans_b[s],
                    _ => &[] as &[usize],
                };
                for &t in outs {
                    next.insert(t);
                }
            }
            next
        };

        for ch in word.bytes() {
            cur = step(&cur, ch, &self.trans_a, &self.trans_b);
            if cur.is_empty() {
                return false;
            }
        }

        cur.iter().any(|s| self.accepting.contains(s))
    }
}
struct UniversalNFA {
    trans: Vec<HashMap<char, Vec<usize>>>,
    accepting: HashSet<usize>,
    start_states: Vec<usize>,
}

impl UniversalNFA {
    fn build() -> Self {
        let max_state = 26usize;
        let mut trans: Vec<HashMap<char, Vec<usize>>> = vec![HashMap::new(); max_state + 1];

        let mut add = |from: usize, c: char, to: usize| {
            trans[from].entry(c).or_insert_with(Vec::new).push(to);
        };

        add(16, 'a', 16);
        add(16, 'b', 16);
        add(16, 'b', 17);
        add(17, 'b', 18);
        add(18, 'a', 19);
        add(19, 'a', 20);
        add(20, 'a', 24);
        add(24, 'b', 25);
        add(25, 'b', 19);
        add(19, 'b', 21);
        add(21, 'a', 25);
        add(20, 'b', 26);
        add(26, 'a', 19);

        add(12, 'b', 10);
        add(9,  'b', 10);
        add(9,  'a', 10);
        add(11, 'a', 12);
        add(15, 'b', 11);
        add(10, 'a', 11);
        add(1,  'b', 4);
        add(2,  'a', 5);
        add(0,  'b', 2);
        add(13, 'a', 15);
        add(8,  'b', 10);
        add(8,  'a', 10);
        add(8,  'b', 0);
        add(14, 'b', 15);
        add(4,  'b', 9);
        add(6,  'b', 9);
        add(4,  'a', 0);
        add(2,  'b', 6);
        add(3,  'b', 8);
        add(11, 'b', 13);
        add(12, 'a', 14);
        add(1,  'a', 3);
        add(0,  'a', 1);
        add(5,  'b', 0);
        add(5,  'b', 9);

        let accepting: HashSet<usize> = [11usize, 19usize].into_iter().collect();

        UniversalNFA {
            trans,
            accepting,
            start_states: vec![16, 0], // E-переходы из "&"
        }
    }

    fn run_branch(&self, start: usize, word: &str) -> bool {
        let mut cur: HashSet<usize> = [start].into_iter().collect();

        for ch in word.chars() {
            let mut next = HashSet::new();
            for &s in &cur {
                if let Some(outs) = self.trans[s].get(&ch) {
                    for &t in outs {
                        next.insert(t);
                    }
                }
            }
            if next.is_empty() {
                return false;
            }
            cur = next;
        }

        cur.iter().any(|s| self.accepting.contains(s))
    }

    /// слово принимается, только если каждая ветка его принимает.
    fn run(&self, word: &str) -> bool {
        self.start_states
            .iter()
            .all(|&s| self.run_branch(s, word))
    }
}

fn rand_word_fixed_len<R: Rng>(rng: &mut R, len: usize) -> String {
    let mut bytes = vec![0u8; len];
    for b in &mut bytes {
        let bit = rng.gen_bool(0.5);
        *b = if bit { b'a' } else { b'b' };
    }
    String::from_utf8(bytes).unwrap()
}


fn main() {
    let n = 500;
    let mut rng = rand::thread_rng();

    let re = Regex::new(r"^(aba|bab|aabb)*(a|b)(a|b)bba(aba|bab|aabb)*$")
        .expect("не удалось скомпилировать регулярку");

    let dfa = DFA::build_min_dfa();
    let nfa = NFA::build_nfa();
    let univ_nfa = UniversalNFA::build();

    for length in 1..30 {
        for i in 1..=n {
            let word = rand_word_fixed_len(&mut rng, length);

            let (_, dfa_ok) = dfa.run(&word);
            let nfa_ok = nfa.run(&word);
            let re_ok = re.is_match(&word);
            let univ_ok = univ_nfa.run(&word);

            if !(dfa_ok == nfa_ok && nfa_ok == re_ok && re_ok == univ_ok) {
                println!("Расхождение на длине {}, тест #{}", length, i);
                println!("Слово: {:?}", word);
                println!("DFA       : {}", dfa_ok);
                println!("NFA       : {}", nfa_ok);
                println!("Regex     : {}", re_ok);
                println!("Universal : {}", univ_ok);
                process::exit(1);
            }
        }
        println!(
            "OK: {} слов длины {} проверено, расхождений не найдено",
            n, length
        );
    }
}