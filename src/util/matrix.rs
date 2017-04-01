

pub trait Matrix<T> {

    fn rows(&self) -> usize;

    fn cols(&self) -> usize;

    fn set(&mut self, row: usize, col: usize, t: T);

    fn get(&mut self, row: usize, col: usize) -> &T;
}



pub struct VectorMatrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T> VectorMatrix<T> {

    pub fn new(rows: usize, cols: usize) -> Self {
        VectorMatrix {
            rows: rows,
            cols: cols,
            data: Vec::with_capacity(rows * cols)
        }
    }
}

impl<T : Clone> VectorMatrix<T> {

    pub fn new_with_default(rows: usize, cols: usize, val: T) -> Self {
        let v : Vec<T> = (0 .. rows*cols).map( |i| val.clone() ).collect();

        VectorMatrix {
            rows: rows,
            cols: cols,
            data: v
        }
    }
}

impl<T> Matrix<T> for VectorMatrix<T> {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn set(&mut self, row: usize, col: usize, t: T) {
        assert!( row < self.rows() );
        assert!( col < self.cols() );
        let idx = row * self.cols() + col;
        self.data[idx] = t;
    }
    fn get(&mut self, row: usize, col: usize) -> &T {
        assert!( row < self.rows() ); 
        assert!( col < self.cols() );
        let idx = row * self.cols() + col;
        &self.data[idx]
    }
}


