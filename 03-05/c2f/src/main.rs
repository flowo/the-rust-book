fn main() {
    let celsius = 0.;
    let fahrenheit = c2f::convert_to_fahrenheit(celsius);
    println!("Water freezes at {fahrenheit} °F / {celsius} °C");

    let fahrenheit = 97.7;
    let celsius = c2f::convert_to_celsius(fahrenheit);
    println!("Humans are healthy at {celsius} °C / {fahrenheit} °F");

    let celsius = 39.1;
    let fahrenheit = c2f::convert_to_fahrenheit(celsius);
    println!("Humans are sick at {fahrenheit} °F / {celsius} °C");

    let fahrenheit = 212.;
    let celsius = c2f::convert_to_celsius(fahrenheit);
    println!("Water boils at {celsius} °C / {fahrenheit} °F");
}
