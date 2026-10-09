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
