use crate::Array2D;

#[derive(Clone)]
pub struct Array3D<T: Clone> {
    width: usize,
    height: usize,
    depth: usize,
    data: Vec<T>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coord3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Coord3D {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Coord3D { x, y, z }
    }
}

impl From<(usize, usize, usize)> for Coord3D {
    fn from(coord: (usize, usize, usize)) -> Self {
        Coord3D::new(coord.0, coord.1, coord.2)
    }
}

impl From<[usize; 3]> for Coord3D {
    fn from(coord: [usize; 3]) -> Self {
        Coord3D::new(coord[0], coord[1], coord[2])
    }
}

pub struct Iter<'a, T: 'a + Clone> {
    this: &'a Array3D<T>,
    coord: Coord3D,
}

impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = (Coord3D, &'a T);

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
        if self.coord.y >= self.this.height {
            self.coord.y = 0;
            self.coord.z += 1;
        }
        next
    }
}

pub struct IterMut<'a, T: 'a + Clone> {
    this: &'a mut Array3D<T>,
    coord: Coord3D,
}

impl<'a, T: Clone> Iterator for IterMut<'a, T> {
    type Item = (Coord3D, &'a mut T);

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
        if self.coord.y >= self.this.height {
            self.coord.y = 0;
            self.coord.z += 1;
        }
        next
    }
}

impl<T: Clone + Default> Array3D<T> {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self::new_with(width, height, depth, T::default())
    }
}

impl<T: Clone> Array3D<T> {
    pub fn new_with(width: usize, height: usize, depth: usize, default: T) -> Self {
        let mut data: Vec<T> = Vec::new();
        data.resize(width * height * depth, default);
        Array3D {
            width,
            height,
            depth,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    fn coord_is_valid(&self, coord: Coord3D) -> bool {
        coord.x < self.width && coord.y < self.height && coord.z < self.depth
    }

    fn coord_index(&self, coord: Coord3D) -> usize {
        coord.x + self.width * coord.y + self.width * self.height * coord.z
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            this: self,
            coord: Coord3D::new(0, 0, 0),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            this: self,
            coord: Coord3D::new(0, 0, 0),
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn at<C: Into<Coord3D>>(&self, coord: C) -> &T {
        let coord = coord.into();
        assert!(self.coord_is_valid(coord));
        &self.data[self.coord_index(coord)]
    }

    pub fn at_mut<C: Into<Coord3D>>(&mut self, coord: C) -> &mut T {
        let coord = coord.into();
        assert!(self.coord_is_valid(coord));
        let index = self.coord_index(coord);
        &mut self.data[index]
    }

    pub fn set<C: Into<Coord3D>>(&mut self, coord: C, value: T) {
        let coord = coord.into();
        *self.at_mut(coord) = value;
    }

    pub fn copy_2d<C: Into<Coord3D>>(&mut self, source: &Array2D<T>, dest: C) {
        let dest = dest.into();
        assert!(dest.x + source.width() <= self.width);
        assert!(dest.y + source.height() <= self.height);

        for i in 0..source.height() {
            let dst = Coord3D::new(dest.x, dest.y + i, dest.z);
            let dst_begin = self.coord_index(dst);
            let dst_end = dst_begin + source.width();
            let src_begin = source.width() * i;
            let src_end = src_begin + source.width();

            let src = &source.data()[src_begin..src_end];
            let dst = &mut self.data[dst_begin..dst_end];
            dst.clone_from_slice(src);
        }
    }
}
