# Unit Fractions: Partitioning the Whole

## 1. The Challenge
Imagine a ribbon exactly $1$ meter long. You need to divide this ribbon equally among $5$ friends so that every person receives an identical length.

What exact portion of the original ribbon does each friend receive?

## 2. Predict Before Calculating
As the number of friends sharing the ribbon increases (say from $5$ to $10$):
- Does each person's piece become longer or shorter?
- What happens to the fraction $\frac{1}{n}$ as $n$ grows larger?

## 3. Definition: Unit Fraction
For any positive integer $n \in \mathbb{Z}^+$:
A **unit fraction** is a rational number of the form:
$$\frac{1}{n}$$
where $1$ is the numerator and $n$ is the denominator. It represents the measure of exactly one part when a whole unit is divided into $n$ congruent (equal-measure) sub-parts.

## 4. Visual Representation
Consider the unit interval $[0, 1]$ partitioned into $n = 5$ segments:
- Segment 1: $[0, \frac{1}{5}]$
- Segment 2: $[\frac{1}{5}, \frac{2}{5}]$
- Segment 3: $[\frac{2}{5}, \frac{3}{5}]$
- Segment 4: $[\frac{3}{5}, \frac{4}{5}]$
- Segment 5: $[\frac{4}{5}, 1]$

Notice that all $5$ segments have identical length $\frac{1}{5}$. Together, their sum restores the unit:
$$\sum_{k=1}^5 \frac{1}{5} = \frac{5}{5} = 1$$

## 5. The Inverse Magnitude Invariant
As the denominator $n$ increases, the size of each piece decreases:
$$n_1 < n_2 \implies \frac{1}{n_1} > \frac{1}{n_2}$$
For example:
$$\frac{1}{2} > \frac{1}{3} > \frac{1}{4} > \frac{1}{10} > \frac{1}{100}$$

## 6. Common Misconception Trap
**The Error:** "$\frac{1}{8}$ is larger than $\frac{1}{4}$ because $8 > 4$."
**The Mathematical Truth:** The denominator indicates division, not multiplication! Dividing a pizza into $8$ slices produces smaller slices than dividing the same pizza into $4$ slices.

## 7. Synthesis and Reflection
Every proper fraction $\frac{m}{n}$ is simply the sum of $m$ copies of the unit fraction $\frac{1}{n}$:
$$\frac{m}{n} = \underbrace{\frac{1}{n} + \frac{1}{n} + \dots + \frac{1}{n}}_{m \text{ times}}$$
Unit fractions are the fundamental atoms of rational numbers.


## Additional learning activities (draft)

**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check

A whole must be identified and the parts must be equal. Can four unequal ribbon pieces each be called one fourth?

### Representation

Mark 0 and 1 on a strip. Put equally spaced marks at 1/5, 2/5, 3/5, and 4/5. One step has length 1/5; the endpoint is 5/5 = 1.

### Complete a partly worked problem

A metre is divided into 6 equal pieces. One piece is __ of a metre. Four pieces are __ of a metre.

Self-check: One piece is 1/6 metre; four pieces are 4/6 = 2/3 metre.

### Independent practice

1. Which is larger: 1/7 or 1/4, using the same whole?

   Self-check: 1/4 is larger because the whole is divided into fewer equal pieces.

2. How many copies of 1/8 make one whole?

   Self-check: 8 copies; 8 × 1/8 = 1.

### Explain your reasoning

Explain why denominators must be positive here, why 1/0 is undefined, and why equal-sized parts matter. You do not need a calculus argument about limits.

### Transfer to a new situation

A 2-metre ribbon is divided into 8 equal pieces. What fraction of the ribbon is one piece, and what length is it?

Self-check: One piece is 1/8 of the ribbon but 2/8 = 1/4 metre long. Fraction of the whole and measured length are different quantities.

### Retrieve after a delay

Tomorrow sketch a number line and locate 1/3. After a week, explain why 1/9 < 1/6 without a calculator.

The learner-facing presentation and these activities are stored in `lesson.json` beside this file.
