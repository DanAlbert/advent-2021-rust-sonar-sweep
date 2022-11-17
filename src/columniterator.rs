pub struct ColumnIterator<T, const W: usize, const H: usize> {
    mat: [[T; W]; H],
    current_column_index: usize,
}

impl<T, const W: usize, const H: usize> ColumnIterator<T, W, H> {
    pub fn new(mat: [[T; W]; H]) -> ColumnIterator<T, W, H> {
        ColumnIterator {
            mat,
            current_column_index: 0,
        }
    }
}

impl<T: Clone, const W: usize, const H: usize> Iterator for ColumnIterator<T, W, H> {
    type Item = [T; H];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column_index >= W {
            return None;
        }

        let mut column = vec![];
        for i in 0..H {
            column.push(self.mat[i][self.current_column_index].clone());
        }
        let result = column.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a column of height {} but it was {}", H, v.len())
        });
        self.current_column_index += 1;
        Some(result)
    }
}

#[test]
fn test() {
    assert_eq!(
        vec![[0, 2, 4], [1, 3, 5]],
        ColumnIterator::new([[0, 1], [2, 3], [4, 5]]).collect::<Vec<[i32; 3]>>()
    );
    assert_eq!(
        vec![[0, 3], [1, 4], [2, 5]],
        ColumnIterator::new([[0, 1, 2], [3, 4, 5]]).collect::<Vec<[i32; 2]>>()
    );
}
