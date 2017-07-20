use std::collections::HashMap;

struct Converter {
    val: HashMap<String, f64>
}

impl Converter {
    fn init() -> Converter {
        Converter { 
            val: HashMap::new()
        }
    }

    fn add_new<U: Into<String>, T: Into<f64>>(&mut self, fname: U, fvalue: T) {
        self.val.insert(fname.into(), fvalue.into());
    }

    fn convert<U: Into<String>, T: Into<f64>>(&self, fvalue: T, fname: U, tname: U) -> Option<f64> {
        let p1 = match self.val.get(&fname.into()) {
            Some(value) => value,
            None => return None
        };
        let p2 = match self.val.get(&tname.into()) {
            Some(value) => value,
            None => return None
        };
        Some(fvalue.into() * p1 / p2)
    }
}

fn main() {
    let mut converter = Converter::init();
    converter.add_new("rub", 1);
    converter.add_new("eur", 68);
    converter.add_new("usd", 59);
    converter.add_new("btc", 155763);
    let from_v = 391.30882352941177;
    let r = converter.convert(from_v, "eur", "usd").unwrap();
    println!("{} eur -> {} usd", from_v, r);
    let from_v = 110882.13559322034;
    let r = converter.convert(from_v, "usd", "btc").unwrap();
    println!("{} usd -> {} btc", from_v, r);
}