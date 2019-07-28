# employee_keeper
This is a slightly extended version of the exercise at the end of chapter 8 (Common Collections) of the Rust book: https://doc.rust-lang.org/book/ch08-03-hash-maps.html

The exercise specifies using HashMaps and Vecs to add and remove employees from departments.

Here, it's extended by using a couple of concepts later on in the book and also serializing and persisting the hashmap in a text file between runs + handling a ctrl-c signal.

#### Example commands:
  + ```Add John to Sales```
  + ```Get Sales```:  Prints all the employees in Sales
  + ```Remove John from Sales```
  + ```Print```:  Print everything in the Hashamp
