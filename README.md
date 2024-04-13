# Soroban Auditor

**soroban-auditor** is a decompiler for WebAssembly Smart Contracts binaries deployed on the Stellar ledger. It can decompile WASM binaries from the MVP version 1; however, it's still under development, and some features, such as proper type recovery, are still missing, resulting in some binaries producing pretty unreadable output.

## Requirements
Running **soroban-auditor** requires libz3 (version 4.8.6 or 4.8.7 should work).

## Building from source
Building or installing **soroban-auditor** from source requires a working Rust Installation (probably at least version 1.37.0).

## Features
- **Tlsh**: This package provides functionality for calculating the Trend Micro Locality Sensitive Hash (TLSH) of input data.
- **Lcs**: The Longest Common Subsequence (LCS) package enables finding the longest subsequence present in given sequences.
- **levenshtein**: The Levenshtein package offers tools for computing the Levenshtein distance between two strings.

## Developers
[stellarchain.io](https://stellarchain.io)
