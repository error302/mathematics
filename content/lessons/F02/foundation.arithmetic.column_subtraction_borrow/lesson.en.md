# Columnar Subtraction with Decomposition (Borrow)

## 1. The Challenge
You need to calculate $52 - 27$.
When you look at the units column ($2 - 7$), $2$ is smaller than $7$. Since we cannot take $7$ whole units away from $2$ without entering negative quantities, what must we do?

## 2. Predict Before Calculating
Predict:
- Is $52 - 27$ greater than or less than $30$?
- Does decomposing a ten from $52$ change the total value of $52$?

## 3. The Algebraic Justification: Unbundling a Ten
Expand the minuend $52$:
$$52 = (5 \times 10) + (2 \times 1)$$
Decompose $1$ of the tens into $10$ units:
$$52 = (4 \times 10) + (1 \times 10 + 2 \times 1) = (4 \times 10) + (12 \times 1)$$
The total value ($40 + 12 = 52$) is strictly unchanged.

Now subtraction can proceed column-by-column:
$$(4 \times 10 + 12 \times 1) - (2 \times 10 + 7 \times 1)$$
$$= (4 - 2) \times 10 + (12 - 7) \times 1$$
$$= 2 \times 10 + 5 \times 1 = 25$$

## 4. The Standard Column Algorithm
$$\begin{array}{r@{\quad}l}
  & \overset{4}{\cancel{5}}\overset{12}{2} \\[-2pt]
- & 27 \\ \hline
  & 25
\end{array}$$

1. **Units Column:** Borrow $1$ ten from $5$ (leaving $4$ tens). Add $10$ to $2$ to make $12$. Compute $12 - 7 = 5$.
2. **Tens Column:** Compute $4 - 2 = 2$.
3. **Difference:** $25$.

## 5. Verification by Inverse Operation
Always check subtraction by adding the difference back to the subtrahend:
$$25 + 27 = (20 + 20) + (5 + 7) = 40 + 12 = 52$$
Since $25 + 27 = 52$, the answer is mathematically certain.

## 6. Misconception Challenge
**The Error:** Subtracting the smaller digit from the larger digit regardless of position ($7 - 2 = 5$, giving $35$).
**Why this is invalid:** Subtraction is not commutative! The top number is the total from which the bottom number is removed. Subtracting $7 - 2$ inverts the problem.

## 7. Reflection
Borrowing is simply the inverse of carrying: where addition bundles $10$ units into $1$ ten, subtraction unbundles $1$ ten into $10$ units.
