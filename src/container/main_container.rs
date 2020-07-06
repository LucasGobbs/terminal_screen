use crate::buffer::Buffer;
#[derive(Debug, Default)]
struct MainContainer<T> 
where T: ComponentDrawable{

    children: Vec<Node<T>>,
}
struct Node<T>{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}