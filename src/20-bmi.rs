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

    let bmi = mass / height.powi(2);
    println!("Индекс массы тела равен {}", bmi);
    let status = if bmi > 0.0 && bmi < 16.0 {
        "Выраженный дефицит массы тела"
    } else if bmi >= 16.0 && bmi < 18.5 {
        "Недостаточная (дефицит) масса тела"
    } else if bmi >= 18.5 && bmi <= 24.99 {
        "Норма"
    } else if bmi >= 25.0 && bmi < 30.0 {
        "Избыточная масса тела (предожирение)"
    } else if bmi >= 30.0 && bmi < 35.0 {
        "Ожирение первой степени"
    } else if bmi >= 35.0 && bmi < 40.0 {
        "Ожирение второй степени"
    } else {
        "Ожирение третьей степени (морбидное)"
    };
    println!("Статус: {}", status);
}
