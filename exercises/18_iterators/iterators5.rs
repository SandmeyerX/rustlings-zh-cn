// 让我们定义一个简单的模型来追踪 Rustlings 的练习进度。
// 进度将使用哈希表进行建模：练习的名称作为键，进度作为值。
// 我们创建了两个计数函数，用来统计具有给定进度的练习数量。
// 请使用迭代器重新实现这套计数功能，尽量不使用命令式循环（`for`/`while`）。

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

// TODO: 使用迭代器而不是 `for` 循环来实现 `count_for` 的功能。
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` 是一个哈希表，其键为 `String` 类型，值为 `Progress` 类型。
    // map = { "variables1": Complete, "conversions3": None, … }
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

// TODO: 使用迭代器而不是 `for` 循环来实现 `count_collection_for` 的功能。
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // `collection` 是哈希表的切片。
    // collection = [{ "variables1": Complete, "conversions3": None, … },
    //               { "variables2": Complete, … }, … ]
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmaps1"), Complete);
        map.insert(String::from("smart_pointers3"), Some);
        map.insert(String::from("conversions5"), None);
        map.insert(String::from("conversions3"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("conversions2"), None);
        other.insert(String::from("conversions4"), None);

        vec![map, other]
    }

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Complete), 3);
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Some), 1);
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::None), 2);
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state),
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_iterator(&collection, Progress::Complete),
            6,
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::Some), 1);
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::None), 4);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state),
            );
        }
    }
}
