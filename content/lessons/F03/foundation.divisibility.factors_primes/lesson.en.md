# Divisibility, Factors, and Prime Numbers

## 1. The Challenge
If you have $12$ square tiles, you can arrange them into several different rectangles: $1 \times 12$, $2 \times 6$, and $3 \times 4$. But if you have $13$ square tiles, the only rectangle you can possibly form is a single long strip: $1 \times 13$. What makes numbers like $12$ structurally different from numbers like $13$?

## 2. Predict Before Calculating
Is the number $1$ considered a prime number?
- If $1$ were prime, would prime factorization still be unique? (Think about $6 = 2 \times 3 = 1 \times 2 \times 3 = 1 \times 1 \times 2 \times 3 \dots$).

## 3. Mathematical Foundations: Divisibility and Primes
Let $a, b \in \mathbb{Z}$ with $a \neq 0$:
1. **Divisibility:** We say that $a$ divides $b$ (written $a \mid b$) if there exists an integer $k \in \mathbb{Z}$ such that:
$$b = a \cdot k$$
Here $a$ is called a **divisor** or **factor** of $b$, and $b$ is a **multiple** of $a$.
2. **Proper Divisors:** A proper divisor of a positive integer $n$ is any positive divisor $d \mid n$ such that $1 \le d < n$.
3. **Prime Numbers:** An integer $p \in \mathbb{N}$ is **prime** if $p > 1$ and its only positive divisors are $1$ and $p$.
4. **Composite Numbers:** An integer $n \in \mathbb{N}$ is **composite** if $n > 1$ and it has at least one proper divisor strictly greater than $1$.
5. **The Fundamental Theorem of Arithmetic:** Every integer $n \ge 2$ can be factored into a product of primes:
$$n = p_1^{e_1} p_2^{e_2} \cdots p_k^{e_k}$$
This factorization is **unique** up to the order of factors.

## 4. Systematic Factor Finding: The Square Root Bound
To find all factors of $n$, it is sufficient to test test candidates $d$ up to $\sqrt{n}$:
- If $d \mid n$, then $n / d$ is also an integer divisor.
- If $n$ has a divisor larger than $\sqrt{n}$, its paired divisor must be strictly smaller than $\sqrt{n}$.
- Therefore, testing candidates up to $\lfloor \sqrt{n} \rfloor$ guarantees finding every factor pair without exhaustive trial division up to $n$.

## 5. Worked Example: Finding Factors of $36$
1. $\sqrt{36} = 6$. We only need to check integers $1, 2, 3, 4, 5, 6$.
2. Check candidate divisors:
   - $36 \div 1 = 36 \implies (1, 36)$
   - $36 \div 2 = 18 \implies (2, 18)$
   - $36 \div 3 = 12 \implies (3, 12)$
   - $36 \div 4 = 9 \implies (4, 9)$
   - $36 \div 5 = 7.2$ (not an integer)
   - $36 \div 6 = 6 \implies (6, 6)$
3. List of all distinct positive factors:
$$\{1, 2, 3, 4, 6, 9, 12, 18, 36\}$$
4. The greatest proper divisor (the largest factor strictly smaller than $36$) is $18$.

## 6. Non-Example Trap
**Mistake 1:** Assuming all odd numbers are prime.
- $9 = 3 \times 3$, $15 = 3 \times 5$, $21 = 3 \times 7$, $25 = 5 \times 5$ are all odd, yet all are composite.

**Mistake 2:** Classifying $1$ as prime.
- $1$ has only one positive divisor (itself), not two. Defining $1$ as prime would destroy the uniqueness of prime factorization.

## 7. Reflection
Prime numbers are the fundamental multiplicative building blocks. Just as chemical elements compose molecules, prime numbers generate all integers through multiplication.

## Additional learning activities (draft)
**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check
Perform whole-number division and verify whether a division leaves a zero remainder.

### Representation
Take $12$ unit squares and build rectangles: $1 \times 12$, $2 \times 6$, $3 \times 4$. Then try with $7$ unit squares: only $1 \times 7$ is possible, showing that $7$ is prime.

### Complete a partly worked problem
Find the factors of $20$: $1 \times \underline{\hspace{1cm}} = 20$, $2 \times \underline{\hspace{1cm}} = 20$, $4 \times \underline{\hspace{1cm}} = 20$. The factors are $\{1, 2, 4, 5, 10, 20\}$. The greatest proper divisor is $\underline{\hspace{1cm}}$.
*Self-check:* $1 \times 20, 2 \times 10, 4 \times 5$; greatest proper divisor is $10$.

### Independent practice
1. List all factors of $28$ and state its greatest proper divisor.
2. Decompose $60$ into its unique prime factorization.

### Explanation
Explain why if a number $n$ has no prime divisors less than or equal to $\sqrt{n}$, it is guaranteed to be prime.

### Transfer
A baker has $48$ muffins and wants to pack them into identical gift boxes with no muffins left over. If each box must contain more than $1$ muffin and fewer than $48$ muffins, what box sizes are possible?

### Retrieval
Tomorrow list the prime numbers between $1$ and $30$. Next week explain how prime factorization simplifies fraction reduction.
