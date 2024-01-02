use std::usize;

#[derive(Clone, Copy, Default)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub struct PolyLine(Vec<Point>);

impl PolyLine {
    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn add(&mut self, point: Point) {
        self.0.push(point);
    }

    pub fn new() -> Self {
        PolyLine(vec![])
    }

    pub fn pop(&mut self) -> Option<Point> {
        self.0.pop()
    }

    pub fn get(&self, index: usize) -> Option<Point> {
        self.0.get(index).copied()
    }

    pub fn remove(&mut self, index: usize) -> Point {
        self.0.remove(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn add_increases_size() {
        let mut poly = PolyLine::new();
        poly.add(Default::default());
        assert_eq!(poly.size(), 1);
    }
}
