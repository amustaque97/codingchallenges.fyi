pub fn merge_sort(arr: &mut Vec<String>) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(&mut left, &mut right, arr);
}

fn merge(left: &mut Vec<String>, right: &mut Vec<String>, arr: &mut Vec<String>) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
pub fn sort(mut list: Vec<String>) -> Vec<String> {
    merge_sort(&mut list);
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
                "Hello".to_string(),
                "Hi".to_string(),
                "Hii".to_string(),
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

