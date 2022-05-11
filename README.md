# novelty-language
Similar to Brain Fuck, this is a simple compiler project that takes input and intperprets it's execution. Made in Rust

Reads in an input file (usually a text file, but not important), and breaks the input buffer down into a series of tokens based on the symbols read.
The tokens are then iterated through, simulating execution of those symbols in the program. They are simple operations that manipulate a unsigned 32-bit integer array and an unsigned 32-bit integer stack.

Below is a list of operations possible in this language and their functions:
'+' => Increment the value of array[index] by 1
'-' => Decrement the value of array[index] by 1
'_' => Set the value of array[index] to 0
'<' => Shift the array index pointer left
'>' => Shift the array index pointer right
'^' => Shift the array index pointer to the value of array[index]
'#' => Push the value of array[index] to the stack
'$' => Pop from the stack and push into array[index]
'&0' => Output the value of array[index] as a number to stdout
'&a' => Output the value of array[index] as an ASCII character to stdout
'?0' => Accept input as a number from stdin and push to array[index]
'?a' => Accept input as an array of ASCII characters and push to array[index+array_char]
'{' => Perform a conditional jump forwards to matching '}' when array[index] == 0
'}' => Conditional jump marker
':' => Perform a non-conditional jump backwards to matching '='
'=' => Non conditional jump marker

Semantic Rules:
 - Integers are unsigned 32-bit integers, cannot be negative.
 - Increment and Decrement statements cannot modify integers to be outside of the 32-bit unsigned range, and will cause an exit-error
 - Shift statements cannot shift the index pointer outside of the array bounds
 - Popping from an empty stack will just return a 0
 - Output statements will always output something to the screen
 - Numeric input statements will exit-error when encountering a non-digit in the input_buffer
 - Conditional and non-conditional jumps will perform matching operations, where they will find the cooresponding marker that matches with the jump-block. Will exit-error if a cooresponding match cannot be found.
      > For example, in "{ + { + } - }", the entirety of " + { + } - " would be skipped if array[index] == 0 at the first '{' executed.
