use crate::customers::CustomerOfProperty;

#[derive(Debug)]
pub struct MaxHeap<'a>{
    arr: Vec<CustomerOfProperty<'a>>,
}

pub fn create_heap(arr: Vec<CustomerOfProperty>) -> MaxHeap{
    let mut heap = MaxHeap{
        arr,
    };
    heap.build_max_heap();
    heap
}

impl MaxHeap<'_> {
    fn heapify(&mut self, i: usize){
        let mut largest = i;
        if  2*i + 1 < self.arr.len(){
            let left_child = self.arr[2*i + 1];
            if left_child.budget > self.arr[largest].budget{
                largest = 2*i + 1;
            }
        }
        if  2*i + 2 < self.arr.len(){
            let right_child = self.arr[2*i + 2];
            if right_child.budget > self.arr[largest].budget{
                largest = 2*i + 2;
            }
        }
        self.arr.swap(i, largest);
    }

    pub fn build_max_heap(&mut self){
        let mut index = self.arr.len() / 2;
        while index > 0{
            self.heapify(index-1);
            index -= 1;
        }
    }
    pub fn pop(&mut self) -> Option<CustomerOfProperty>{
        if self.arr.len() == 0 { return None; }
        let index = self.arr.len()-1;
        self.arr.swap(0, index);
        let element = self.arr.pop();
        if index != 0 { self.heapify(0) };
        match element {
            Some(x) => { Some(x) },
            None => { None },
        }
    }

}