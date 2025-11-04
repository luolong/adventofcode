# Advent of Code 2025 (Java)

This repository contains my solutions to Advent of Code puzzles over the years and in various programming languages.

This branch here represents solutions to Advent of Code 2025 solutions implemented in Java.

## Results (Day 1, 2, 3)
## Running and testing

This project uses Justfile to manage the build and execution of the solutions.

Basic usage (run all solutions):

```
./just run all
```

To run just one day:

```
./just run day 1
```


## Compilation

This project uses an [early access build of Java 26][1] with [JEP 401][2] (Project Valhalla).

Additionally, the compilation takes advantage of some of the ahead of time optimization options to pre-warm the runtime based on a _"test run"_.


[1]: https://jdk.java.net/valhalla/
[2]: https://openjdk.org/jeps/401


```
./just run all
Day 1 Part 1: 1191 (finished in 9 milliseconds 942 microseconds 441 ns)
Day 1 Part 2: 6858 (finished in 5 milliseconds 839 microseconds 470 ns)
```
