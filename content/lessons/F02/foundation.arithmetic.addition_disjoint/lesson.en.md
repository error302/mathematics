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


## Additional learning activities (draft)

**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check

Count a collection once. Two boxes of separate objects do not overlap; two lists can refer to the same objects.

### Representation

Make a row of 7 counters and another row of 8. Move 3 counters from the second row to complete a row of ten; 5 remain. Total: 10 + 5 = 15.

### Complete a partly worked problem

Use make-a-ten: 8 + 7 = 8 + (__ + __) = 10 + __ = __.

Self-check: Split 7 into 2 + 5: 8 + (2 + 5) = 10 + 5 = 15.

### Independent practice

1. Compute 9 + 6 using a regrouped expression.

   Self-check: 9 + (1 + 5) = 10 + 5 = 15.

2. Explain why 4 + 3 + 6 can be grouped as (4 + 6) + 3.

   Self-check: Addition of these whole numbers is commutative and associative, so the sum is 10 + 3 = 13.

### Explain your reasoning

State the difference between changing the order of addends and changing how they are grouped. Illustrate each with numbers.

### Transfer to a new situation

A club has 5 chess players and 4 football players; 2 play both. How many distinct people are in these lists?

Self-check: 5 + 4 - 2 = 7. Adding the lists without removing the shared people double-counts them.

### Retrieve after a delay

Tomorrow solve 8 + 6 by making ten. A week later create an example where adding two list sizes overcounts people.

The learner-facing presentation and these activities are stored in `lesson.json` beside this file.
