#[derive(Clone)]
pub struct Array2D<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coord2D {
    pub x: usize,
    pub y: usize,
}

impl Coord2D {
    pub fn new(x: usize, y: usize) -> Self {
        Coord2D { x, y }
    }
}

impl From<(usize, usize)> for Coord2D {
    fn from(coord: (usize, usize)) -> Self {
        Coord2D::new(coord.0, coord.1)
    }
}

impl From<[usize; 2]> for Coord2D {
    fn from(coord: [usize; 2]) -> Self {
        Coord2D::new(coord[0], coord[1])
    }
}

pub struct Iter<'a, T: 'a + Clone> {
    this: &'a Array2D<T>,
    coord: Coord2D,
}

impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = (Coord2D, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.this.coord_is_valid(self.coord) {
            Some((self.coord, self.this.at(self.coord)))
        } else {
            None
        };
        self.coord.x += 1;
        if self.coord.x >= self.this.width {
            self.coord.x = 0;
            self.coord.y += 1;
        }
        next
    }
}

pub struct IterMut<'a, T: 'a + Clone> {
    this: &'a mut Array2D<T>,
    coord: Coord2D,
}

impl<'a, T: Clone> Iterator for IterMut<'a, T> {
    type Item = (Coord2D, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.this.coord_is_valid(self.coord) {
            let data = self.this.at_mut(self.coord) as *mut T;
            let data = unsafe { data.as_mut() }.unwrap();
            Some((self.coord, data))
        } else {
            None
        };
        self.coord.x += 1;
        if self.coord.x >= self.this.width {
            self.coord.x = 0;
            self.coord.y += 1;
        }
        next
    }
}

impl<T: Clone + Default> Array2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with(width, height, T::default())
    }
}

impl<T: Clone> Array2D<T> {
    pub fn new_with(width: usize, height: usize, default: T) -> Self {
        let mut data: Vec<T> = Vec::new();
        data.resize(width * height, default);
        Array2D {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn coord_is_valid(&self, coord: Coord2D) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn coord_index(&self, coord: Coord2D) -> usize {
        coord.x + self.width * coord.y
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            this: self,
            coord: Coord2D::new(0, 0),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            this: self,
            coord: Coord2D::new(0, 0),
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn at<C: Into<Coord2D>>(&self, coord: C) -> &T {
        let coord = coord.into();
        assert!(self.coord_is_valid(coord));
        &self.data[self.coord_index(coord)]
    }

    pub fn at_mut<C: Into<Coord2D>>(&mut self, coord: C) -> &mut T {
        let coord = coord.into();
        assert!(self.coord_is_valid(coord));
        let index = self.coord_index(coord);
        &mut self.data[index]
    }

    pub fn set<C: Into<Coord2D>>(&mut self, coord: C, value: T) {
        let coord = coord.into();
        *self.at_mut(coord) = value;
    }

    pub fn sub<C: Into<Coord2D>>(&self, coord: C, width: usize, height: usize) -> Self {
        let coord = coord.into();
        assert!(width > 0);
        assert!(height > 0);
        assert!(self.coord_is_valid(coord));

        let mut data: Vec<T> = Vec::new();
        for i in 0..height {
            let src_begin = self.width * (coord.y + i) + coord.x;
            let src_end = src_begin + width;
            data.extend_from_slice(&self.data[src_begin..src_end]);
        }

        Self {
            data,
            width,
            height,
        }
    }

    pub fn copy<C: Into<Coord2D>>(&mut self, source: &Self, dest: C) {
        let dest = dest.into();
        assert!(dest.x + source.width <= self.width);
        assert!(dest.y + source.height <= self.height);

        for i in 0..source.height {
            let dst_begin = self.width * (dest.y + i) + dest.x;
            let dst_end = dst_begin + source.width;
            let src_begin = source.width * i;
            let src_end = src_begin + source.width;

            let src = &source.data[src_begin..src_end];
            let dst = &mut self.data[dst_begin..dst_end];
            dst.clone_from_slice(src);
        }
    }
}
