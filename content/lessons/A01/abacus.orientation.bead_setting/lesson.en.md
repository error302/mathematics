# Single-Rod Bead Manipulation (0–9) & The Neutral State

## 1. The Challenge
A traditional Japanese Soroban has only $5$ beads per rod: $1$ upper bead and $4$ lower beads.
How can just $5$ beads represent all $10$ decimal digits ($0, 1, 2, 3, 4, 5, 6, 7, 8, 9$)?

## 2. Predict Before Practicing
Predict:
- How would you represent the number $6$?
- How would you represent the number $9$?

## 3. The Bi-Quinary Structure
The Soroban uses a **bi-quinary** representation:
- The single **upper bead** has value $5$.
- Each of the four **lower beads** has value $1$.
- Beads only count toward the represented value when moved **toward the reckoning beam**.

## 4. Setting Digits 0 through 9
| Digit | Upper Bead ($5$) | Lower Beads ($1$ each) | Arithmetic Formula |
|---|---|---|---|
| **0** | Up (away from beam) | Down (away from beam) | $0 + 0 = 0$ (Neutral) |
| **1** | Up | $1$ bead Up | $0 + 1 = 1$ |
| **2** | Up | $2$ beads Up | $0 + 2 = 2$ |
| **3** | Up | $3$ beads Up | $0 + 3 = 3$ |
| **4** | Up | $4$ beads Up | $0 + 4 = 4$ |
| **5** | Down (toward beam) | $0$ beads Up | $5 + 0 = 5$ |
| **6** | Down | $1$ bead Up | $5 + 1 = 6$ |
| **7** | Down | $2$ beads Up | $5 + 2 = 7$ |
| **8** | Down | $3$ beads Up | $5 + 3 = 8$ |
| **9** | Down | $4$ beads Up | $5 + 4 = 9$ |

## 5. Standard Fingering Rules
1. **Thumb (Lower Beads Up):** Always use the thumb to push lower beads up toward the beam (adding $1$ to $4$).
2. **Index Finger (Lower Beads Down):** Always use the index finger to pull lower beads down away from the beam (subtracting $1$ to $4$).
3. **Index Finger (Upper Bead):** Use the index finger for both moving the upper bead down (adding $5$) and moving it up (clearing $5$).
4. **Pinch Action (6, 7, 8, 9):** To enter numbers $6$ through $9$ in one motion, pinch the thumb and index finger together to move upper and lower beads toward the beam simultaneously.

## 6. The Neutral State (Zeroing)
Before every computation, the abacus must be cleared. Slanting the abacus slightly and running your index finger along the reckoning beam resets all rods to $0$.

## 7. Reflection
By combining a base-$5$ sub-quinary bead with base-$1$ unit beads, the Soroban reduces $9$ discrete physical items into just $5$, maximizing finger speed and visual recognition.


## Additional learning activities (draft)

**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check

There are four lower beads and one upper bead per rod. Legal engaged lower beads form a contiguous group touching the bar.

### Representation

Digits 0–4 use that many lower beads; digits 5–9 use the upper bead plus 0–4 lower beads. Thus 7 uses 5 + 2.

### Complete a partly worked problem

Complete the setting for 8: upper bead __; lower beads engaged __. For 4: upper __; lower __.

Self-check: 8: upper engaged, 3 lower. 4: upper away, 4 lower.

### Independent practice

1. Describe settings for 0, 5, and 9.

   Self-check: 0: all away; 5: upper only; 9: upper and all 4 lower.

2. Can a legal rod use five lower beads to show 5?

   Self-check: No. It has only four lower beads. Use the upper bead to show 5.

### Explain your reasoning

Explain how one upper/four lower beads provide exactly ten digit states without counting beads away from the bar.

### Transfer to a new situation

Read a physical instrument in which the upper bead is away and two lower beads touch the bar, then explain how keyboard controls can represent the same state.

Self-check: The rod digit is 2. The equivalent logical state is upper=false, lower=2. This describes value; it does not assess a physical finger technique.

### Retrieve after a delay

Tomorrow set/read 6, 2, 9 in random order. After a week, describe the change from 9 to 4.

The learner-facing presentation and these activities are stored in `lesson.json` beside this file.
