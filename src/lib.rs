

pub fn square(s: u32) -> u64 {
    let mut i: u64 = 1;
    let mut sum: u64 = 1;
    if s > 0 && s < 65{
        while i < s.into() {
        i += 1;
        sum = sum*2;
    }
    }
    else {
        panic!("Square must be between 1 and 64")
    }
    sum
    //unimplemented!("grains of rice on square {s}");
}

pub fn total() -> u64 {
    let mut i: u64 = 1;
    let mut sum: u64 = 1;
    let mut tot = 1;
    while i < 64 {
        i += 1;
        sum = sum*2;
        tot += sum;
    }
    tot
}
