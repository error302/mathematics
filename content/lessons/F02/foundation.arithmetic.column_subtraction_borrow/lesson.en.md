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


## Additional learning activities (draft)

**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check

Decomposing one ten creates ten units while decreasing the tens count by one. The total must stay unchanged before subtraction.

### Representation

For 52 - 27, exchange one ten: 52 = 4 tens + 12 units. Remove 2 tens + 7 units; 2 tens + 5 units remain.

### Complete a partly worked problem

63 - 28: rewrite 63 as __ tens and __ units. The units difference is __ and the final result is __.

Self-check: 5 tens and 13 units; units difference 5; tens difference 3; result 35.

### Independent practice

1. Compute 71 - 46, then check by addition.

   Self-check: 71 = 6 tens + 11 units; result 25. Check: 46 + 25 = 71.

2. Compute 302 - 178. Explain how to exchange across the zero.

   Self-check: 302 = 2 hundreds + 10 tens + 2 units = 2 hundreds + 9 tens + 12 units. Subtract to get 124.

### Explain your reasoning

Why is subtracting the smaller digit from the larger digit separately in each column invalid? Use 52 - 27 as a counterexample.

### Transfer to a new situation

A budget is 205 and spending is 89. Estimate what remains, calculate exactly, and verify the result.

Self-check: About 200 - 90 = 110. Exact: 116. Check 89 + 116 = 205.

### Retrieve after a delay

Tomorrow compute 84 - 39. A week later explain the regrouping in 400 - 167, including each zero.

The learner-facing presentation and these activities are stored in `lesson.json` beside this file.
