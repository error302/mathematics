# Greatest Common Divisor and Least Common Multiple

## 1. The Challenge
Two lighthouses on the coast flash periodically: Lighthouse A flashes every $12$ seconds, and Lighthouse B flashes every $18$ seconds. If both lighthouses flash simultaneously at midnight:
1. After how many seconds will they flash together again?
2. If you want to cut a piece of wire of length $12\text{ cm}$ and another of length $18\text{ cm}$ into equal segments of the largest possible integer length with no remainder, how long should each segment be?

## 2. Predict Before Calculating
For any two positive integers $a$ and $b$, what is the exact mathematical relationship between their product $a \times b$ and the quantities $\gcd(a, b)$ and $\text{lcm}(a, b)$?
- Can they be multiplied together to recover the original numbers?

## 3. Mathematical Definitions: GCD and LCM
Let $a, b \in \mathbb{N}$:
1. **Greatest Common Divisor ($\gcd$):** The unique integer $g = \gcd(a, b)$ satisfying:
   - $g \mid a$ and $g \mid b$ ($g$ is a common divisor), and
   - For every $d \in \mathbb{N}$, if $d \mid a$ and $d \mid b$, then $d \le g$ (or equivalently $d \mid g$).
2. **Least Common Multiple ($\text{lcm}$):** The unique integer $m = \text{lcm}(a, b)$ satisfying:
   - $a \mid m$ and $b \mid m$ ($m$ is a common multiple), and
   - For every $c \in \mathbb{N}$, if $a \mid c$ and $b \mid c$, then $m \le c$ (or equivalently $m \mid c$).

## 4. Prime Factorization Perspective
Let the prime factorizations of $a$ and $b$ be:
$$a = \prod_{i} p_i^{\alpha_i}, \qquad b = \prod_{i} p_i^{\beta_i}$$
Then:
$$\gcd(a, b) = \prod_{i} p_i^{\min(\alpha_i, \beta_i)}$$
$$\operatorname{lcm}(a, b) = \prod_{i} p_i^{\max(\alpha_i, \beta_i)}$$

Because $\min(\alpha_i, \beta_i) + \max(\alpha_i, \beta_i) = \alpha_i + \beta_i$ for every prime power:
$$\gcd(a, b) \times \operatorname{lcm}(a, b) = a \times b$$

## 5. Worked Example: $\gcd(24, 36)$ and $\text{lcm}(24, 36)$
1. Prime factorize:
   - $24 = 2^3 \times 3^1$
   - $36 = 2^2 \times 3^2$
2. Compute $\gcd$:
   $$\gcd(24, 36) = 2^{\min(3, 2)} \times 3^{\min(1, 2)} = 2^2 \times 3^1 = 4 \times 3 = 12$$
3. Compute $\text{lcm}$:
   $$\operatorname{lcm}(24, 36) = 2^{\max(3, 2)} \times 3^{\max(1, 2)} = 2^3 \times 3^2 = 8 \times 9 = 72$$
4. Check identity:
   $$\gcd \times \operatorname{lcm} = 12 \times 72 = 864$$
   $$a \times b = 24 \times 36 = 864$$

## 6. Non-Example Trap
**Mistake:** Assuming that because "Greatest" is in the name of GCD, it must be larger than LCM.
- $\gcd(a, b) \le \min(a, b)$ because a divisor cannot exceed the number it divides.
- $\operatorname{lcm}(a, b) \ge \max(a, b)$ because a multiple cannot be smaller than the number multiplied.
- The greatest common divisor is a divisor (bounded above); the least common multiple is a multiple (bounded below).

## 7. Reflection
The GCD represents the prime intersection of two numbers; the LCM represents their prime union. In fractions, the GCD simplifies fractions to lowest terms, while the LCM provides the least common denominator.

## Additional learning activities (draft)
**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check
Determine prime factorizations of small two-digit integers.

### Representation
Draw a Venn diagram of prime factors: place shared factors in the overlap (their product is the GCD), and all factors across both sets form the LCM.

### Complete a partly worked problem
Find $\gcd(18, 30)$: $18 = 2 \times 3^2$, $30 = 2 \times 3 \times 5$. Shared primes with minimum powers are $2^{\underline{\hspace{0.5cm}}} \times 3^{\underline{\hspace{0.5cm}}} = \underline{\hspace{0.5cm}}$. So $\gcd(18, 30) = \underline{\hspace{0.5cm}}$.
*Self-check:* $2^1 \times 3^1 = 6$; $\gcd(18, 30) = 6$.

### Independent practice
1. Calculate $\gcd(45, 60)$ and $\operatorname{lcm}(45, 60)$.
2. Explain why if two numbers are relatively prime ($\gcd(a, b) = 1$), their LCM must equal their product $a \times b$.

### Explanation
Explain how Euclid's algorithm computes $\gcd(a, b)$ repeatedly using division with remainder without requiring prime factorization.

### Transfer
Two athletes run around a track: Runner 1 completes a lap in $40$ seconds, Runner 2 in $50$ seconds. If they start together, after how many seconds will they cross the start line at the exact same moment?

### Retrieval
Tomorrow practice finding the GCD of $28$ and $42$. Next week explain how finding common denominators in fractions is identical to finding the LCM of the denominators.
