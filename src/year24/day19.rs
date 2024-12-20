use itertools::Itertools;
use bnum::BUint;

type UIBIG = BUint<1>;
const ALPHABET_SIZE: usize = 5;
const MAX_TOWEL_LEN: usize = 500;

const NOT_FOUND: usize = 0;

#[derive(Clone, Debug)]
struct TrieEntry {
    nbs: [usize; ALPHABET_SIZE + 1],
    terminal: bool,
}

impl TrieEntry {
    fn new() -> TrieEntry {
        TrieEntry {
            nbs: [NOT_FOUND; ALPHABET_SIZE + 1],
            terminal: false,
        }
    }
}

#[derive(Debug)]
struct CountTrie {
    entries: Vec<TrieEntry>,
    counts: [UIBIG; MAX_TOWEL_LEN + 1],
}

impl CountTrie {
    fn new() -> CountTrie {
        CountTrie {
            entries: vec![TrieEntry::new(); 2],
            counts: [UIBIG::ZERO; MAX_TOWEL_LEN + 1],
        }
    }

    fn get_or_create_nb(&mut self, node_id: usize, letter: u8) -> usize {
        let mut res = self.entries[node_id].nbs[letter as usize];
        if res == NOT_FOUND {
            res = self.entries.len();
            self.entries.push(TrieEntry::new());
            self.entries[node_id].nbs[letter as usize] = res;
        };
        res
    }

    fn insert<T: Iterator<Item=u8>>(&mut self, word: T) {
        let mut node_id = 1;
        word.for_each(|x| {
            node_id = self.get_or_create_nb(node_id, x);
        });
        self.entries[node_id].terminal = true;
    }

    fn count_paths(&mut self, word: &Vec<u8>) -> UIBIG {
        self.counts.fill(UIBIG::ZERO);
        self.counts[0] = UIBIG::ONE;
        for i in 0..word.len() {
            let c = self.counts[i % MAX_TOWEL_LEN];
            self.counts[i % MAX_TOWEL_LEN] = UIBIG::ZERO;
            let mut curr_ind = 1;
            for j in i..word.len() {
                curr_ind = self.entries[curr_ind].nbs[word[j] as usize];
                if curr_ind == NOT_FOUND {
                    break;
                };

                if self.entries[curr_ind].terminal {
                    self.counts[(j + 1) % MAX_TOWEL_LEN] += c;
                }
            }
        }
        self.counts[word.len() % MAX_TOWEL_LEN]
    }
}

fn encode_num(c: char) -> u8 {
    match c {
        'w' => 1,
        'u' => 2,
        'b' => 3,
        'r' => 4,
        'g' => 5,
        _ => unreachable!(),
    }
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let mut trie = CountTrie::new();
    data.lines().next().unwrap().split(",").for_each(|x| {
        let word = x.trim().chars().map(encode_num);
        trie.insert(word);
    });

    let tasks = data
        .lines()
        .skip(2)
        .map(|x| x.trim().chars().map(encode_num).collect_vec());

    let mut p1 = 0;
    let mut p2 = UIBIG::ZERO;
    tasks.for_each(|x| {
        let cnt = trie.count_paths(&x);
        if cnt > UIBIG::ZERO {
            p1 += 1;
            p2 += cnt;
        }
    });

    (p1.to_string(), p2.to_string())
}
