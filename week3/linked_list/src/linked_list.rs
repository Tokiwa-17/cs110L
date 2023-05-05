use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}
//#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl <T> Node<T>{
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl <T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl <T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl <T> Drop for LinkedList<T>{
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl <T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            value: self.value.clone(),
            next: self.next.clone(),
        }
    }
}

impl <T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> LinkedList<T> {
        LinkedList {head: self.head.clone(), size: self.size}
    }
}

impl <T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next == other.next
    }
}

impl <T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut result: bool = true;
        if self.size != other.size {
            result = false;
        }
        let mut self_current: &Option<Box<Node<T>>> = &self.head;
        let mut other_current: &Option<Box<Node<T>>> = &other.head;
        match (self_current, other_current) {
            (Some(node1), Some(node2)) => {
                if *node1 != *node2 {
                    result = false;
                }
            }
            (None, None) => {
                result = false;
            }
            (_, _) => {
                result = false;
            }
        }
        result
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for LinkedList<f64> {
    fn compute_norm(&self) -> f64 {
        let mut result: f64 = 0.0;
        let mut current: &Option<Box<Node<f64>>> = &self.head;
        loop {
           match current {
               Some(node) => {
                   result += node.value.powi(2);
                   current = &node.next;
               }
               None => {
                   break;
               }
           }
        }
        result.powf(0.5)
    }
}


