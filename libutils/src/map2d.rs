#[derive(Clone)]
pub struct Map2D<T>
where
    T: Copy,
{
    data: Vec<T>,
    default: T,
    size_x: usize,
}

impl<T> Map2D<T>
where
    T: Copy,
{
    pub fn new(default: T, size_x: usize) -> Self {
        Self {
            data: vec![],
            default,
            size_x,
        }
    }

    pub fn directions(&self) -> &'static [(i32, i32)] {
        &[
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 0),
            /*(0, 0),*/ (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ]
    }

    pub fn size_x(&self) -> usize {
        self.size_x
    }
    pub fn size_y(&self) -> usize {
        if self.data.len() == 0 {
            0
        } else {
            ((self.data.len() - 1) / self.size_x) + 1
        }
    }

    pub fn get_value(&self, x: i32, y: i32) -> T {
        if x < 0 || y < 0 {
            return self.default;
        }

        let x = x as usize;
        let y = y as usize;
        return self.get_value_usize(x, y);
    }
    pub fn get_value_usize(&self, x: usize, y: usize) -> T {
        if x >= self.size_x {
            return self.default;
        }

        let idx = x + y * self.size_x;
        if idx >= self.data.len() {
            return self.default;
        }

        self.data[idx]
    }

    pub fn set_value(&mut self, x: i32, y: i32, value: T) {
        if x < 0 || y < 0 {
            panic!("can't set a negative value");
        }
        self.set_value_usize(x as usize, y as usize, value)
    }

    pub fn set_value_usize(&mut self, x: usize, y: usize, value: T) {
        if x > self.size_x {
            panic!("index out of bound for x");
        }

        let idx = x + y * self.size_x;
        while self.data.len() <= idx {
            self.data.push(self.default);
        }

        self.data[idx] = value;
    }
    pub fn add_row(&mut self, row: Vec<T>) {
        if self.size_x != row.len() {
            panic!("Cannot add a row of a different size!");
        }
        let row_index = self.size_y();
        for (i, item) in row.into_iter().enumerate() {
            self.set_value(i as i32, row_index as i32, item);
        }
    }

    pub fn fold<F>(&self, initial: F, fold_func: fn(F, &T, i32, i32) -> F) -> F {
        let mut value = initial;
        for x in 0..self.size_x as i32 {
            for y in 0..self.size_y() as i32 {
                value = fold_func(value, &self.get_value(x, y), x, y);
            }
        }
        return value;
    }

    pub fn coordinates_with_filter(&self, filter: fn(&T) -> bool) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        for y in 0..self.size_y() as i32 {
            for x in 0..self.size_x() as i32 {
                if filter(&self.get_value(x, y)) {
                    result.push((x, y))
                }
            }
        }
        return result;
    }

    #[inline]
    pub fn matches_in_straight_direction(
        &self,
        x: i32,
        y: i32,
        move_x: i32,
        move_y: i32,
        letter: &[T],
    ) -> bool
    where
        T: Eq,
    {
        if self.get_value(x + move_x, y + move_y) == letter[0] {
            if letter.len() == 1 {
                return true;
            } else {
                self.matches_in_straight_direction(
                    x + move_x,
                    y + move_y,
                    move_x,
                    move_y,
                    &letter[1..],
                )
            }
        } else {
            return false;
        }
    }

    pub fn regions_with_filter(
        &self,
        filter: fn(&T) -> bool,
        connected_to: fn(&T, &T) -> bool,
    ) -> Map2D<i32>
    where
        T: PartialEq,
    {
        let mut result = Map2D::new(-1, self.size_x);
        result.data.resize(self.size_y() * self.size_x(), -1);

        let mut todo_list = vec![];

        for x in 0..self.size_x as i32 {
            for y in 0..self.size_y() as i32 {
                let value_current = self.get_value(x, y);
                if filter(&value_current) {
                    todo_list.push((x, y));
                }
            }
        }

        let mut next_group_id = 0;

        while let Some((x, y)) = todo_list.pop() {
            let value_current = self.get_value(x, y);

            if !filter(&value_current) || result.get_value(x, y) != -1 {
                continue;
            }

            let directions = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];

            let mut current_group_id = None;

            // n = neighbor
            for (nx, ny) in directions {
                let neighbor_value = self.get_value(nx, ny);
                let neighbor_group = result.get_value(nx, ny);
                if filter(&neighbor_value) && connected_to(&value_current, &neighbor_value) {
                    if neighbor_group < 0 {
                        todo_list.push((nx, ny))
                    } else {
                        current_group_id = Some(neighbor_group);
                    }
                }
            }

            if let Some(group) = current_group_id {
                result.set_value(x, y, group)
            } else {
                result.set_value(x, y, next_group_id);
                next_group_id += 1;
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Map2D;

    #[test]
    pub fn test_get_set_values() {
        let mut t = Map2D::new(None, 3);

        assert_eq!(0, t.size_y());

        // test for (0,0)
        assert_eq!(None, t.get_value(0, 0));
        t.set_value(0, 0, Some(1));
        assert_eq!(Some(1), t.get_value(0, 0));
        assert_eq!(1, t.size_y());

        // test for (2,2)
        assert_eq!(None, t.get_value(2, 2));
        t.set_value(2, 2, Some(100));
        assert_eq!(Some(100), t.get_value(2, 2));
        assert_eq!(3, t.size_y());

        // test for negative values and out-of-range
        assert_eq!(None, t.get_value(-1, 0));
        assert_eq!(None, t.get_value(0, -1));
        assert_eq!(None, t.get_value(1000, 0));
        assert_eq!(None, t.get_value(0, 1000));
        assert_eq!(None, t.get_value(3, 0));
        assert_eq!(None, t.get_value(0, 3));

        assert_eq!(3, t.size_y())
    }

    #[test]
    pub fn test_get_values_out_of_range() {
        let mut t = Map2D::new(None, 3);
        assert_eq!(None, t.get_value(0, 0));
        t.set_value(0, 0, Some(1));
        assert_eq!(Some(1), t.get_value(0, 0));
    }

    #[test]
    pub fn test_regions() {
        let mut t = Map2D::new(None, 3);
        // 55-
        // --6
        // -66
        t.set_value(0, 0, Some(5));
        t.set_value(1, 0, Some(5));
        t.set_value(2, 1, Some(6));
        t.set_value(2, 2, Some(6));
        t.set_value(1, 2, Some(6));

        let regions = t.regions_with_filter(|x| x.is_some(), |x, y| x == y);
        assert_eq!(regions.get_value(0, 0), regions.get_value(1, 0));
        assert_eq!(regions.get_value(2, 1), regions.get_value(2, 2));
        assert_eq!(regions.get_value(1, 2), regions.get_value(2, 2));
        // empty(filtered-out) space should be negative
        assert_eq!(-1, regions.get_value(2, 0));
        assert_eq!(-1, regions.get_value(0, 1));
        assert_eq!(-1, regions.get_value(1, 1));
    }
}
