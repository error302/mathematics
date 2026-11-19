# Definite Integrals & The Fundamental Theorem of Calculus

## Intuition
The **Fundamental Theorem of Calculus (FTC)** unites the two branches of calculus—differential calculus (instantaneous rates of change) and integral calculus (accumulated area and net change)—as inverse processes. If $F'(x) = f(x)$, then the total accumulated signed area under $y = f(x)$ from $x = a$ to $x = b$ equals the net change $F(b) - F(a)$.

## Formal Definition
1. **Reverse Power Rule:** For any integer $n \ge 0$, an antiderivative of $x^n$ is $\frac{x^{n+1}}{n+1}$ since $\frac{d}{dx}\left(\frac{x^{n+1}}{n+1}\right) = x^n$.
2. **Fundamental Theorem of Calculus:** If $f$ is continuous on $[a, b]$ and $F'(x) = f(x)$, then:
   $$\int_a^b f(x)\,dx = \bigl[F(x)\bigr]_a^b = F(b) - F(a)$$
3. **Net Change Theorem:** For a quantity $s(t)$ with rate of change $v(t) = s'(t)$ and initial value $s(0)$,
   $$s(t) = s(0) + \int_0^t v(\tau)\,d\tau$$

## Worked Example
**Problem:** Evaluate $\int_0^2 (4x^2 + 3)\,dx$ exactly over $\mathbb{Q}$.

1. **Find an antiderivative $F(x)$:**
   $$F(x) = \frac{4}{3}x^3 + 3x$$
2. **Apply the Fundamental Theorem of Calculus on $[0, 2]$:**
   $$F(2) - F(0) = \left(\frac{4}{3}(2^3) + 3(2)\right) - 0 = \frac{32}{3} + 6 = \frac{32 + 18}{3} = \frac{50}{3}$$

## Common Misconceptions
- **Omitting division by $(n + 1)$:** Antidifferentiating $2ax$ gives $\frac{2a}{2}x^2 = ax^2$, and antidifferentiating $ax^2$ gives $\frac{a}{3}x^3$.
- **Forgetting $x$ on the constant term:** The antiderivative of a constant $c$ is $cx$, so at $x = b$ it contributes $cb$, not $c$.
- **Omitting initial position $s(0)$:** The definite integral of velocity gives net displacement $\Delta s$; add $s(0)$ to obtain final position $s(t)$.
