# Code Test simple-calculator

Source: <https://github.com/ThePinkBear/code-test-simpleCalculator>

Wanted to give the problem a try so copied the instructions below for ease of reference

# Instructions

Your task is to write a simple calculator that can add, subtract and multiply values in a set of registers.
The syntax is quite simple

```html
<register> <operation> <value>
print <register>
quit
```

Allowed operations are add, subtract and multiply. Here is a simple example:

- A add 2
- A add 3
- print A
- B add 5
- B subtract 2
- print B
- A add 1
- print A
- quit

The output will be:

- 5
- 3
- 6

The calculator should also support using registers as values, with lazy evaluation (evaluated at print), e.g. A multiply B. Here is two more examples:

- a add 10
- b add a
- b add 1
- print b
- QUIT

The output should be: 11

- result add revenue
- result subtract costs
- revenue add 200
- costs add salaries
- salaries add 20
- salaries multiply 5
- costs add 10
- print result
- QUIT
The output should be:
- 90

Additional requirements:

- Any name consisting of alphanumeric characters should be allowed as register names.
- All input should be case insensitive.
- The program should either take its input from the standard input stream, or from a file.
- When the program is launched with one command line argument, input should be read from the file specified in
  the argument. When accepting input from file, it should not be necessary to include quit to exit the
program.

Invalid commands can be ignored, but should be logged to the console