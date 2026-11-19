# Base-10 Regrouping & Trading

## 1. The Challenge
You have $14$ single dollar coins. A shopkeeper prefers paper bills. If ten $1$ dollar coins can be exchanged for one $\$10$ bill, how do you express this amount canonically?

## 2. Predict Before Calculating
Can a single positional column in base-$10$ hold the number $14$?
Why is the numeral written as $14$ rather than a single symbol?

## 3. The Base-10 Trading Rule
In base $b = 10$, each rod or column can hold at most $b - 1 = 9$ discrete units.
As soon as $10$ units accumulate in position $k$:
$$10 \times 10^k = 1 \times 10^{k+1}$$
They must be bundled (**regrouped**) into $1$ unit of the next higher position $k+1$.

## 4. Worked Example: Regrouping 14 Units
1. Start with $14$ units: $14 \times 1$.
2. Decompose $14$ into $10 + 4$.
3. Trade the group of $10$ units for $1$ ten:
$$14 = (1 \times 10) + (4 \times 1)$$
4. Place $4$ in the units column and $1$ in the tens column:
Result: **14**.

## 5. Reverse Regrouping (Decomposition / Borrowing)
The trading rule works in reverse:
$$1 \text{ ten } = 10 \text{ units}$$
$$1 \text{ hundred } = 10 \text{ tens}$$
This reversible decomposition is the exact mathematical foundation of subtraction with borrowing.

## 6. Misconception Trap
**The Error:** Believing that regrouping changes the total quantity.
**The Truth:** Regrouping preserves exact numerical equality:
$$1 \text{ ten } + 4 \text{ units } = 14 \text{ units}$$
It merely changes the partition into canonical positional representatives.

## 7. Reflection
Regrouping is the bridge between concrete physical counting and abstract columnar computation.


## Additional learning activities (draft)

**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check

One ten is exactly ten units. Regrouping changes the description but preserves the number.

### Representation

Build 14 from fourteen units, then exchange ten units for one ten rod. Both collections have the same total: 1 ten + 4 units.

### Complete a partly worked problem

27 units = __ tens + __ units. 3 tens + 16 units = __ tens + __ units.

Self-check: 27 units = 2 tens + 7 units; 3 tens + 16 units = 4 tens + 6 units.

### Independent practice

1. Give 53 as tens/units, then exchange a ten into units.

   Self-check: 5 tens + 3 units = 4 tens + 13 units.

2. Is 2 hundreds + 13 tens + 4 units equal to 234? Explain.

   Self-check: No; it is 200 + 130 + 4 = 334.

### Explain your reasoning

Why does carrying one ten not add an extra ten to the original quantity? Explain the exchange rather than reciting the algorithm.

### Transfer to a new situation

You must remove 8 single beads from 32 stored as 3 bundles of ten and 2 singles. Show the exchange first.

Self-check: 32 = 2 tens + 12 singles. Remove 8 singles to leave 2 tens + 4 singles = 24.

### Retrieve after a delay

Tomorrow express 41 as 3 tens and some units. A week later explain both bundling and unbundling using 106.

The learner-facing presentation and these activities are stored in `lesson.json` beside this file.
