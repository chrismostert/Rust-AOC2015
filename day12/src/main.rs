use serde_json::Value;

fn json_sum(json: &Value, stop_on_red: bool) -> i64 {
    match json {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(v) => v.iter().map(|v| json_sum(v, stop_on_red)).sum(),
        Value::Object(m) => {
            let mut sum = 0;
            for val in m.values() {
                if let Value::String(s) = val {
                    if s == "red" && stop_on_red {
                        // Early stop
                        return 0;
                    }
                }
                sum += json_sum(val, stop_on_red);
            }
            sum
        }
        _ => 0,
    }
}

fn main() {
    let input: Value = serde_json::from_str(include_str!("../input.txt")).unwrap();
    println!("Part 1: {}", json_sum(&input, false));
    println!("Part 2: {}", json_sum(&input, true));
}
