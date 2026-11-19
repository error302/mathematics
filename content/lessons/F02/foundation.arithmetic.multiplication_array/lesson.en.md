# Multiplication: Equal Groups and Rectangular Arrays

## 1. The Challenge
If you arrange $24$ square tiles in $4$ rows of $6$, or in $6$ rows of $4$, why do both arrangements contain the exact same number of tiles? How does this visual pattern explain why order does not matter in multiplication?

## 2. Predict Before Calculating
Does rotating a $3 \times 5$ array of counters into a $5 \times 3$ array change the total count of items?

## 3. Mathematical Definition: Cartesian Product and Repeated Addition
For non-negative integers $a, b \in \mathbb{N}_0$:
1. **Repeated Addition:** Multiplication is defined as adding $a$ identical copies of $b$:
$$a \times b = \underbrace{b + b + \cdots + b}_{a \text{ times}}$$
2. **Set Theoretic Formulation:** Given finite sets $A$ with $|A| = a$ and $B$ with $|B| = b$, the product is the cardinality of their Cartesian product:
$$a \times b = |A \times B| = |\{(x, y) : x \in A, y \in B\}|$$

## 4. Fundamental Axiomatic Properties
Multiplication satisfies four core axioms over $\mathbb{N}_0$:
1. **Commutativity:** Order of factors does not alter the product:
$$a \times b = b \times a$$
*(Geometric intuition: rotating a rectangular array of dimension $a \times b$ by $90^\circ$ yields dimension $b \times a$ without creating or destroying cells).*
2. **Associativity:** Grouping does not alter the product:
$$(a \times b) \times c = a \times (b \times c)$$
3. **Multiplicative Identity:** $1$ is the unique neutral element:
$$a \times 1 = a$$
4. **Distributivity over Addition:**
$$a \times (b + c) = (a \times b) + (a \times c)$$

## 5. Worked Example: Mental Decomposition via Distributivity
To compute $7 \times 6$:
Decompose $6$ into $5 + 1$:
$$7 \times 6 = 7 \times (5 + 1) = (7 \times 5) + (7 \times 1) = 35 + 7 = 42$$
This principle allows any multi-digit multiplication to be split into place-value partial products.

## 6. Non-Example Trap
A common misconception is believing that "multiplication always makes numbers bigger."
- $a \times 1 = a$ (does not grow)
- $a \times 0 = 0$ (collapses to zero)
- For fractions $0 < q < 1$, multiplying by $q$ strictly shrinks the quantity.

## 7. Reflection
A rectangular array turns arithmetic into geometry: area and perimeter originate directly from arrays of unit squares.

## Additional learning activities (draft)
**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check
Count equal groups and add whole numbers. Notice that adding $4 + 4 + 4$ is three groups of four.

### Representation
Draw $3$ rows of $4$ dots. Count by rows: $4 + 4 + 4 = 12$. Turn the page to view $4$ columns of $3$: $3 + 3 + 3 + 3 = 12$. Both views represent the same $12$ dots.

### Complete a partly worked problem
Use the distributive property: $6 \times 8 = 6 \times (5 + 3) = (6 \times \underline{\hspace{1cm}}) + (6 \times \underline{\hspace{1cm}}) = 30 + \underline{\hspace{1cm}} = \underline{\hspace{1cm}}$.
Self-check: $6 \times (5 + 3) = (6 \times 5) + (6 \times 3) = 30 + 18 = 48$.

### Independent practice
1. Calculate $8 \times 7$ by breaking $7$ into $5 + 2$.
2. Explain why an array of $5$ rows and $9$ columns has the same total as an array of $9$ rows and $5$ columns.

### Explanation
Explain how the distributive property allows you to multiply a multi-digit number like $14$ by $3$ using place value.

### Transfer
A classroom has $6$ rows of desks with $5$ desks in each row. If $2$ new rows of $5$ desks are added, what is the new total number of desks?
Solution: $(6 + 2) \times 5 = 8 \times 5 = 40$ desks.

### Retrieval
Tomorrow write the multiplication facts for $6 \times 7$ and $8 \times 9$ using decomposition. Next week explain why $a \times (b + c) = a \times b + a \times c$ with a drawn grid.
