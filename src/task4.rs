#[test]

/*
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    pub fn new()

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}
*/


fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Успех!");
}
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. Реализуем ассоциированную функцию `new`
    // 2. Она вернёт TrafficLight с цветом "red"
    // 3. Используем `Self`, не используем `TrafficLight` в сигнатурах функций или теле
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}
/*
В функции new мы создаём и возвращаем новый экземпляр TrafficLight с цветом "red".
Используем Self для обозначения структуры, что позволяет избежать использования её имени.
Метод get_state - Этот метод возвращает ссылку на цвет, хранящийся в структуре.
*/

