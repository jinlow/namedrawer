use rand::seq::IteratorRandom;
use rand::Rng;
use std::collections::HashMap;

#[cfg(test)]
mod test_namedrawing {
    use super::*;
    #[test]
    fn test_single_key_pass() {
        let mut mmap = HashMap::new();
        mmap.insert(1, String::from("something"));
        mmap.insert(1, String::from("else"));
        assert!(all_keys_idx(1, &mmap));
    }
    #[test]
    fn test_single_key_fail() {
        let mut mmap = HashMap::new();
        mmap.insert(1, String::from("something"));
        mmap.insert(2, String::from("else"));
        assert!(!all_keys_idx(1, &mmap));
    }
    #[test]
    fn adding_new_item() {
        let mut nmd = NameDrawing::new();
        nmd.add_names(vec![String::from("What"), String::from("that")]);
        nmd.add_names(vec![String::from("them")]);
        assert_eq!(nmd.name_key.len(), 2);
    }
}

pub struct NameDrawing {
    name_key: HashMap<i32, Vec<String>>,
    buys_for: HashMap<String, String>,
}

impl NameDrawing {
    pub fn new() -> NameDrawing {
        NameDrawing {
            name_key: HashMap::new(),
            buys_for: HashMap::new(),
        }
    }
    pub fn from_vec(names: Vec<Vec<String>>) -> NameDrawing {
        // Construct hashmap
        let mut namedrawing = NameDrawing::new();
        for n in names {
            namedrawing.add_names(n);
        }
        namedrawing
    }

    pub fn add_names(&mut self, name_vec: Vec<String>) {
        let idx = self.name_key.len() as i32;
        self.name_key.insert(idx, name_vec);
    }

    fn full_name_list(&self) -> Vec<(i32, String)> {
        let mut full_name: Vec<(i32, String)> = Vec::new();
        for (key, value) in &self.name_key {
            for name in value {
                full_name.push((*key, name.to_string()));
            }
        }
        full_name
    }

    pub fn draw_names(&mut self) {
        self.buys_for.clear();
        let mut nkc = self.name_key.clone();
        for (idx, name) in self.full_name_list() {
            let mut rkey = rnd_key(&nkc);
            // There are times we can hit a corner case,
            // where we end up that the only remaining
            // indexes to choose from is the current names
            // partner (or the name you don't want them to
            // buy for). In such instances we can use
            // recursion to start over, untill we reach
            // an acceptable solution. Not sure if there is
            // a more performant approach to this, but in such
            // a small example, it doesn't matter?
            if all_keys_idx(idx, &nkc) {
                self.draw_names();
                return;
            }
            while rkey == idx {
                rkey = rnd_key(&nkc);
            }
            let name_vec = nkc[&rkey].to_vec();
            let draw_name = name_vec[rnd_idx(&name_vec)].to_string();
            nkc.get_mut(&rkey)
                .unwrap()
                .retain(|x| x != &draw_name.to_string());
            self.buys_for.insert(name, draw_name);
            if nkc[&rkey].len() == 0 {
                nkc.remove(&rkey);
            }
        }
        self.reveal_drawing();
    }
    pub fn reveal_drawing(&self) {
        for (key, value) in &self.buys_for {
            println!("{} buys for {}.", key, value)
        }
    }
}

fn all_keys_idx<T>(idx: i32, mmp: &HashMap<i32, T>) -> bool {
    // Are the only keys, the index present?
    let idx_present: Vec<i32> = mmp.keys().map(|x| (x == &idx) as i32).collect();
    let idx_p_count = idx_present.iter().sum::<i32>();
    idx_p_count == (mmp.len() as i32)
}

fn rnd_idx<T>(vec: &Vec<T>) -> usize {
    rand::thread_rng().gen_range(0, vec.len())
}

fn rnd_key<T1: Copy, T2>(nkey: &HashMap<T1, T2>) -> T1 {
    let mut rng = rand::thread_rng();
    *nkey.keys().choose(&mut rng).unwrap()
}
