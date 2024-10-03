#[test]

/*

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {

}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}
*/




fn main() {
    let c = TrafficLightColor::Yellow;

    // Проверяем, что цвет правильный
    assert_eq!(c.color(), "yellow");

    // Выводим отладочную информацию
    println!("{:?}", c);
}
#[derive(Debug)]
#[allow(dead_code)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Реализация метода для TrafficLightColor
impl TrafficLightColor {
    pub fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}

/*
Метод color: Этот метод использует match для сопоставления значения перечисления и возвращает соответствующее строковое представление цвета.
assert_eq!: Проверяет, что метод color возвращает правильное значение для состояния светофора.
println!("{:?}", c);: Выводит отладочную информацию о перечислении, используя #[derive(Debug)].
*/