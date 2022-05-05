# Mediantor-rs
 
![tests](https://github.com/SoVictor/mediantor-rs/actions/workflows/build_and_test.yml/badge.svg)
 
Mediantor is a container that stores a collection of integers and provides only two operations:

* `insert(x)` - adds _x_ to collection;
* `take()` - returns the value of the median element in the collection and removes this element.

This project provides three implementations of Mediantor:

1. As a sorted vactor, with complexity _O(N)_ for each operation;
2. As sqrt decomposition, with complexity _O(sqrt(N))_ for each operation;
3. As two heaps, with complexity _O(log(N))_ for each operation.

## Building

Run

`$ cargo build`

to build the project. No additional input is required.

## Running

Run 

`$ cargo run --example manual_input`

to manually test the project. See the section below about the structure of a test input.

Run 

`$ cargo test`

to get results of automated testing.

## Structure of a test input

The first line of input should contain one single integer _n_ (1 ≤ _n_ ≤ 10<sup>5</sup>) - a number of operations with Mediantor.

The following _n_ lines should contain descriptions of these operations. If the line reads like

`1 x`,

it means that `Insert(x)` will be performed (-10<sup>9</sup> ≤ _x_ ≤ 10<sup>9</sup>). If the line contains one single zero, it means that `Take()` performed.

It is granted that `Take()` will not be called when Mediantor is empty.

Manual tests should follow the same rules.

## Structure of a test output

For each called `Take()`, output will contain a line with a returned number.

## Example

| Input       | Output      |
| ----------- | ----------- |
| 10<br>1 1<br>1 2<br>1 5<br>1 3<br>1 9<br>0<br>0<br>0<br>1 -3<br>0<br> | 3<br>2<br>5<br>1<br><br><br><br><br><br><br><br> |
