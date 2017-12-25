use volatile::Volatile;

pub fn delay(t : u32) {
    let mut t = Volatile::new(t);
    while t.read() > 0 {
        t.update(|t| *t -= 1);
    }
}
