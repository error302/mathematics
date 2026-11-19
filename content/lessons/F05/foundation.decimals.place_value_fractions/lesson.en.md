# Decimals as Fractions with Powers-of-Ten Denominators

**Course:** F05 — Decimals, Percentages, and Proportional Reasoning  
**Status:** Draft (`review_status: independent_review_pending`)  
**Domain:** Rational Numbers $\mathbb{Q}$

---

## 1. The Core Meaning of the Decimal Point

In our Hindu-Arabic base-ten positional system, each step to the left multiplies a place value by ten:
$$\dots, 1000, 100, 10, 1$$

What happens when we move to the right of the ones place? Symmetrically, each step to the right divides the place value by ten:
$$\frac{1}{10}, \frac{1}{100}, \frac{1}{1000}, \dots$$

The decimal point is merely a punctuation mark separating whole unit values from fractional powers-of-ten parts:
$$a_0 . d_1 d_2 d_3 \dots = a_0 + \frac{d_1}{10} + \frac{d_2}{100} + \frac{d_3}{1000} + \dots$$

---

## 2. Converting Finite Decimals to Canonical Fractions

To convert any finite decimal into an exact rational number $p/q$:
1. Count the number of decimal digits $k$ after the decimal point.
2. The denominator is $10^k$.
3. The numerator is the integer formed by removing the decimal point.
4. Simplify by dividing by the greatest common divisor $\gcd(\text{numerator}, 10^k)$.

### Worked Example:
Convert $0.35$ to a canonical fraction:
$$0.35 = \frac{35}{100}$$
Now find $\gcd(35, 100) = 5$:
$$\frac{35 \div 5}{100 \div 5} = \frac{7}{20}$$

---

## 3. Which Fractions Terminate in Base 10?

A rational fraction in lowest terms $\frac{a}{b}$ terminates as a finite decimal if and only if the prime factorization of the denominator $b$ has the form:
$$b = 2^m \cdot 5^n \quad (m, n \ge 0)$$

Because $10 = 2 \times 5$, only multiples of powers of $2$ and $5$ can be scaled up to a power of $10$ by an integer factor:
- $\frac{1}{4} = \frac{1 \times 25}{4 \times 25} = \frac{25}{100} = 0.25$
- $\frac{3}{8} = \frac{3 \times 125}{8 \times 125} = \frac{375}{1000} = 0.375$
- $\frac{1}{3} = 0.333\dots$ never terminates because $3$ cannot divide any power of $10$.

---

## 4. Misconception Alert: Length vs Magnitude

A common error is to judge decimal magnitude by the number of digits after the decimal point. For instance, claiming:
$$0.35 > 0.7 \quad \text{(FALSE!)}$$
Looking at place value columns:
- $0.7 = \frac{7}{10} = \frac{70}{100}$ (70 hundredths)
- $0.35 = \frac{35}{100}$ (35 hundredths)
Since $70 > 35$, we have $0.7 > 0.35$.
Aligning decimal points restores clear place-value comparison.
