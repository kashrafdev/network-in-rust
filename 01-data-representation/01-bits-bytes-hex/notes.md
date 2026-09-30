Computers do not send integers over the network they send bits

=> So netwroking needs us to:

1. decide how an integer is converted into a byte (Serialization)
2. send this data to the receiver (Transimation)
3. how the receiver convert it from bytes to integer (Deserialization)

```
Human representation
        ↓
       25

Binary representation
        ↓
      11001

32-bit representation
        ↓
00000000 00000000 00000000 00011001
```

```
 INTEGER
    │
    │ serialization
    ▼
  BYTES
    │
    │ transmitted
    ▼
  BYTES
    │
    │ deserialization
    ▼
 INTEGER
```

===========================================================

===========================================================

=> The bytes iteself does not say what it means the protocol defines their meanings

For example, we could design our own protocol:

```
Byte 0: message type
Byte 1-4: user ID
Byte 5-8: timestamp
```

Now if we receive:

```
01 00 00 00 2A ...
```

we know:

```
01
↓
message type

00 00 00 2A
↓
user ID = 42
```

This is why networking is heavily about representing structured information as bytes.
Which makes it much easier for the receiver to parse

===================================================

==================================================

| Rust type | Bits | Bytes |
| --------- | ---: | ----: |
| `u8`      |    8 |     1 |
| `u16`     |   16 |     2 |
| `u32`     |   32 |     4 |
| `u64`     |   64 |     8 |
| `u128`    |  128 |    16 |

```
1 byte
00011001

4 bytes
00000000 00000000 00000000 00011001

8 bytes
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00011001
```

=======================================================

=======================================================

Why we use hexadedcimal and why don't networking programmers just use binary?

Because binary is painful to read.

Imagine looking at: `11001010111100010010001101010101` That's 32 bits.

Hexadecimal compresses every `4 bits` into `one digit`:

```
1100 1010 1111 0001 0010 0011 0101 0101
  C    A    F    1    2    3    5    5
```

So: `11001010111100010010001101010101` becomes: `CAF12355` Much easier.

And because:

```
1 hex digit = 4 bits
2 hex digits = 8 bits = 1 byte
```

hex maps beautifully onto bytes.

For example:

Binary: `00000000 00000000 00000000 00101010`
Hex: `00 00 00 2A`
Decimal: `42`

So when you're using tools such as Wireshark later, you'll constantly see things like:

```
45 00 00 3C 1C 46 40 00 ...
```

Those are bytes represented using hexadecimal.

==============================================================

==============================================================

Integer representation VS Serialization

integer-representation: how the integer itsellf is represented in bytes
serialization: how my data structure `struct {...}` is converted into bytes

```
INTEGER REPRESENTATION
       ↓
How does 42 become bytes?

SERIALIZATION
      ↓
How do my program's data structures become bytes?

PROTOCOL
      ↓
What do those bytes mean?
```

========================================================

=======================================================

When learning networking:

- stop thinking: `I'm sending an integer.`
- Instead think: `I'm sending a sequence of bytes, and my protocol defines how those bytes should be interpreted.`

========================================================

=======================================================

# DECIMAL -> BINARY

Repeated division by 2
Take: `13`
Divide by 2 repeatedly and record the remainder:

```
13 ÷ 2 = 6 remainder 1
 6 ÷ 2 = 3 remainder 0
 3 ÷ 2 = 1 remainder 1
 1 ÷ 2 = 0 remainder 1
```

Now read the remainders from bottom to top: `1101`

Therefore: `13₁₀ = 1101₂`

The rule

```
while number > 0:
    divide by 2
    save remainder
    continue with quotient
```

read remainders backwards

# BINARY -> DECIMAL

use the `POWER OF 2`

```
  1    1    0    1
  ↓    ↓    ↓    ↓
 2³   2²   2¹   2⁰

(1×8 + 1×4 + 0×2 + 1×1) = (8 + 4 + 0 + 1) = (13)
```

Rule => `bit × 2^position`

# HEX

in hexdecimal there is 16 symbol `0 1 2 3 4 5 6 7 8 9 A B C D E F`
most important mapping is:

```
A = 10
B = 11
C = 12
D = 13
E = 14
F = 15
```

# DECIMAL -> HEX

Use repeated division by 16.

```
500 ÷ 16 = 31 remainder 4
31 ÷ 16  = 1  remainder 15
1 ÷ 16   = 0  remainder 1
```

Read it backward: `1F4`

# HEX -> DECIMAL

Use power of 16

```
  2     A
  ↓     ↓
 16¹   16⁰

(2 × 16¹ + 10 × 16⁰) = (2 × 16 + 10 × 1) = (32 + 10) = (42)
```

# BINARY <--> HEX

This is is important for netwroking

| Binary | Hex |
| ------ | --- |
| `0000` | `0` |
| `0001` | `1` |
| `0010` | `2` |
| `0011` | `3` |
| `0100` | `4` |
| `0101` | `5` |
| `0110` | `6` |
| `0111` | `7` |
| `1000` | `8` |
| `1001` | `9` |
| `1010` | `A` |
| `1011` | `B` |
| `1100` | `C` |
| `1101` | `D` |
| `1110` | `E` |
| `1111` | `F` |

Example

```
11010110 --split-> 1101 0110 --convert-> D6

A7 --convert-> 1010 0111  (Note: Never remove the leading zero)
```

Relation between Binary and Hex

```
1 hex digit = 4 bits

2 hex digits = 8 bits = 1 byte

4 hex digits = 16 bits = 2 bytes

8 hex digits = 32 bits = 4 bytes

16 hex digits = 64 bits = 8 bytes
```

so `u32` -> represented by `32 binary bits` -> or a `8 hex digit`

Rule: `n hex digit = (bits / 8) * 2`

=================================================================

=================================================================

# Representing Signed Integers

## What is a negative

if we have a number x so it's negative is any number that if we added to it give us `zero`
so `x + -x = 0`

## Complements

we need to know thee two types of complements (radix, diamenshional) in different number systems

### Decimal system

1. 9's complement: it's the number x that we add to our number a so that we reach 9 in all avilable position
   a -> |5| |7| |9| + x = |9| |9| |9|, so x must be |4| |2| |0|
   so `420` is the number that if we added to a we reach the maximum possible value our three boxes can reach

2. 10's complement: it's the number x that we add to our number a so that we reach 10 in all avilable position
   since that each box can only represent one number so 10 is represented as 0 and carry one to the next box
   so it will be like this |1| |0| |0| |0|, where `|1|` is an additional box
   a -> |5| |7| |9| + x = |1| |0| |0| |0|, so x must be |4| |2| |1|
   so we can say that `10's complement` is 9's complement + 1

### Binary system

1. 1's complement: it's the number x that we add to our number a so that we reach 1 in all avilable position
   a -> |1| |0| |1| + 1's complement = |1| |1| |1|, so 1's complement must be |0| |1| |0|
   so `010` is the number that if we added to a we reach the maximum possible value our three boxes can reach
   which is the `invertion` of `101` it looks like if we just inverted the a since subtraction from ones never borrow
   so it seems like if we inverted a
   the maximum value can be reached by n boxes is `(2^n) - 1`
   so 1's complement of `a` is `(2^n) - 1` - `a`

2. 2's complement: it's the number x that we add to our number a so that we reach 2 in all avilable position
   since in binary system `1` is thee maximum number we can reach + each box can only represent one number so
   2 is represented as 10 and carry one to the next box
   so it will be like this |1| |0| |0| |0|, where `|1|` is an additional box
   a -> |1| |0| |1| + x = |1| |0| |0| |0|, so x must be |0| |1| |1|
   so we can say that `2's complement` is 1's complement + 1
   since 1's complement is `(2^n) - 1 - a` or only the `inversion` of the number
   so 2's complement is `((2^n)- 1 - a) + 1` = `2^n - a` or `inversion` + 1

## How complements help in representing negatives in binary

Step 1: What is a negative number?
−x is the number that, added to x, gives 0. That's the only thing we need from it.

Step 2: Overflow gives us a zero for free.
With 3 boxes, the numbers wrap around.
If you go past 111, you get 1000, but there are only 3 boxes, so the carry is lost and you're left with 000.

111 + 001 = 1000 → drop the carry → 000

So in 3 boxes, adding 1 to 111 gives 0. That means 111 behaves like −1.

Step 3: The 2's complement is the number that makes x overflow to 0.
By definition, x + (2's complement of x) = 2ⁿ = 1000, and the carry is dropped, leaving 000.
So the 2's complement of x does exactly what −x is supposed to do.
We simply _decide_ to use it as the pattern for −x.

**Example (3 boxes), x = 3 = 011:**

- 1's complement: 100
- Add 1: 101
- Check: 011 + 101 = 1000 → drop the carry → 000 ✓

So 101 represe−3−3*Step 4: Why it works for calculations.s.**
Compute 5 − 3 as 5 + (−3):
101 + 101 = 1010 → drop the carry → 010 = 2 ✓

The adder just adds. It doesn't know or care that one number is "negative." The wraparound makes the answer come out righStep 5: The full table (3 boxes).).**

| Pattern | Value |
| ------- | ----- |
| 000     | 0     |
| 001     | 1     |
| 010     | 2     |
| 011     | 3     |
| 100     | −4    |
| 101     | −3    |
| 110     | −2    |
| 111     | −1    |

We ca see a Patterns:

1. patterns starting with 0 are non-negative,
   so is there is 3 bits the non-negative are represented by only 2 bits sincec the first is always 0
   so if we have an n-bits non-negative are represented by n-1 bits
   since maximum number can be represneted by n-bits is `(2^n)`
   so the max non-negative number can be represented by n-bits is `2^(n-1) - 1`
   we subtract `1` bec we start counting from `0`
   and used `n-1` beacuse the first bit is not used in representing the number

2. patterns starting with 1 are negative.
   so is there is 3 bits the negative are represented by 2 bits since the first is always 1
   so if we have an n-bits negative are represented by n-1 bits
   so the max negative number can be represented by n-bits is `-2^(n-1)`
   and used `n-1` beacuse the first bit is not used in representing the number
   and we did not subtract 1 since counting start from -1 not 0

3. so the range is `-2^(n-1) : 2^(n-1) - 1`

That's why the leftmost bit acts as the sign biIn one sentence:
the 2's complement of x is the pattern that adds to x to give zero after overflow,
and "adds to x to give zero" is exactly what a negative number is.
