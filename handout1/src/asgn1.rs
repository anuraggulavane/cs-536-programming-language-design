pub mod asgn1{

/**  Q1: 
 * TASK: 
 *  Implement the function hello_world which prints 
 * "Hello, world!" using the println! function.
 * Staff solution length: 1 line of code. */
pub fn hello_world() {
    println!("Hello, world!");
}

/** Q2.A:
 * TASK: 
 * Implement the function get which accepts an integer array (0-indexed), 
 * an index (i) into the array, and returns the i'th element of the array.
 * Note: "usize" means an integer big enough to store array indices.
 * Assume that the array is big enough.
* Staff solution length: 1 line of code. */
pub fn get(a: &[i64], i: usize) -> i64 {
    a[i]
}


/** Q2.B
 * TASK:
 * Implement the function square which accepts an integer x
 * and returns x^2
 * Staff solution length: 1 line of code. */
pub fn square(arg: i64) -> i64 {
    arg * arg
}

/** Q2.C:
 * TASK:
 * Implement the function square_array which accepts a
 * (mutable reference to an) integer array of unknown length.
 * Modify it in-place, updating every array element x to x^2  
 * Staff solution length: 3 lines of code. */
pub fn square_array(arg: &mut [i64]) {
    for i in 0..arg.len() {
        arg[i] = arg[i] * arg[i];
    };
}

/** Q3.A:  
 * TASK: 
 * Implement the function reverse_array which accepts a
 * (mutable reference to an) integer array containing
 * exactly 10 elements.
 * Use a for loop to reverse the 10-element array in-place.
 * Staff solution length: 5 lines of code. */
 pub fn reverse_array(arg: &mut [i64]) {
    let n = arg.len();
    for i in 0..n /2 {
        let temp = arg[i];
        arg[i] = arg[n - 1 - i];
        arg[n - 1 - i] = temp;
    }
}

/** Q3.B:  
 * TASK:
 * Implement the function sum_to_index which accepts 
 * an integer n and returns the sum from i = 1 to n of i^2.
 * Return a sum of zero elements when n < 1.
 * Staff solution length: 5 lines of code.  */
 pub fn sum_to_index(n: i64) -> i64 {
    if n < 1 {
        return 0;
    }
    let mut sum = 0;
    for i in 1..=n {
        sum += i * i;
    }
    sum
}


/** Q3.C:  
 * TASK: 
 * Implement the function sum_until_zero which accepts 
 * (an immutable reference to) an array of integers.
 * Compute the sum of its elements one-by-one until you an encounter a 0,
 * then return the total count so far.
 * If there is no 0, return the sum of the entire array.
 * Staff solution length: 7 lines of code.  
 */
pub fn sum_until_zero(arg: &[i64]) -> i64 {
    let mut sum = 0;
    for &num in arg {
        if num == 0 {
            break;
        }
        sum += num;
    }
    sum
}

/* 
   This definition is provided for you to use in a following task.
   You do not need to edit the definition in any way.
   It defines a type named IntMap, which represents a map
   from integers to integers. The type is defined recursively.
   Empty represents an empty map.
   Node(l,k,v,r)  represents a nonempty map where looking up the key  "k"
   would return the value "v". Map l contains all keys less than k.
   Map r contains all keys greater than k. Keys are assumed to be unique.
*/
#[derive (PartialEq,Eq,Clone)]
pub enum IntMap {
    Empty,
    Node(Box<IntMap>,i64, i64, Box<IntMap>),
}
 
/** Q4.
 * TASK:
 * Implement the function bsearch, which takes in a (boxed) IntMap and key,
 * then searches the IntMap for the key.
 * Assume that the IntMap is a binary search tree, i.e., it is sorted by key.
 * Do not assume that the given key is in the map; return -1 if not.
 * If it is in the map, return the stored value, which may or may not be -1.
 * A correct solution should take advantage of sortedness and should
 * run in time proportional to tree depth, not total number of nodes.
 * Staff solution length: 11 lines of code.  
*/
pub fn bsearch(t: Box<IntMap>, key: i64) -> i64 {
    match t.as_ref() {
        IntMap::Empty => -1,
        IntMap::Node(l,k,v,r) => {
            if key == *k {
                *v
            }
            else if key < *k {
                bsearch(l.clone(), key)
            }
            else {
                bsearch(r.clone(), key)
            }
        }
    }
}

/** Q4.B 
 * TODO:
 * Implement the function insert, which takes a boxed map, as well 
 * as a key and value, and returns a new map with the key-value pair inserted.
 * Assume the input map is sorted and ensure the output map is sorted.
 * If the key already exists, replace its value with the given value.
 * If not, add the new key with the new value.
 * Staff solution length: 11 lines of code.  
*/
pub fn insert(t: Box<IntMap>, key: i64, value: i64) -> Box<IntMap> {
    match *t {
        IntMap::Empty => Box::new(IntMap::Node(Box::new(IntMap::Empty), key, value, Box::new(IntMap::Empty))),
        IntMap::Node(l,k ,v ,r ) => {
            if key == k {
                Box::new(IntMap::Node(l, key, value, r))
            }
            else if key < k {
                Box::new(IntMap::Node(insert(l, key, value), k, v, r))
            }
            else {
                Box::new(IntMap::Node(l, k, v, insert(r,key,value)))
            }
        }
    }
}
}