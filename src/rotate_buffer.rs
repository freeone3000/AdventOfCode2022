#[derive(Debug)]
pub struct RotateBuffer<R> {
    pos: usize,
    max_size: usize,
    buffer: Vec<R>,
}

#[derive(Debug)]
pub struct RotateBufferIter<'buf, R> {
    iter_pos: usize,
    rot_buffer: &'buf RotateBuffer<R>,
}

impl <'buf, R> Iterator for RotateBufferIter<'buf, R>
    where R: Default + Eq + Clone
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_pos >= self.rot_buffer.max_size {
            None
        } else {
            let ret = self.rot_buffer.buffer[(self.iter_pos + self.rot_buffer.pos) % self.rot_buffer.max_size].clone();
            if ret == R::default() {
                None
            } else {
                self.iter_pos += 1;
                Some(ret)
            }
        }
    }
}

impl <R: Default> RotateBuffer<R>
    where R: Ord + Clone + Default
{

    pub fn new(max_size: usize) -> RotateBuffer<R> {
        let mut rot_buffer = RotateBuffer {
            pos: 0,
            max_size,
            buffer: Vec::new(),
        };
        rot_buffer.buffer.resize(max_size, R::default());
        rot_buffer
    }

    pub fn push(&mut self, item: R) {
        self.buffer[self.pos] = item;
        self.pos = (self.pos + 1) % self.max_size;
    }

    pub fn iter(&self) -> RotateBufferIter<R> {
        RotateBufferIter {
            iter_pos: 0,
            rot_buffer: self,
        }
    }

    // this feels slow.
    pub fn distinct(&self) -> usize
        where R: PartialEq
    {
        let mut distinct = vec![];
        for item in self.iter() {
            if !distinct.contains(&item) {
                distinct.push(item.clone());
            }
        }
        distinct.len()
    }
}
