# ✨lemorage.github.io✨

This repository contains the source code for my personal website, built entirely in **pure Rust**. The blog site is built in [hugo](https://gohugo.io/), please refer to my another [repo](https://gitlab.com/lemorage/lemorage-blog) for more details.

## Installation Guide

Follow the steps below to build and run this project. 

> **Note**: The steps assume that Rust and the associated toolchain are already installed on your machine. If you haven't installed Rust yet, please refer to the official [Rust website](https://www.rust-lang.org/tools/install) for installation instructions.

To simplify the process of building and running this project, you can use `cargo-make`. Here’s how to set up the workflow:

### 1. Install `cargo-make`
If you haven't already installed `cargo-make`, you can do so by running the following command:

```bash
cargo install cargo-make
```

### 2. Clone the Repository
Start by cloning the project to your local machine:

```bash
git clone git@github.com:lemorage/lemorage.github.io.git
cd lemorage.github.io
```

### 3. Running the Automated Workflow
Once you've cloned the repository, you can run the entire build process by executing:

```bash
cargo make
```

This command will take care of building and running the project in one go.
