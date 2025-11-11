# Cryptography

This repository contains my work for lecture called *Cryptography*.

## Lab1

This project written in rust was about creating collision in md5 function based on article written by [Xiaoyun Wang, 2004](https://eprint.iacr.org/2004/199.pdf)

We had to first implement the hash function, which i did in rust. I also compared it to cargo crate which implemented the same function in a way that copied original implementation made in C. Benchmark was made on ryzen 5 on windows and my implementation was 5% faster on average.

Next we had to ceck if our implementation was in fact producing collisions with calculated beforhand blocks.

Lastly we had to implement second step single message modification algorithm that would find two 512bit blocks that would produce collison.

As an additional part we could implement first step or/and multi message modification for extra point. I did implement *mmm*.