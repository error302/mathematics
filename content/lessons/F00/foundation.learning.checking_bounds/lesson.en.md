# Checking Answers: Bounds, Invariants, and Counterexamples

One of the sharpest differences between routine mechanical calculating and mathematical thinking is how errors are caught.

When learners make an error, their default reaction is often to repeat the exact same calculation, often repeating the exact same cognitive blind spot. Mathematicians, by contrast, verify statements using **orthogonal checks**:
1. **Parity and Invariants** (Could this number possibly end in an even digit?)
2. **Order-of-Magnitude Bounds** (Is the result between $2{,}000$ and $3{,}000$?)
3. **Inverse Operations** (Does the quotient times the divisor equal the dividend?)
4. **Counterexamples** (Can we find a single case where the rule breaks?)

## 1. Parity Invariants

In integer arithmetic, numbers are partitioned into two disjoint classes:
- **Even**: $2k$ for some integer $k$
- **Odd**: $2k + 1$ for some integer $k$

The parity algebra is rigid:
$$\begin{aligned}
\text{Odd} \times \text{Odd} &= \text{Odd} \\
\text{Odd} \times \text{Even} &= \text{Even} \\
\text{Even} \times \text{Even} &= \text{Even} \\
\text{Odd} + \text{Odd} &= \text{Even} \\
\text{Odd} + \text{Even} &= \text{Odd}
\end{aligned}$$

If someone calculates $47 \times 53$ and writes $2{,}492$, you do not need a pencil or paper to reject it: an odd number times an odd number can never be even!

## 2. Order of Magnitude Bounds

Before computing $47 \times 53$, establish lower and upper bounds:
- **Lower bound**: $40 \times 50 = 2{,}000$
- **Upper bound**: $50 \times 60 = 3{,}000$

Any answer outside $[2{,}000, 3{,}000]$ is immediately exposed as wrong.

## 3. The Power of a Counterexample

In empirical sciences, observing a phenomenon $1{,}000$ times builds strong statistical confidence. In pure mathematics, observation alone never constitutes proof.

Consider the quadratic formula:
$$P(n) = n^2 - n + 11$$

Testing small values:
- $n = 1 \implies 1 - 1 + 11 = 11$ (prime)
- $n = 2 \implies 4 - 2 + 11 = 13$ (prime)
- $n = 3 \implies 9 - 3 + 11 = 17$ (prime)
- $n = 4 \implies 16 - 4 + 11 = 23$ (prime)
- $n = 5 \implies 25 - 5 + 11 = 31$ (prime)

Does this mean $P(n)$ is always prime?
Let $n = 11$:
$$P(11) = 11^2 - 11 + 11 = 11^2 = 121 = 11 \times 11$$
$121$ is composite!

A single counterexample ($n = 11$) completely and irrefutably destroys the conjecture.
