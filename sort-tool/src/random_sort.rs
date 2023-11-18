use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn sort(mut list: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    let mut temp = list.into_boxed_slice();

    temp.shuffle(&mut rng);

    temp.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn random_sort() {
        let mut arr: Vec<String> = vec![
            "Hii".to_string(),
            "This is a very long string".to_string(),
            "Hi".to_string(),
            "world".to_string(),
            "Hello".to_string(),
        ];

        let sorted_arr = sort(arr);
        println!("{:?}", sorted_arr);
    }

}
