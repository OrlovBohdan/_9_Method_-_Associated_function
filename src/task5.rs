#[test]

/*
struct Rectangle {
    width: u32,
    height: u32,
}

// Using multiple `impl` blocks to rewrite the code below.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    println!("Success!");
}
*/


fn main() {
    println!("Success!");
}

#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Первый блок `impl` для метода `area`
impl Rectangle {
    fn _area(&self) -> u32 {
        self.width * self.height
    }
}

// Второй блок `impl` для метода `can_hold`
impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*
азделение на несколько impl блоков: Методы area и can_hold находятся в отдельных блоках impl.
Это позволяет разбить реализацию методов, что может быть полезно для группировки методов по
категориям или для добавления методов в разные моменты времени.
*/

