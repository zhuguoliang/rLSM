// min heap node
pub struct MinHeapNode{
    pub element:usize, // The element to be stored
    pub i:usize, // index of the array from which the element is taken
    pub j:usize  // index of the array from which the element is taken
}

pub struct MinHeap{
    //pub hv:Vec<MinHeapNode>
    pub harr:&'static MinHeapNode,
    pub heap_size:usize
}

pub fn swap(x:&mut MinHeapNode, y:&mut MinHeapNode) {
    std::mem::swap(x, y);
}
pub fn left(i:usize)  -> usize{(2*i + 1)} // to get index of left child of node at index i
pub fn right(i:usize) -> usize{(2*i + 2)} // to get index of right child of node at index i

impl MinHeap{
// Constructor: Builds a heap from a given array a[] of given size
    pub fn init(&mut self, a:&'static MinHeapNode, size:usize){
        self.heap_size = size;
        self.harr = a;
        let mut i:usize = (self.heap_size - 1)/2;
        while i >=0 {
            self.MinHeapify(i);
            i = i-1;
        }
    }
// A recursive method to heapify a subtree with root at given index
// This method assumes that the subtrees are already heapified
    pub fn MinHeapify(&mut self, i:usize){
        let l = left(i);
        let r = right(i); 
        let smallest = i;
        if l < self.heap_size && self.harr[l].element < self.harr[i].element {
            let smallest = l;
        }
        if r < self.heap_size && self.harr[r].element < self.harr[smallest].element {
            let smallest = r;
        }
        if smallest != i {
            std::mem::swap(&mut self.harr[i], &mut self.harr[smallest]);
            self.MinHeapify(&mut self, smallest);
        }
    }

}

#[test]
fn test_heap(){

}
// to get index of left child of node at index i
//int left(int i) { return (2*i + 1); }

// to get index of right child of node at index i
//int right(int i) { return (2*i + 2); }

