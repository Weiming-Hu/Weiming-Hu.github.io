---
layout: post
title: "Understanding Value Swapping in Python"
description: "How Python swaps values might surprise you"
tags: Python
giscus_comments: true
date: 2021-08-05
related_posts: false
---

## Introduction

[Python](https://www.python.org/) has some very nice [syntax sugars](https://en.wikipedia.org/wiki/Syntactic_sugar) that really come in handy when we program. One such example is swapping values. It is probably one of the first things you learn and this feature is always touted to show Python's convenience and readability. The idea is that, instead of relying on a third variable, you can directly swap two values. Nice and easy!

But **there are caveats**. And that is why I wrote this article to explain what happens under the hood.

Try the following questions and see whether you are already familiar with what I'm going show you. Have fun!

```python
# Level 1: The swap feature that comes in handy
a, b = 1, 2
a, b = b, a
print('a: {}; b: {}'.format(a, b))

# Level 2: Yes. You can do more than just two.
a, b, c, d = 1, 2, 3, 4
a, b, c, d = d, c, b, a
print('a: {}; b: {}; c: {}; d: {}'.format(a, b, c, d))

# Level 3: It also works for lists.
arr = [3, 2, 1, 0]
arr[0], arr[3] = arr[3], arr[0]
print('arr: {}'.format(arr))

# Level 4: Getting fancier
arr = [3, 2, 1, 0]
arr[arr[0]], arr[0] = arr[0], arr[arr[0]]
print('arr: {}'.format(arr))

# Level 5: This might surprise you.
arr = [3, 2, 1, 0]
arr[0], arr[arr[0]] = arr[arr[0]], arr[0]
print('arr: {}'.format(arr))

# Level 6
# Explain how they are different @_@

###########
# Answers #
###########

# 1. a = 2, b = 1
# 2. a = 4, b = 3, c = 2, d = 1
# 3. arr = [0, 2, 1, 3]
# 4. arr = [0, 2, 1, 3]
# 5. arr = [3, 2, 1, 0]
# 6. Please read the rest of the post.
```

Code for this post can be downloaded from [here](https://weiming-hu.github.io/assets/python/2021-08-05-understand-swap.py).

## Preparation

I am glad that you decide to read on. It is recommended to have Python `3.x` running. I am using Python `3.9.5`.

We are going to read some [assembly code](https://en.wikipedia.org/wiki/Assembly_language). But **you do not need to have any knowledge in assembly language**. It is simply a way here for us to see how exactly Python is manipulating machine memory.

To compile Python into assembly, we need the native Python module, `dis`. [Here](https://stackoverflow.com/a/47529318) is a pretty good explanation of the output produced by `dis.dis`.

[Here](https://docs.python.org/3/library/dis.html#python-bytecode-instructions) is a dictionary of assembly instructions. It contains documentation for all instructions that we are going to see in this post.

## Diving In

To understand this section, you do not need to know assembly language, but it is good to understand [memory stack](https://www.tutorialspoint.com/what-is-memory-stack-in-computer-architecture). Basically, it is a place to store data and instructions that the CPU can retrieve and execute.

For example, when you execute `1 + 1` in Python, the two numbers are stored as data and `+` is stored as an instruction in the stack so that CPU can then execute the command.

The goal of this section is to understand:

1. the execution order of a Python command
2. the instruction carried out to the memory stack

### Level 1

| Line # | Python      | Assembly        |      |          |
| ------ | ----------- | --------------- | ---- | -------- |
| 1      | a, b = 1, 2 | LOAD_CONST      | 1    | ((1, 2)) |
| 2      |             | UNPACK_SEQUENCE | 2    |          |
| 3      |             | STORE_FAST      | 0    | (a)      |
| 4      |             | STORE_FAST      | 1    | (b)      |
| 5      | a, b = b, a | LOAD_FAST       | 1    | (b)      |
| 6      |             | LOAD_FAST       | 0    | (a)      |
| 7      |             | **ROT_TWO**     |      |          |
| 8      |             | STORE_FAST      | 0    | (a)      |
| 9      |             | STORE_FAST      | 1    | (b)      |
| 10     |             | LOAD_CONST      | 0    | (None)   |

The most important instruction here is [ROT_TWO](https://docs.python.org/3/library/dis.html#opcode-ROT_TWO) [line 7] which swaps the two top-most stack items. In our case, it swaps 1 and 2 so that, later on, when [STORE_FAST](https://docs.python.org/3/library/dis.html#opcode-STORE_FAST) [lines 8 and 9] retrieves the top-of-stack values in order, it actually assigns 2 to `a` and 1 to `b`.

This is exactly what we would expect when we swap values.

It is also interesting to note that `a, b = 1, 2` internally creates a sequence and then unpack them [lines 1 and 2]. We will also observe this in the later examples.

### Level 2

| Line # | Python                  | Assembly        |      |                |
| ------ | ----------------------- | --------------- | ---- | -------------- |
| 1      | a, b, c, d = 1, 2, 3, 4 | LOAD_CONST      | 1    | ((1, 2, 3, 4)) |
| 2      |                         | UNPACK_SEQUENCE | 4    |                |
| 3      |                         | STORE_FAST      | 0    | (a)            |
| 4      |                         | STORE_FAST      | 1    | (b)            |
| 5      |                         | STORE_FAST      | 2    | (c)            |
| 6      |                         | STORE_FAST      | 3    | (d)            |
| 7      | a, b, c, d = d, c, b, a | LOAD_FAST       | 3    | (d)            |
| 8      |                         | LOAD_FAST       | 2    | (c)            |
| 9      |                         | LOAD_FAST       | 1    | (b)            |
| 10     |                         | LOAD_FAST       | 0    | (a)            |
| 11     |                         | **BUILD_TUPLE** | 4    |                |
| 12     |                         | UNPACK_SEQUENCE | 4    |                |
| 13     |                         | STORE_FAST      | 0    | (a)            |
| 14     |                         | STORE_FAST      | 1    | (b)            |
| 15     |                         | STORE_FAST      | 2    | (c)            |
| 16     |                         | STORE_FAST      | 3    | (d)            |
| 17     |                         | LOAD_CONST      | 0    | (None)         |

Yes, you can carry out value swaps with more than two variables. However, the underlying instructions are different. We no longer see `ROT_TWO` but we see [BUILD_TUPLE](https://docs.python.org/3/library/dis.html#opcode-BUILD_TUPLE) [line 11] which creates a tuple consuming a certain number of items from the stack.

You probably have already guessed. When swapping more values, Python, instead, creates a new tuple with the desired data [line 11] and then unpack the sequence [line 12]. This is different from the previous behavior where top of the stacks are directly swapped.

### Level 3

| Line # | Python                          | Assembly       |      |                |
| ------ | ------------------------------- | -------------- | ---- | -------------- |
| 1      | arr = [3, 2, 1, 0]              | BUILD_LIST     | 0    |                |
| 2      |                                 | LOAD_CONST     | 1    | ((3, 2, 1, 0)) |
| 3      |                                 | LIST_EXTEND    | 1    |                |
| 4      |                                 | STORE_FAST     | 0    | (arr)          |
| 5      | arr[0], arr[3] = arr[3], arr[0] | LOAD_FAST      | 0    | (arr)          |
| 6      |                                 | **LOAD_CONST** | 2    | **(3)**        |
| 7      |                                 | BINARY_SUBSCR  |      |                |
| 8      |                                 | LOAD_FAST      | 0    | (arr)          |
| 9      |                                 | **LOAD_CONST** | 3    | **(0)**        |
| 10     |                                 | BINARY_SUBSCR  |      |                |
| 11     |                                 | **ROT_TWO**    |      |                |
| 12     |                                 | LOAD_FAST      | 0    | (arr)          |
| 13     |                                 | **LOAD_CONST** | 3    | **(0)**        |
| 14     |                                 | STORE_SUBSCR   |      |                |
| 15     |                                 | LOAD_FAST      | 0    | (arr)          |
| 16     |                                 | **LOAD_CONST** | 2    | **(3)**        |
| 17     |                                 | STORE_SUBSCR   |      |                |
| 18     |                                 | LOAD_CONST     | 0    | (None)         |

This syntax sugar also applies to lists. **But this is where caveat exists**.

If you already understand the previous examples, this one does not look too different: Python first loads the two values from the array into the top of the stack in order [lines 5 ~ 10], and then it swaps the top two values of the stack [line 11]. Finally, values are assigned back to the array [lines 12 ~ 17].

So far so good.

### Level 4

| Line # | Python                                    | Assembly       |      |                |
| ------ | ----------------------------------------- | -------------- | ---- | -------------- |
| 1      | arr = [3, 2, 1, 0]                        | BUILD_LIST     | 0    |                |
| 2      |                                           | LOAD_CONST     | 1    | ((3, 2, 1, 0)) |
| 3      |                                           | LIST_EXTEND    | 1    |                |
| 4      |                                           | STORE_FAST     | 0    | (arr)          |
| 5      | arr[arr[0]], arr[0] = arr[0], arr[arr[0]] | LOAD_FAST      | 0    | (arr)          |
| 6      |                                           | **LOAD_CONST** | 2    | **(0)**        |
| 7      |                                           | BINARY_SUBSCR  |      |                |
| 8      |                                           | LOAD_FAST      | 0    | (arr)          |
| 9      |                                           | LOAD_FAST      | 0    | (arr)          |
| 10     |                                           | **LOAD_CONST** | 2    | **(0)**        |
| 11     |                                           | BINARY_SUBSCR  |      |                |
| 12     |                                           | BINARY_SUBSCR  |      |                |
| 13     |                                           | **ROT_TWO**    |      |                |
| 14     |                                           | LOAD_FAST      | 0    | (arr)          |
| 15     |                                           | LOAD_FAST      | 0    | (arr)          |
| 16     |                                           | **LOAD_CONST** | 2    | **(0)**        |
| 17     |                                           | BINARY_SUBSCR  |      |                |
| 18     |                                           | STORE_SUBSCR   |      |                |
| 19     |                                           | LOAD_FAST      | 0    | (arr)          |
| 20     |                                           | **LOAD_CONST** | 2    | **(0)**        |
| 21     |                                           | STORE_SUBSCR   |      |                |
| 22     |                                           | LOAD_CONST     | 0    | (None)         |

If you have ever written code like this, please either stop or put very good comments. It is almost unreadable, but it serves our purpose just right.

Note that **the execution order of the right-hand side is from left to right**, so that `arr[0]` [lines 5 ~ 7] is first loaded into the stack, and then `arr[arr[0]]` [lines 8 ~ 12]. Then things become straightforward: Python swap the top two values in the stack, and assign values back to the array. **The assignment order is also from left to right**, first assigning `arr[arr[0]]` [lines 14 ~ 18] and then `arr[0]` [lines 19 ~ 21].

### Level 5

| Line # | Python                                    | Assembly       |      |                |
| ------ | ----------------------------------------- | -------------- | ---- | -------------- |
| 1      | arr = [3, 2, 1, 0]                        | BUILD_LIST     | 0    |                |
| 2      |                                           | LOAD_CONST     | 1    | ((3, 2, 1, 0)) |
| 3      |                                           | LIST_EXTEND    | 1    |                |
| 4      |                                           | STORE_FAST     | 0    | (arr)          |
| 5      | arr[0], arr[arr[0]] = arr[arr[0]], arr[0] | LOAD_FAST      | 0    | (arr)          |
| 6      |                                           | LOAD_FAST      | 0    | (arr)          |
| 7      |                                           | **LOAD_CONST** | 2    | **(0)**        |
| 8      |                                           | BINARY_SUBSCR  |      |                |
| 9      |                                           | BINARY_SUBSCR  |      |                |
| 10     |                                           | LOAD_FAST      | 0    | (arr)          |
| 11     |                                           | **LOAD_CONST** | 2    | **(0)**        |
| 12     |                                           | BINARY_SUBSCR  |      |                |
| 13     |                                           | **ROT_TWO**    |      |                |
| 14     |                                           | LOAD_FAST      | 0    | (arr)          |
| 15     |                                           | **LOAD_CONST** | 2    | **(0)**        |
| 16     |                                           | STORE_SUBSCR   |      |                |
| 17     |                                           | LOAD_FAST      | 0    | (arr)          |
| 18     |                                           | LOAD_FAST      | 0    | (arr)          |
| 19     |                                           | **LOAD_CONST** | 2    | **(0)**        |
| 20     |                                           | BINARY_SUBSCR  |      |                |
| 21     |                                           | STORE_SUBSCR   |      |                |
| 22     |                                           | LOAD_CONST     | 0    | (None)         |

Now, this code actually does not do what it is supposed to do. The result of `arr` **does not change**. The key here is the execution order.

Lines 5 ~ 12 load 0 and 3 into the stack. After `ROT_TWO` [line 13], the top of the stack is 0 and the second is 3.

Lines 14 ~ 16 assign the top value of the stack, 0, to `arr[0]`. At this point, `arr` **has already been changed!** When lines 18 and 19 access `arr[0]`, the value is 0, rather than 3. Finally, when `STORE_SUBSCR` [line 21] gets evaluated, the equivalent Python expression is `arr[0] = 3`. As a result, the array is not changed.

### Level 6

I hope you now understand the differences between these swapping scenarios. Differences in instructions potentially affect the performance. And when swapping values in arrays, you need to pay extra attention to cases where values are changing "in real time".

Please let me know if you have any questions or suggestions. I enjoyed writing this post and I hope this post can help you understand Python a little bit better.
