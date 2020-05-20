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
}

pub struct NameDrawing {
    name_key: HashMap<i32, Vec<String>>,
    buys_for: HashMap<String, String>,
}

impl NameDrawing {
    pub fn new(names: Vec<Vec<String>>) -> NameDrawing {
        // Construct hashmap
        let mut name_key = HashMap::new();
        for (i, v) in names.iter().enumerate() {
            name_key.insert(i as i32, v.to_vec());
        }
        NameDrawing {
            name_key: name_key,
            buys_for: HashMap::new(),
        }
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
