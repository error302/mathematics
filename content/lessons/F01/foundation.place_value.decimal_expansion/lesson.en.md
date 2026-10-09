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
