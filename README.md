# Compact and Malicious Private Set Intersection for Small Sets

This code is purely an educational project.

## Overview
This is a simple implementation of a private set intersection for small sets based on the paper   [**Compact and Malicious Private Set Intersection for Small Sets**](https://eprint.iacr.org/2021/1159). For small sets (500 items or fewer), this protocol offers the fastest performance and minimal communication compared to any known PSI protocol, including those that are only semi-honest secure or not based on Diffie-Hellman. 

## Features
1. Ideal permutation - aes-256 with a fixed key is used as the ideal permutation.
2. Elligator - [curve25519_elligator2](https://docs.rs/curve25519-elligator2/latest/curve25519_elligator2/index.html) (fork of curve25519-dalek crate) is used.
3. Polynomial - Lagrange interpolation

## Example:
The main code illustrates a simple example of the protocol. It takes two rounds for each sender and reciever.
### To do:
1. Fix Elligator bug
2. Use FFT for polynomial interpolation
3. Do a clean sweep for vulnerabilities