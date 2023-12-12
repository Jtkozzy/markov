//This proram is my Rust version of Kernighan & Pike's book "The Practice of Programming"'s
//Chapter 3 Markov program, look https://www.cs.princeton.edu/~bwk/tpop.webpage/code.html
//(C) by Jari Korhonen <jtkorhonen@gmail.com>, 2023.
//This program is free software, use as you wish, I don't care
//You need Rust 1.7.2 or higher due to my using the awesome String leak() API
//For suitable input material, the above website has "King James Bible" and "Book Of Psalms" files
//Program reads input from stdin and outputs to stdout
//This should be fast enough, neck and neck with C version on "Psalms", much faster on "King James"

use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io;

const NWORDS: usize = 10000; //Maximum words generated
const NPREFIX: usize = 2; //2 prefix words used
const SHORT_DOC_ERR: &str = "Expected longer document";
const STDIN_READ_ERROR: &str = "Error reading from stdin";
const APPROX_NUM_PREFIXES: usize = 20_000; //About this many in 'Psalms' file
const MULTIPLIER: u32 = 31;

//Hashmap key is structure Prefix, which has 2 str references, very much like C version
#[derive(Copy, Clone, PartialEq, Eq)]
struct Prefix {
    a: &'static str,
    b: &'static str,
}

impl Hash for Prefix {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut h: u32 = 0;

        for p in self.a.bytes() {
            h = MULTIPLIER * h + p as u32;
        }
        for p in self.b.bytes() {
            h = MULTIPLIER * h + p as u32;
        }
        state.write_u32(h)
    }
}

//Hashmap value is a list of all possible words after 2 key's words
type WordList = Vec<&'static str>;

fn main() {
    //Read everything from stdin and then leak data to get &'static reference
    let data = io::read_to_string(io::stdin()).expect(STDIN_READ_ERROR);
    let leaked_data: &'static str = data.leak();

    //State is the central data structure, which has wordlist vector for every
    //Prefix combination
    let mut state: HashMap<Prefix, WordList> = HashMap::with_capacity(APPROX_NUM_PREFIXES);

    //Build
    //Set initial prefix..
    let mut iter = leaked_data.split_whitespace();
    let first_prefix: Prefix;
    first_prefix = Prefix {
        a: iter.next().expect(SHORT_DOC_ERR),
        b: iter.next().expect(SHORT_DOC_ERR),
    };
    //..and build
    let mut prefix = first_prefix;
    for word in iter {
        state.entry(prefix).or_insert(Vec::new()).push(word);
        prefix.a = prefix.b;
        prefix.b = word;
    }

    //Generate
    //Cache output first to string to avoid blocking on stdout mutex
    const APPROX_OUTPUT_SIZE: usize = 50_000;
    let mut s: String = String::with_capacity(APPROX_OUTPUT_SIZE);
    prefix = first_prefix;
    let mut w: &str;
    let mut rng = thread_rng();

    //start with original prefix...
    s.push_str(&format!("{}\n{}\n", prefix.a, prefix.b));
    //...and then output rest of words
    for _ in 0..NWORDS - NPREFIX {
        if let Some(suf) = state.get(&prefix) {
            let ind = rng.gen::<usize>() % suf.len();
            w = suf[ind];
            s.push_str(w);
            s.push('\n');
            prefix.a = prefix.b;
            prefix.b = w;
        } else {
            break;
        }
    }
    //output cached string to stdout
    print!("{s}");
}
