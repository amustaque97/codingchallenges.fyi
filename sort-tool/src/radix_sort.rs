pub fn sort(mut list: Vec<String>) -> Vec<String> {
    let max_len = list
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();
    let mut arr: Vec<Vec<String>> = vec![Vec::with_capacity(max_len); 126];

    for mut i in 0..max_len {
        for w in &list {
            if i >= w.len() {
                i = 0;
            }
            let char = w.as_str().chars().nth(i).unwrap() as usize;
            arr[char].push(w.to_string());
        }
        let mut temp = Vec::new();
        for x in 0..=125 {
            temp.extend(arr[x].drain(..));
        }
        list = temp;
    }
    list
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_sort() {
        let mut arr: Vec<String> = vec![
            "Hii".to_string(),
            "This is a very long string".to_string(),
            "Hi".to_string(),
            "world".to_string(),
            "Hello".to_string(),
        ];

        let sorted_arr = sort(arr);
        assert_eq!(
            sorted_arr,
            vec![
                "Hi".to_string(),
                "Hii".to_string(),
                "Hello".to_string(),
                "This is a very long string".to_string(),
                "world".to_string()
            ]
        );
    }

    #[test]
    fn basic_case() {
        let mut arr: Vec<String> = vec![
            "banana".to_string(),
            "apple".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "grape".to_string(),
        ];

        let sorted_arr = sort(arr);
        assert_eq!(
            sorted_arr,
            vec![
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
                "date".to_string(),
                "grape".to_string()
            ]
        );
    }

    #[test]
    fn repeat_case() {
        let mut arr: Vec<String> = vec![
            "banana".to_string(),
            "apple".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "grape".to_string(),
            "apple".to_string(),
        ];

        let sorted_arr = sort(arr);
        assert_eq!(
            sorted_arr,
            vec![
                "apple".to_string(),
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
                "date".to_string(),
                "grape".to_string()
            ]
        );
    }

    #[test]
    fn already_sorted_case() {
        let mut arr: Vec<String> = vec![
            "apple".to_string(),
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "grape".to_string(),
        ];

        let sorted_arr = sort(arr);
        assert_eq!(
            sorted_arr,
            vec![
                "apple".to_string(),
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
                "date".to_string(),
                "grape".to_string()
            ]
        );
    }

    #[test]
    fn upper_and_lower_case() {
        let mut arr: Vec<String> = vec![
            "Apple".to_string(),
            "banana".to_string(),
            "Cherry".to_string(),
            "date".to_string(),
            "Grape".to_string(),
        ];

        let sorted_arr = sort(arr);
        assert_eq!(
            sorted_arr,
            vec![
                "Apple".to_string(),
                "Cherry".to_string(),
                "Grape".to_string(),
                "banana".to_string(),
                "date".to_string(),
            ]
        );
    }
}
