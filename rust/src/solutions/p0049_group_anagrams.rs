use std::collections::HashMap;

pub struct MySolution;

// 超出时间限制了
impl MySolution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        let mut added_indexes: Vec<usize> = vec![];

        for (i, str1) in strs.iter().enumerate() {
            if added_indexes.contains(&i) {
                continue;
            }
            let mut item: Vec<String> = vec![str1.to_string()];
            added_indexes.push(i);

            for (j, str2) in strs.iter().enumerate() {
                if added_indexes.contains(&j) || i == j {
                    continue;
                }
                if Self::is_same_letter(str1, str2) {
                    item.push(str2.to_string());
                    added_indexes.push(j);
                }
            }

            res.push(item);
        }

        res
    }

    fn is_same_letter(str1: &str, str2: &str) -> bool {
        if str1.len() != str2.len() {
            return false;
        }

        let str1_count_letter_map = Self::get_count_letter_map(&str1);
        let str2_count_letter_map = Self::get_count_letter_map(&str2);

        if str1_count_letter_map.len() != str2_count_letter_map.len() {
            return false;
        }

        for key in str1_count_letter_map.keys() {
            if str1_count_letter_map.get(key) != str2_count_letter_map.get(key) {
                return false;
            }
        }
        
        true
    }

    fn get_count_letter_map(str: &str) -> HashMap::<char, i32> {
        let mut res = HashMap::<char, i32>::new();
        
        for (_, char) in str.chars().enumerate() {
            if let Some(value) = res.get(&char) {
                res.insert(char, value + 1);
            } else {
                res.insert(char, 0);
            }
        }

        res
    }
}

pub struct SolutionSort;

impl SolutionSort {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::<String, Vec<String>>::new();
        
        for (_, str) in strs.iter().enumerate() {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort();
            let sorted_string: String = chars.into_iter().collect();
            if let Some(vec) = map.get_mut(&sorted_string)  {
                vec.push(str.clone());
            } else {
                map.insert(sorted_string, vec![str.clone()]);
            }
        }

        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {

    }
}