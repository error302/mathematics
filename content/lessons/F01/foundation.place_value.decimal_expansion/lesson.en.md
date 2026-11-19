# Positional Notation & Decimal Expansion

## 1. The Challenge
Look at the numbers $47$ and $74$. Both contain the identical digits $\{4, 7\}$.
Yet, $74$ is much greater than $47$. Why does the order of symbols change the quantity so radically?

## 2. Predict Before Calculating
In the numeral $444$:
- Do all three $4$'s denote the same quantity?
- How much larger is the leftmost $4$ compared to the rightmost $4$?

## 3. The Positional Principle
In the Hindu-Arabic base-$10$ decimal system:
The value represented by a digit $d \in \{0, 1, 2, \dots, 9\}$ depends strictly on its **position** relative to the units column.

For an integer written with digits $d_k d_{k-1} \dots d_1 d_0$:
$$N = \sum_{i=0}^k d_i \cdot 10^i = d_k \cdot 10^k + d_{k-1} \cdot 10^{k-1} + \dots + d_1 \cdot 10^1 + d_0 \cdot 10^0$$

## 4. Worked Example: Decomposing 843
Consider $843$:
- The digit $3$ is in position $0$ (units): $3 \times 10^0 = 3 \times 1 = 3$.
- The digit $4$ is in position $1$ (tens): $4 \times 10^1 = 4 \times 10 = 40$.
- The digit $8$ is in position $2$ (hundreds): $8 \times 10^2 = 8 \times 100 = 800$.

Summing these positional values restores the original number:
$$800 + 40 + 3 = 843$$

## 5. Face Value vs. Place Value
- **Face Value:** The intrinsic numeral symbol itself ($4$).
- **Place Value:** The product of the face value and the base weight ($4 \times 10 = 40$).

## 6. Misconception Challenge
**The Error:** "In $843$, the value of the middle digit is $4$."
**Correction:** The face value is $4$, but because it occupies the tens column, its value in the number is $40$.

## 7. Key Takeaway
Positional notation allows a finite set of $10$ symbols $\{0, \dots, 9\}$ to represent arbitrarily large integers through power-of-ten scaling.


## Additional learning activities (draft)

**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check

Read the digits 0–9 and distinguish a written symbol from a quantity. The same digit can contribute different amounts in different positions.

### Representation

For 843, use 8 hundred blocks, 4 ten rods, and 3 single units. This makes 800 + 40 + 3. The digit 4 contributes 40, not 4.

### Complete a partly worked problem

Complete: 507 = __ hundreds + __ tens + __ units = __ + __ + __.

Self-check: 5 hundreds + 0 tens + 7 units = 500 + 0 + 7. The zero reserves the tens position.

### Independent practice

1. In 462, state both the digit and its value in the tens column.

   Self-check: The digit is 6; its value is 60.

2. Explain why 305 and 35 represent different numbers.

   Self-check: 305 = 300 + 5; 35 = 30 + 5. The zero preserves the empty tens position in 305.

### Explain your reasoning

Explain to someone why 74 and 47 differ although they use the same symbols. Name the position and contribution of each digit.

### Transfer to a new situation

A store has 3 boxes of 100 beads, 12 bundles of 10, and 4 single beads. Write the total as a standard numeral.

Self-check: 300 + 120 + 4 = 424. Twelve tens regroup into one hundred and two tens.

### Retrieve after a delay

Tomorrow decompose 608 without notes. A week later, give two different regrouped descriptions of 352.

The learner-facing presentation and these activities are stored in `lesson.json` beside this file.
