#[derive(Debug, Copy, Clone)]
struct Celcius(f32);
#[derive(Debug, Copy, Clone)]
struct Fahrenheit(f32);
#[derive(Debug, Copy, Clone)]
struct Kelvin(f32);

impl From<Celcius> for Fahrenheit {
    fn from(t: Celcius) -> Fahrenheit {
        Fahrenheit(9.0/5.0 * t.0 + 32.0)
    }
}

impl From<Celcius> for Kelvin {
    fn from(t: Celcius) -> Kelvin {
        Kelvin(t.0 + 273.15)
    } 
}

impl From<Kelvin> for Celcius {
    fn from(t: Kelvin) -> Celcius {
        Celcius(t.0 - 273.15)
    }
}

impl From<Fahrenheit> for Celcius {
    fn from(t: Fahrenheit) -> Celcius {
        Celcius(5.0/9.0 * (t.0 - 32.0))
    }
}

impl From<Fahrenheit> for Kelvin {
    fn from(t: Fahrenheit) -> Kelvin {
        Kelvin::from(Celcius::from(t))
    }
}

impl From<Kelvin> for Fahrenheit {
    fn from(t: Kelvin) -> Fahrenheit {
        Fahrenheit::from(Celcius::from(t))
    }
}

fn main() {
    let a = Celcius(36.6);
    // Celcius -> Fahrenheit
    let b = Fahrenheit::from(a);
    // Fahrenheit -> Kelvin
    let c = Kelvin::from(b);
    // Kelvin -> Celcius
    let d = Celcius::from(c);
    println!("{:?} vs {:?} vs {:?} vs {:?}", a, b, c, d);
}