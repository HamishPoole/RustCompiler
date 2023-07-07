<h2 align="center">VC Compiler</h2>
<p align="center"> A fun little Rust compiler, exploring the Rust programming language & minimalist programming.
WIP. 
</p>
<p align="center">Currently targets a subset of C called VC, by Jingling Xue. 
</p>
<h2 align="center">Usage</h2>

### Setup:

- Clone the repository.

```
git clone git@github.com:HamishPoole/RustCompiler.git
```

- Build using build locally.

```
source build_locally.sh
```

- Either install locally via install locally

```
source install_locally.sh
```

- Or alias the build binary to vc

```
alias vc="./vc"
```

### Usage:

- Parse a file and print the abstract syntax tree with the following command.

```
vc parse <filepath>
```

- Parse, and print out the original file from the abstract syntax tree using unparse.

```
vc unparse <filepath>
```
