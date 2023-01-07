# Smart pointers
## Reference cycles can leak memory
- though difficult, it is not impossible to create memory leak
- using `Rc` and `RefCell`, it is possible to create references where items refer to each other in cycle.
```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // this adds the reference to b at a
    // but b also has a refrence to a
    // which creates a refrence cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack, as the program cannot find the tail
    // println!("a next item = {:?}", a.tail());
}
```
- Rust cannot drop values for a and b since the reference count of `Rc` remains at 1 even after dropping the original references to the values
- when you have the combination of interiro mutability and counting references, you must ensure that you don't create cycles

## Preventing reference cycles: turning an Rc<T> into a Weak<T>
- `Rc` instance is only cleaned up once the `strong_coung` is 0.
- you can also create a *weak reference* to the value within an `Rc` instance by calling `Rc::downgrade` and passing a reference to the `Rc`.
- weak references don't express an ownership relationship, and their count does not affect when an `Rc` instance is cleaned up
- `Rc::downgrade` creates smart pointer of type `Weak<T>`
  - because the value that `Weak` references might have been dropped, to do anything with the value that `Weak` is pointing to, you must call `upgrade` method on `Weak` to ensure that the value still exists
  - the `upgrade` method returns `Option<Rc<T>>`, so you can handle `Some` when there is a value and `None` when the value has already been dropped

### Example with a tree data structure: a node with child nodes
```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    // node in leaf has two owners leaf and branch

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // at this point leaf parent is None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    // leaf parent is mutated to contain the reference to branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```
- node know about their children nodes
- we also want that the children also know about their parent
  - but we cannot use `Rc` because that would create a reference cycle
  - a parent node should own its children: if a parent node is dropped, its child nodes should also be dropped. 
  - However, a child should not own its parent: if we drop a child node, the parent should still exist. 
```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),  // leaf strong = 1
        Rc::weak_count(&leaf),    // leaf weak   = 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // leaf strong = 1 (branch)
            Rc::weak_count(&branch),   // leaf weak   = 1 (leaf)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),  // leaf strong = 2 (leaf and branch)
            Rc::weak_count(&leaf),    // leaf weak   = 0
        );
      // branch is dropped as its strong count reaches 0
    }

    // None, since the value that the weak points to has been cleared
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());  
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```
