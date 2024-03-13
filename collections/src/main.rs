// 1 arrays have a fixed size and elements of the same data type

// Example 1: Array of integers
let numbers: [i32; 3] = [1, 2, 3];

// Example 2: Array of characters
let chars: [char; 4] = ['a', 'b', 'c', 'd'];

// Example 3: Array of booleans
let flags: [bool; 5] = [true, false, true, false, true];



// 2 vectors are dynamic arrays that can grow or shring in size

// Example 1: Vector of strings
let fruits: Vec<&str> = vec!["Apple", "Banana", "Orange"];

// Example 2: Vector of integers
let numbers: Vec<i64> = vec![10, 20, 30, 40, 50];

// Example 3: Vector of tuples
let coordinates: Vec<(f64, f64)> = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];



// 3 slices are references to a contiguous sequence of elements in another collection

// Example 1: Slice of an array
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let slice_array = &numbers[1..4];

// Example 2: Slice of a vector
let fruits: Vec<&str> = vec!["Apple", "Banana", "Orange"];
let slice_vector = &fruits[1..3];

// Example 3: Slice of a string
let message: &str = "Hello, World!";
let slice_string = &message[7..12];



// 4 Tuples are fixed-size ordered collections of elements of different types

// Example 1: Tuple of integers and strings
let person: (i32, &str) = (25, "Alice");

// Example 2: Tuple of boolean values
let status: (bool, bool, bool) = (true, false, true);

// Example 3: Tuple of mixed types
let mixed: (i32, f64, &str) = (42, 3.14, "Hello");



// 5 HashMaps store key-value pairs and allow efficient lookups

use std::collections::HashMap;

// Example 1: HashMap of string keys and integer values
let mut grades = HashMap::new();
grades.insert("Alice", 95);
grades.insert("Bob", 87);
grades.insert("Charlie", 92);

// Example 2: HashMap of integer keys and string values
let mut ages = HashMap::new();
ages.insert(25, "Alice");
ages.insert(30, "Bob");
ages.insert(22, "Charlie");

// Example 3: HashMap of enum keys and boolean values
#[derive(Debug, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}
let mut favorite_colors = HashMap::new();
favorite_colors.insert(Color::Red, true);
favorite_colors.insert(Color::Green, false);
favorite_colors.insert(Color::Blue, true);


// 6 HashSets store unique elements and do not allow duplicates

use std::collections::HashSet;

// Example 1: HashSet of integers
let unique_numbers: HashSet<i32> = [1, 2, 3, 4, 5, 5, 4, 3, 2, 1].iter().cloned().collect();

// Example 2: HashSet of strings
let unique_words: HashSet<&str> = ["apple", "orange", "banana", "apple"].iter().cloned().collect();

// Example 3: HashSet of custom structs
#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
let unique_points: HashSet<Point> = vec![
    Point { x: 1, y: 2 },
    Point { x: 3, y: 4 },
    Point { x: 1, y: 2 },
].into_iter().collect();

// 7 LinkedLists are collections of elements with dynamic size and efficient insertion and removal.

use std::collections::LinkedList;

// Example 1: LinkedList of characters
let mut characters = LinkedList::new();
characters.push_back('A');
characters.push_back('B');
characters.push_back('C');

// Example 2: LinkedList of strings
let mut words = LinkedList::new();
words.push_back("apple");
words.push_back("banana");
words.push_back("orange");

// Example 3: LinkedList of integers
let mut numbers = LinkedList::new();
numbers.push_back(1);
numbers.push_back(2);
numbers.push_back(3);


// 8 BinaryHeaps are priority queues that maintain the maximum (or minimum) element at the root.

use std::collections::BinaryHeap;

// Example 1: BinaryHeap of integers (max heap)
let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
max_heap.push(10);
max_heap.push(5);
max_heap.push(8);

// Example 2: BinaryHeap of integers (min heap)
let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();
min_heap.push(10);
min_heap.push(5);
min_heap.push(8);

// Example 3: BinaryHeap of custom structs (max heap)
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Student {
    name: String,
    grade: i32,
}
let mut student_heap: BinaryHeap<Student> = BinaryHeap::new();
student_heap.push(Student { name: "Alice".to_string(), grade: 90 });
student_heap.push(Student { name: "Bob".to_string(), grade: 87 });
student_heap.push(Student { name: "Charlie".to_string(), grade: 92 });






