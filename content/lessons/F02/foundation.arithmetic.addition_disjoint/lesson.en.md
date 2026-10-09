# Addition: Union of Disjoint Sets

## 1. The Challenge
You have a box containing $5$ blue marbles, and another box containing $3$ yellow marbles. None of the marbles belong to both boxes.
When you pour both boxes into an empty bowl, how many marbles are in the bowl?

## 2. Predict Before Calculating
Does the order in which you pour the boxes change the total count?
Why does pouring $5$ then $3$ yield the exact same quantity as pouring $3$ then $5$?

## 3. Mathematical Definition: Set Theoretic Addition
Let $A$ and $B$ be two finite sets that are **disjoint**, meaning they share no elements in common:
$$A \cap B = \emptyset$$
The addition of their cardinalities $|A| = a$ and $|B| = b$ is defined as the cardinality of their set union:
$$a + b = |A \cup B|$$

## 4. Fundamental Axiomatic Properties
Addition over non-negative integers $\mathbb{N}_0$ satisfies two foundational axioms:
1. **Commutativity:** Order does not affect the sum:
$$a + b = b + a \quad \forall a, b \in \mathbb{N}_0$$
*(Follows from the set-theoretic symmetry of union: $A \cup B = B \cup A$)*
2. **Associativity:** Grouping does not affect the sum:
$$(a + b) + c = a + (b + c) \quad \forall a, b, c \in \mathbb{N}_0$$
3. **Additive Identity:** Zero is the neutral element:
$$a + 0 = a \quad (\text{since } A \cup \emptyset = A)$$

## 5. Worked Example: Making a Ten
Using associativity and commutativity, we can rearrange terms to simplify calculation:
$$7 + 8 = 7 + (3 + 5) = (7 + 3) + 5 = 10 + 5 = 15$$
We decomposed $8$ into $3 + 5$ specifically to complete the base-$10$ bundle with $7$.

## 6. Non-Example Trap
If the sets are **not** disjoint ($A \cap B \neq \emptyset$), naive addition double-counts elements!
$$|A \cup B| = |A| + |B| - |A \cap B|$$
Always verify that elements counted in additive models are distinct and non-overlapping.

## 7. Reflection
Addition is not merely an algorithm for manipulating digits; it is the fundamental mathematical operation of combining disjoint collections.
