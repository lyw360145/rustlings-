# Vectors

Vectors are one of the most-used Rust data structures. In other programming
languages, they'd simply be called Arrays, but since Rust operates on a
bit of a lower level, an array in Rust is stored on the stack (meaning it
can't grow or shrink, and the size needs to be known at compile time),
and a Vector is stored in the heap (where these restrictions do not apply).

Vectors are a bit of a later chapter in the book, but we think that they're
useful enough to talk about them a bit earlier. We shall be talking about
the other useful data structure, hash maps, later.


**向量（Vectors）是 Rust 中最常用的数据结构之一。** 在其他编程语言中，它们通常直接被称为数组（Arrays），但由于 Rust 在稍低层的层面运行，Rust 中的数组存储在栈上（这意味着它不能增长或缩小，且大小需要在编译时已知），而向量存储在堆上（在那里这些限制不适用）。

向量在本书中算是稍后一点的章节，但我们认为它们足够有用，值得提前讨论一下。我们将在后面讨论另一个有用的数据结构——哈希映射（Hash Maps）。

## Further information

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
