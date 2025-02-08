# C-to-Rust Automated Translation Framework

This repository implements a multi-phase automated translation framework that converts legacy C code into modern, memory-safe Rust code. The approach preserves functionality while addressing compilation, error handling, and semantic equivalence challenges by leveraging advanced large language models (LLMs) and iterative refinement.

---

## Table of Contents

- [Overview](#overview)
- [Methodology](#methodology)
- [Evaluation](#evaluation)
- [Key Contributions](#key-contributions)
- [Replication](#replication)
- [Authors](#authors)
- [Citation](#citation)
- [Contact](#contact)

---

## Overview

The project focuses on automating the translation of C code to Rust, making it easier for developers to modernize legacy codebases. By combining multiple translation phases with error-driven repairs and automated testing, the framework ensures that the generated Rust code is semantically equivalent and functionally robust.

---

## Methodology

The translation process is organized into four key phases:

- **Initial Translation:**  
  Utilizes Mistral-7b-instruct as a baseline to generate an initial Rust translation from the input C code.

- **Compilation Validation:**  
  Checks the generated Rust code for compilation errors. When issues are detected, an error-driven refactoring mechanism uses compiler messages to guide necessary adjustments.

- **Fuzz Testing:**  
  Performs comprehensive fuzz testing to validate that the translated Rust code behaves identically to its C counterpart under varied inputs.

- **Multi-Agent Collaboration:**  
  Implements a feedback loop between Mistral and Llama agents, leveraging counterexample-driven enhancements to iteratively improve translation accuracy while maintaining lightweight configurations.

---

## Evaluation

The framework was evaluated using two distinct C libraries:

- **libopenaptx:** Consisting of 31 source files.
- **opl:** Consisting of 81 source files.

For each library, the methodology included configuring up to 12 repeated iterations of the initial translation, followed by fuzz testing with constraints such as a maximum input size of 32,768 bytes and a 7-minute execution timeout per test case.

---

## Key Contributions

- **Multi-Phase Translation:**  
  Integrates initial translation, compilation validation, fuzz testing, and multi-agent collaboration to tackle the challenges of translating legacy C code to Rust.

- **Error-Driven Repair:**  
  Leverages real-time compiler error messages and counterexample-based feedback to systematically address translation issues.

- **Advanced LLM Utilization:**  
  Incorporates state-of-the-art LLMs (Mistral-7b-instruct and Llama) to enhance translation fidelity and manage intricate code dependencies.

- **Scalability and Practicality:**  
  Demonstrates a scalable approach that automates the translation of real-world C libraries while reducing manual intervention.

---

## Replication

A complete replication package is available, including detailed instructions for dataset construction, configuration, and evaluation. Researchers and practitioners are encouraged to use this package to reproduce experimental results and further explore advanced translation techniques.

---

## Authors

- Quoc Hung Le (qle3@ncsu.edu)  
- Yung Hsuan Teng (yteng2@ncsu.edu)  
- Krishna Prashant Patel (kpatel43@ncsu.edu)  
- Dhyey Samirbhai Shah (dshah22@ncsu.edu)

---

## Citation

If you use this work in your research, please cite:

  Quoc Hung Le, Yung Hsuan Teng, Krishna Prashant Patel, and Dhyey Samirbhai Shah. 2024. *Conversational Agent for Code Translation to Rust*

---

## Contact

For questions, suggestions, or further discussion, please open an issue on GitHub or contact the project maintainers directly.

---

This README provides a detailed overview of the translation framework, outlining the methodology, evaluation procedures, and key contributions that enable robust, automated migration of C codebases to Rust.

---
Answer from Perplexity: pplx.ai/share
