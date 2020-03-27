fn main() {
    let dishes = vec![
        vec![
            String::from("Salad"),
            String::from("Tomato"),
            String::from("Cucumber"),
            String::from("Salad"),
            String::from("Sauce"),
        ],
        vec![
            String::from("Pizza"),
            String::from("Tomato"),
            String::from("Sausage"),
            String::from("Sauce"),
            String::from("Dough"),
        ],
        vec![
            String::from("Quesadilla"),
            String::from("Chicken"),
            String::from("Cheese"),
            String::from("Sauce"),
        ],
        vec![
            String::from("Sandwich"),
            String::from("Salad"),
            String::from("Bread"),
            String::from("Tomato"),
            String::from("Cheese"),
        ],
    ];
    let ingredients = grouping_dishes(dishes);
    ingredients
        .iter()
        .for_each(|ingredient| println!("{:?}", ingredient));
}

// Input:
//
// dishes:
// [["Salad","Tomato","Cucumber","Salad","Sauce"],
//  ["Pizza","Tomato","Sausage","Sauce","Dough"],
//  ["Quesadilla","Chicken","Cheese","Sauce"],
//  ["Sandwich","Salad","Bread","Tomato","Cheese"]]
//
//
// Expected Output:
//
// [["Cheese","Quesadilla","Sandwich"],
//  ["Salad","Salad","Sandwich"],
//  ["Sauce","Pizza","Quesadilla","Salad"],
//  ["Tomato","Pizza","Salad","Sandwich"]]

fn grouping_dishes(dishes: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut ingredients: Vec<(String, Vec<String>)> = Vec::new();

    for dish in dishes {
        for i in dish.iter().skip(1) {
            match ingredients.iter_mut().find(|ing| ing.0 == *i) {
                Some(ingredient) => {
                    ingredient.1.push(dish[0].clone());
                }
                None => {
                    ingredients.push((i.clone(), vec![dish[0].clone()]));
                }
            }
        }
    }

    let mut result = ingredients
        .drain(0..ingredients.len())
        .filter(|ingredient| ingredient.1.len() >= 2)
        .map(|mut ingredient| {
            ingredient.1.sort();
            ingredient.1.insert(0, ingredient.0);
            ingredient.1
        })
        .collect::<Vec<Vec<String>>>();
    result.sort_by_key(|ingredient| ingredient[0].clone());
    result
}
