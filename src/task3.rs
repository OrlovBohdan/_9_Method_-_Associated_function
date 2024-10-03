#[test]

/*
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(__)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(__) {
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}
*/




fn main() {
    let mut light = TrafficLight {
        color: "red".to_string(),
    };

    light.show_state(); // Отображает текущее состояние
    light.change_state(); // Меняет состояние на зеленый
    light.show_state(); // Отображает обновленное состояние
    println!("Успех!");
}
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Используем `&self` для обращения к экземпляру.
    pub fn show_state(&self) {
        println!("Текущая световая сигнализация: {}", self.color);
    }

    // Используем `&mut self`, чтобы разрешить изменение.
    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}
/*
&self используется в методах для заимствования экземпляра неизменяемо, что позволяет читать его поля.
&mut self используется, когда нужно изменить экземпляр, что позволяет вносить изменения в его поля.
*/
