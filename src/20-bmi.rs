use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Введите массу тела, кг");
    io::stdin().read_line(&mut buffer).unwrap();
    let mass = match buffer.trim().parse::<f32>() {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };
    buffer.clear();
    println!("Введите рост, м");
    io::stdin().read_line(&mut buffer).unwrap();
    let height = match buffer.trim().parse::<f32>() {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };
    let BMI = mass / height.powi(2);
    println!("Индекс массы тела равен {}", BMI);
    let status = if BMI > 0.0 && BMI < 16.0 {
        "Выраженный дефицит массы тела".to_owned()
    } else if BMI >= 16.0 && BMI < 18.5 {
        "Недостаточная (дефицит) масса тела".to_owned()
    } else if BMI >= 18.5 && BMI <= 24.99 {
        "Норма".to_owned()
    } else if BMI >= 25.0 && BMI < 30.0 {
        "Избыточная масса тела (предожирение)".to_owned()
    } else if BMI >= 30.0 && BMI < 35.0 {
        "Ожирение первой степени".to_owned()
    } else if BMI >= 35.0 && BMI < 40.0 {
        "Ожирение второй степени".to_owned()
    } else {
        "Ожирение третьей степени (морбидное)".to_owned()
    };
    println!("Статус: {}", status);
}