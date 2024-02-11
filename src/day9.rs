fn main() {
    let input = std::fs::read_to_string("data/day9.txt").unwrap();
    let mut lines = input.lines();

    let mut last_sum = 0;
    let mut first_sum = 0;
    for line in lines {
        let base = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut all_layers = Vec::new();
        let mut current_layer = base;
        all_layers.push(current_layer.clone());
        while current_layer.iter().any(|x| *x != 0) {
            let mut next_layer = Vec::new();
            for i in 0..current_layer.len() - 1 {
                next_layer.push(current_layer[i + 1] - current_layer[i]);
            }
            all_layers.push(next_layer.clone());
            current_layer = next_layer;
        }
        let mut last_value = 0;
        let mut first_value = 0;
        for layer in all_layers.iter().rev() {
            last_value = layer.last().unwrap() + last_value;
            first_value = layer.first().unwrap() - first_value;
        }
        last_sum += last_value;
        first_sum += first_value;
    }
    println!("sum: {}", last_sum);
    println!("sum: {}", first_sum);
}