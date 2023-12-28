# LexicalParser

Rust program to map out BNF structures and produce a “diagrammed” version of the input string, which means a sentence in properly parenthesized form. “Properly parenthesized” means that each non-terminal appearing in the input string now has parentheses around it. For instance, the input string:

    alice found mean green book

would be parenthesized as

    (((alice)) (found) (((mean(green))book)))


Here is the structure used:
        <sentence>    -->  <subject> <verb_phrase> <object>
        <subject>     -->  <noun_phrase>
        <verb_phrase> -->  <verb> | <verb> <adv>
        <object>      -->  <noun_phrase>
        <noun_phrase> -->  [<adj_phrase>] <noun> [<prep_phrase>]
        <adj_phrase>  -->  <adj> | <adj> <adj_phrase>
        <prep_phrase> -->  <prep> <noun_phrase>

The program will also note two distinct error conditions. First, if a given string does not consist of valid tokens, then respond with this message:

    Input has invalid tokens.

Second, if the parameter is a string consisting of valid tokens but it is not a legitimate sentence according to the grammar, then respond with the message:

    Input is not a sentence.
