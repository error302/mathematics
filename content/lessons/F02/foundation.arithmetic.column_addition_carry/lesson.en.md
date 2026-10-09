# Columnar Addition with Regrouping (Carry)

## 1. The Challenge
You need to add two large quantities: $47$ and $38$.
When you add the units digits ($7 + 8$), the result is $15$. Since a base-$10$ units column can only hold a single digit ($0$ through $9$), what happens to the extra value?

## 2. Predict Before Calculating
Predict:
- What is the units digit of $47 + 38$?
- Is the total greater than $80$?

## 3. The Algebraic Justification of Column Addition
Write both numbers in their expanded polynomial form:
$$47 = (4 \times 10) + (7 \times 1)$$
$$38 = (3 \times 10) + (8 \times 1)$$
By commutativity and associativity of addition:
$$47 + 38 = (4 \times 10 + 3 \times 10) + (7 \times 1 + 8 \times 1)$$
$$= (4 + 3) \times 10 + (7 + 8) \times 1$$
$$= 7 \times 10 + 15 \times 1$$

Now apply the **base-10 trading rule**:
$$15 = 1 \times 10 + 5 \times 1$$
Substitute this back:
$$7 \times 10 + (1 \times 10 + 5 \times 1) = (7 + 1) \times 10 + 5 \times 1 = 8 \times 10 + 5 = 85$$

## 4. The Standard Column Algorithm
$$\begin{array}{r@{\quad}l}
  & \overset{1}{4}7 \\[-2pt]
+ & 38 \\ \hline
  & 85
\end{array}$$

1. **Units Column:** $7 + 8 = 15$. Write $5$ in the units answer position and carry $1$ ten to the top of the tens column.
2. **Tens Column:** $1 \text{ (carried)} + 4 + 3 = 8$. Write $8$ in the tens answer position.
3. **Total:** $85$.

## 5. Misconception Challenge
**The Error:** Writing both digits in the units column (e.g., writing $47 + 38$ as $715$).
**Why this fails:** A column in base-$10$ represents a single power of $10$. Writing $15$ in the units place erroneously shifts the tens digit into the hundreds place ($700 + 15 = 715$).

## 6. Reflection
Carrying is not an arbitrary rule — it is simply the algebraic regrouping of $10$ smaller units into $1$ larger unit of ten.
