# what-is-a-pointer

## Installation

This will install the necessary dependencies to run the code and build the project in debug mode.
```bash
cargo build
```

### To run the code:
```bash
./run.sh <example number>
```

### Example 1:
Show the structure of a pointer and the various values per page table entry.
```bash
./run.sh 1
```

### Example 2:
Iterate a block of memory and touch each page, it collects page faults and prints the page table entries.
```bash
./run.sh 2
```
