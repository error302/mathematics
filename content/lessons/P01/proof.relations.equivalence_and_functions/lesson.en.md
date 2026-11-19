# Equivalence Relations, Partitions & Injective Functions

## Intuition
An **equivalence relation** $\sim$ on a set $S$ generalizes equality by requiring three axioms: **reflexivity** ($x \sim x$), **symmetry** ($x \sim y \implies y \sim x$), and **transitivity** ($x \sim y \land y \sim z \implies x \sim z$).

The fundamental theorem of equivalence relations states that an equivalence relation $R \subseteq S \times S$ is the exact same information as a **partition** of $S$ into disjoint non-empty blocks (equivalence classes) $C_1, C_2, \dots, C_m$. Within each block $C_i$, every ordered pair in $C_i \times C_i$ belongs to $R$, contributing $|C_i|^2$ ordered pairs.

## Formal Definition
1. **Modular Equivalence Classes:** For $m \ge 2$, $a \equiv b \pmod m \iff m \mid (a - b)$. Every integer $a$ is congruent to a unique canonical representative $r \in \{0, 1, \dots, m - 1\}$ via $a = qm + r$.
2. **Block Decomposition of an Equivalence Relation:**
   $$R = \bigsqcup_{i=1}^{m} (C_i \times C_i) \implies |R| = \sum_{i=1}^{m} |C_i|^2$$
3. **Counting Injections ($f: A \to B$):** If $|A| = k \le |B| = n$, the number of one-to-one functions is:
   $$n(n - 1)\cdots(n - k + 1) = \frac{n!}{(n - k)!}$$

## Worked Example
**Problem:** Find the canonical representative $r \in \{0, \dots, 6\}$ of $[-19]_7$, and find the number of ordered pairs in an equivalence relation with class sizes $4$, $3$, and $2$.

1. **Modular class representative:** Since $-19 = (-3)(7) + 2$, $[-19]_7 = [2]_7$, so $r = 2$.
2. **Equivalence relation size:** $|R| = 4^2 + 3^2 + 2^2 = 16 + 9 + 4 = 29$.

## Common Misconceptions
- **Negative remainders:** In $\mathbb{Z}/m\mathbb{Z}$, canonical representatives always lie in $\{0, 1, \dots, m - 1\}$.
- **Unordered vs. ordered pairs in $R$:** Because $R \subseteq S \times S$ is reflexive and symmetric, each class $C_i$ contributes $|C_i|^2$ ordered pairs, not $\binom{|C_i|}{2}$.
