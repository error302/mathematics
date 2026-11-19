# Comparing Fractions: The Role of the Common Denominator

## 1. The Challenge
Suppose you have two measuring beakers of equal capacity. One is filled to $\frac{3}{4}$ of its volume, and the other to $\frac{5}{8}$. 

Which beaker contains more liquid?

## 2. Predict Before Calculating
Make a prediction:
- Is $\frac{3}{4}$ smaller than, equal to, or greater than $\frac{5}{8}$?
*(It is completely fine to be uncertain — mathematical thinking begins by testing our intuition!)*

## 3. Definition: What is a Rational Fraction?
A positive rational fraction $\frac{a}{b}$ (where $a, b \in \mathbb{Z}^+$ and $b \neq 0$) denotes $a$ parts of a whole that has been partitioned into $b$ equal pieces.
- The **denominator** $b$ specifies the **size** of the parts (larger denominator means each individual part is smaller).
- The **numerator** $a$ counts **how many** of those parts are selected.

> **Crucial Axiom:** You cannot compare two fractions merely by looking at which numerator is larger, because the pieces are not of equal size!

## 4. Visual Representation on the Unit Interval $[0, 1]$
Consider the number line segment from $0$ to $1$:
- Dividing the segment into $4$ equal intervals creates steps of length $\frac{1}{4}$. The point $\frac{3}{4}$ lies at the third tick mark ($0.75$).
- Dividing the segment into $8$ equal intervals creates steps of length $\frac{1}{8}$. Notice that two eighth-steps equal one fourth-step ($\frac{2}{8} = \frac{1}{4}$).

## 5. Worked Example: Finding a Common Denominator
To compare $\frac{3}{4}$ and $\frac{5}{8}$ with certainty, we express both using the same unit partition (a common denominator).

**Step 1:** The least common multiple of $4$ and $8$ is $8$.
**Step 2:** Scale $\frac{3}{4}$ by multiplying numerator and denominator by $2$:
$$\frac{3 \times 2}{4 \times 2} = \frac{6}{8}$$
**Step 3:** Now compare the numerators:
$$\frac{6}{8} > \frac{5}{8} \implies \frac{3}{4} > \frac{5}{8}$$

Since $6$ eighths is strictly greater than $5$ eighths, $\frac{3}{4}$ is greater than $\frac{5}{8}$.

## 6. Non-Example and Misconception Challenge
**The Error:** "Since $5 > 3$ and $8 > 4$, therefore $\frac{5}{8} > \frac{3}{4}$."
**Why this is invalid:**
$5$ eighths may have more pieces than $3$ fourths, but each eighth is half the size of each fourth ($\frac{1}{8} < \frac{1}{4}$). When the unit sizes differ, numerator comparisons alone are meaningless.

## 7. Strategy Reflection
Whenever you need to compare fractions $\frac{a}{b}$ and $\frac{c}{d}$:
1. Find a common denominator $D = \text{lcm}(b, d)$.
2. Convert both fractions: $\frac{a \cdot (D/b)}{D}$ and $\frac{c \cdot (D/d)}{D}$.
3. Compare the resulting numerators.


## Additional learning activities (draft)

**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check

A fraction compares equal-sized parts of the same whole. Explain why 1/2 and 2/4 mark the same point before comparing fractions.

### Representation

Draw two equal-length strips. Split the first into 4 equal parts and shade 3; split the second into 8 and shade 5. Splitting each fourth in half produces 6 shaded eighths without changing the first strip.

### Complete a partly worked problem

Complete: 2/3 = __/12 and 3/4 = __/12. Which is larger?

Self-check: 2/3 = 8/12 and 3/4 = 9/12, so 2/3 < 3/4. Every piece now has the same size.

### Independent practice

1. Compare 5/6 and 7/9; include a reason.

   Self-check: 5/6 = 15/18 and 7/9 = 14/18, so 5/6 > 7/9.

2. Compare 4/6 and 2/3. What case did the first worked example not include?

   Self-check: They are equal: 4/6 simplifies to 2/3. Comparison must include equality, not just less/greater.

### Explain your reasoning

Explain why multiplying both numerator and denominator by the same positive integer preserves the fraction. Your explanation should refer to subdividing parts or multiplying by 1.

### Transfer to a new situation

Bottle A contains 3/4 of a 1-litre bottle. Bottle B contains 5/8 of a 2-litre bottle. Which holds more liquid? Why does the original comparison no longer decide?

Self-check: A contains 3/4 litre; B contains 10/8 = 5/4 litres. B holds more. Fraction comparisons describe equal wholes only when the whole units match.

### Retrieve after a delay

Tomorrow, without reopening the example, compare 7/12 and 3/5. A week later, compare 5/8 and 2/3 and explain why your method is valid.

The learner-facing presentation and these activities are stored in `lesson.json` beside this file.
