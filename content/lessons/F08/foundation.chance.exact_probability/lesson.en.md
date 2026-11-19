# Exact Probability, Complements, and Compound Draws

## 1. Classical Probability Over $\mathbb{Q}$

When an experiment has a finite sample space $\Omega$ of equally likely outcomes, the probability of an event $E \subseteq \Omega$ is the exact rational ratio:

$$P(E) = \frac{|E|}{|\Omega|}$$

Because $0 \le |E| \le |\Omega|$, every probability satisfies the fundamental bounds:

$$0 \le P(E) \le 1$$

> **Probability vs. Odds:** If a bag has $3$ red tokens and $5$ blue tokens, the total sample space has $|\Omega| = 3 + 5 = 8$ tokens. The probability of drawing red is $\frac{3}{8}$, **not** $\frac{3}{5}$.

## 2. The Complement Rule

Every outcome in $\Omega$ either belongs to $E$ or belongs to its complement $E^c$ ("not $E$"):

$$|E| + |E^c| = |\Omega| \implies P(E) + P(\text{not } E) = 1$$

Therefore:

$$P(\text{not } E) = 1 - P(E) = \frac{|\Omega| - |E|}{|\Omega|}$$

## 3. Sequential Draws Without Replacement

Suppose a pouch contains $g$ gold coins and $s$ silver coins, for a total of $N = g + s$ coins. If we draw two coins **without replacement**:
1. On the first draw, $P(\text{1st is gold}) = \frac{g}{N}$.
2. Given that the first coin was gold, there are now $g - 1$ gold coins left among $N - 1$ total remaining coins: $P(\text{2nd is gold} \mid \text{1st is gold}) = \frac{g - 1}{N - 1}$.

By the multiplication rule for dependent events:

$$P(\text{both gold}) = \frac{g}{N} \times \frac{g - 1}{N - 1}$$
