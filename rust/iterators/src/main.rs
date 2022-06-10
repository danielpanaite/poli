struct MyVect<T> {
    data: Vec<T>,
    size: usize
}

impl<T> MyVect<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: 0
        }
    }

    pub fn push(&mut self, var: T) {
        self.size += 1;
        self.data.push(var)
    }

    pub fn iter(&self) -> MyRefIterator<T> {
        MyRefIterator::new(self)
    }

    pub fn iter_mut(&mut self) -> MyMutIterator<T> {
        MyMutIterator::new(self)
    }
}

struct MyIterator<T> {
    container: MyVect<T>,
    selected_element: usize
}

impl< T> MyIterator< T> {
    fn new(c: MyVect<T>) -> Self {
        Self {
            container: c,
            selected_element: 0
        }
    }
}
struct MyRefIterator<'a, T> {
    container_ref: &'a Vec<T>,
    selected_element: usize
}

impl<'a, T> MyRefIterator<'a, T> {
    fn new(c: &'a MyVect<T>) -> Self {
        Self {
            container_ref: c.data.as_ref(),
            selected_element: 0
        }
    }
}

struct MyMutIterator<'a, T: 'a> {
    container_ref: Box<std::slice::IterMut<'a, T> >,
}

impl<'a, T> MyMutIterator<'a, T> {
    fn new(c: &'a mut MyVect<T>) -> Self {
        Self {
            container_ref: Box::new(c.data.iter_mut()),
        }
    }
}

impl<T> IntoIterator for MyVect<T> {
    type Item = T;
    type IntoIter = MyIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'a,'b, T> IntoIterator for &'a MyVect<T> {
    type Item = &'a T;
    type IntoIter = MyRefIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(&self)
    }
}

impl<'a, T> IntoIterator for &'a mut MyVect<T> {
    type Item = &'a mut T;
    type IntoIter = MyMutIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new( self)
    }
}

impl<T> Iterator for MyIterator< T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        return if self.selected_element < self.container.size {
            self.selected_element += 1;
            self.container.data.pop()
        } else { None }
    }
}

impl<'a, T> Iterator for MyRefIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        return if self.selected_element < self.container_ref.len() {
            self.selected_element += 1;
            Some(&self.container_ref[self.selected_element-1])
        } else { None }
    }
}

impl<'a, T> Iterator for MyMutIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
            /*
            * For this iterator, we return directly the original (mutable) iterator
            * due to the fact that the example relies to a container to store data.
            */
            return self.container_ref.next()

    }
}

fn main() {
    let mut v: MyVect<i32> = MyVect::new();
    v.push(10);
    v.push(12);

    // MyMutIterator
    for c in &mut v { //o for c in v.iter_mut() {
        println!("{}", c);
        *c *= 2
    }

    // MyRefIterator
    for c in &v { // o for c in v.iter() {
        println!("{}", c);
    }

    // MyIterator
    for c in v { // o for c in v.into_iter() {
        println!("{}", c);
    }
    //println!("{}", v.data[0]); //=> this is not working because the previous iterator moves the data
}
