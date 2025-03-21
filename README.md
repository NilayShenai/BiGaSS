# BiGaSS Sort (Bidirectional Gapping Swap Sort)

## Overview

BiGaSS Sort is a sorting algorithm that combines ideas from **Comb Sort** and **Cocktail Shaker Sort** to improve sorting efficiency. It uses **dynamic gap shrinking** and **bidirectional traversal** to rearrange elements, making it a good choice for small to medium sized datasets.

## How It Works

### Bidirectional Gapped Passes

1. **Dynamic Gap Initialization**

   - The gap starts as the size of the array.
   - It is reduced in each step using a shrink factor of **10/13**.

2. **Forward Pass**

   - The array is scanned from **left to right**.
   - Elements that are `gap` positions apart are compared.
   - If needed, they are swapped.

3. **Backward Pass**

   - The array is scanned from **right to left**.
   - Elements at `gap` positions are compared and swapped if required.

4. **Repeat**

   - The process continues until the gap becomes **1** and no swaps are made in a full pass.

## Based On

BiGaSS Sort applies key concepts from:

1. **Comb Sort**

   - Uses **gap-based comparisons** to speed up sorting.
   - Shrinks the gap using a **10/13 factor**.

2. **Cocktail Shaker Sort**

   - Uses **both forward and backward passes**.
   - Helps smaller elements move to their correct positions quickly.

## Why Use BiGaSS Sort?

- **Works well for small to medium datasets**
- **In-place sorting with O(1) extra space**
- **Combines two proven sorting techniques for better efficiency**

## Complexity Analysis

| Case         | Time Complexity |
| ------------ | --------------- |
| Best Case    | O(n)            |
| Average Case | O(n²)           |
| Worst Case   | O(n²)           |

**Space Complexity:** O(1) (no extra memory needed)

## Example

### Input:

```rust
let mut arr = [5, 1, 4, 2, 8, 0, 3];
```

### Sorting Process:

#### Step-by-Step Process

**Gap = 5** (Shrink from 7 → 5)

- **Forward Pass (Left to Right)**
  - Compare elements 5 positions apart:
  - Index 0 vs 5: 5 vs 0 → Swap → `[0, 1, 4, 2, 8, 5, 3]`
  - No other swaps.
- **Backward Pass (Right to Left)**
  - No swaps.

**Gap = 3** (Shrink from 5 → 3)

- **Forward Pass**
  - Index 4 vs 1: 8 vs 1 → Swap → `[0, 8, 4, 2, 1, 5, 3]`
- **Backward Pass**
  - No swaps.

**Gap = 2** (Shrink from 3 → 2)

- **Forward Pass**
  - Index 1 vs 3: 8 vs 2 → Swap → `[0, 2, 4, 8, 1, 5, 3]`
  - Index 2 vs 4: 4 vs 1 → Swap → `[0, 2, 1, 8, 4, 5, 3]`
  - Index 3 vs 5: 8 vs 5 → Swap → `[0, 2, 1, 5, 4, 8, 3]`
  - Index 4 vs 6: 4 vs 3 → Swap → `[0, 2, 1, 5, 3, 8, 4]`
- **Backward Pass**
  - Index 6 vs 4: 4 vs 3 → Swap → `[0, 2, 1, 5, 4, 8, 3]`
  - Index 4 vs 2: 4 vs 1 → Swap → `[0, 2, 4, 5, 1, 8, 3]`

**Gap = 1** (Shrink from 2 → 1)

- **Forward Pass**
  - Index 3 vs 4: 5 vs 1 → Swap → `[0, 2, 4, 1, 5, 8, 3]`
  - Index 5 vs 6: 8 vs 3 → Swap → `[0, 2, 4, 1, 5, 3, 8]`
- **Backward Pass**
  - Index 5 vs 4: 3 vs 5 → Swap → `[0, 2, 4, 1, 3, 5, 8]`
  - Index 3 vs 2: 1 vs 4 → Swap → `[0, 2, 1, 4, 3, 5, 8]`
  - Index 2 vs 1: 1 vs 2 → Swap → `[0, 1, 2, 4, 3, 5, 8]`

**Final Pass (Adjacent Swaps)**

- **Forward Pass**
  - Swap 4 ↔ 3 → `[0, 1, 2, 3, 4, 5, 8]`
- **Backward Pass**
  - No swaps needed.

### Output:

```rust
[0, 1, 2, 3, 4, 5, 8]
```

