use rand::seq::SliceRandom;

pub fn get_fruit_list() -> Vec<String> {
    vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "orange".to_string(),
        "fig".to_string(),
        "pomegranate".to_string(),
        "pear".to_string(),
        "kiwi".to_string(),
        "mango".to_string(),
        "plum".to_string(),
    ]
}

pub fn create_fruit_salad(fruit_number: usize, fruits: Vec<String>) -> Result<Vec<String>, String> {
    if fruits.is_empty() {
        if fruit_number == 0 {
            return Err("You need to specify the number of fruits or the fruits to include in the salad".to_string());
        } else if fruit_number > get_fruit_list().len() {
            return Err("You can't create a salad with more fruits than the available ones".to_string());
        } else {
            return Ok(create_fruit_salad_by_number_of_fruits(fruit_number));
        }
    } else {
        if !fruits_in_list(fruits.clone()) {
            return Err("You can't create a salad with fruits that are not in the list".to_string());
        } else {
            return Ok(create_fruit_salad_by_selected_fruits(fruits));
        }
    }
}

fn fruits_in_list(fruits: Vec<String>) -> bool {
    let fruit_list = get_fruit_list();
    fruits.iter().all(|f| fruit_list.contains(&f.to_string()))
}

fn create_fruit_salad_by_number_of_fruits(fruit_number: usize) -> Vec<String> {
    let mut fruit = get_fruit_list();

    let mut rng = rand::thread_rng();
    fruit.as_mut_slice().shuffle(&mut rng);

    let salad: Vec<String> = fruit.iter().take(fruit_number).cloned().collect();

    salad
}

fn create_fruit_salad_by_selected_fruits(fruits: Vec<String>) -> Vec<String> {
    let salad: Vec<String> = fruits.iter().map(|f| f.to_string()).collect();

    salad
}
