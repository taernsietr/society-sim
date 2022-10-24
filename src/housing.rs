struct Neighborhood {
    lots: Vec<Housing>,
}

struct Housing {
    location: (u16, u16),
    capacity: u8,
}
