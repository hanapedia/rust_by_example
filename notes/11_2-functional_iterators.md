# Processing series of data with Iterators
- an iterator is responsible for the logic of iterating over each item and determining when the sequence has finished
- iterators can be used in many kinds of sequences not just data structures that you can index into
## The iterator trait and the `next` method
- all iterators implement a trait named `Iterator` that is defined in the standard library.
  - it requires you to define an associated type, which is usd as the return type of the `next` method
  - and `next` method, which returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns `None`
  - the iterator must be mutable since next method mutates the internal state
    - but not when used with `for` loops as the loop takes the ownership and makes it mutable behind the scenes 
    - also the values we get from the calls to `next` are immutable references to the values in the vector
    - use `into_iter` if you want to create an iterator that takes ownership
      - the iterator takes the ownership of the vector
    - use `iter_mut` if you want to iterate over mutable references
      - makes the items of the iterator mutable and not the iterators themselves
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations elided
}
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // the iterator must be mutable since next method mutates the internal state
        let mut v1_iter = v1.iter(); 

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```
## Methods that consumes iterators
- some of the default methods of iterator traits uses `next` and calling them uses up the iterator. 
- they are called *consuming adaptors*
- `sum` method is one example
```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        // sum takes the owenship of v1_iter and returns integer
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```
## Methods that produce other iterators
- some of the default methods of iterator traits don't consume the iterator, but instead they produce different iterators by changing some aspect of the original iterator
- they are called *iterator adaptors*
- `map` method is one example
```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    // map takes a closure to be executed on each item
    v1.iter().map(|x| x + 1);
    // this code produces a warning since the iterators are lazy, meaning they have no effect until you call methods on them
    // so call a method like collect
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```
- many iterator adaptors takes closures as arguments, and commonly the closures we'll specify as arguments to iterators will be closures that captures their environment
- another exmaple is `filter` method

## Choosing between loops and iterators 
### Performance
- iterator is a zero-cost abstraction, meaning that using iterator does not change the performance of the execution
  - both the low level loop and iterators gets compiled down to roughly the same code
