pub struct Rng {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Rng {
        Rng { x: seed, y: 362_436_069, z: 521_288_629, w: 88_675_123 }
    }
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        self.w
    }
}
