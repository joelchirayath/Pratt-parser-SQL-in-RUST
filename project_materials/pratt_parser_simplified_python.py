import re

token_pat = re.compile(r"\s*(?:(\d+)|(.))")


# Simplest Pythonic tokenizer
# Understanding this tokenizer is not required
def tokenize(program):
    for number, operator_or_identifier in token_pat.findall(program):
        if number:
            yield NumberToken(number)
        elif operator_or_identifier == "+":
            yield AddToken()
        elif operator_or_identifier == "-":
            yield SubToken()
        elif operator_or_identifier == "*":
            yield MulToken()
        elif operator_or_identifier == "/":
            yield DivToken()
        elif operator_or_identifier == '(':
            yield LeftParenthesesToken()
        elif operator_or_identifier == ')':
            yield RightParenthesesToken()
        elif operator_or_identifier == "&":
            yield AndToken()
        elif operator_or_identifier == "|":
            yield OrToken()
        elif operator_or_identifier == "!":
            yield NotToken()
        elif operator_or_identifier == ">":
            yield GreaterThanToken()
        elif operator_or_identifier == "<":
            yield LessThanToken()
        elif operator_or_identifier == "A":
            yield AscToken()
        elif operator_or_identifier == "D":
            yield DescToken()
        elif operator_or_identifier == "=":
            yield EqualsToken()
        else:
            yield IdentifierToken(operator_or_identifier)
    yield EofToken()

# Binary operation holder
class BinaryOperation:
    def __init__(self, left, op, right):
        self.left = left
        self.op = op
        self.right = right

    def __str__(self):
        return f"({self.left} {self.op} {self.right})"

# Unary operation holder
class UnaryOperation:
    def __init__(self, operand, op):
        self.operand = operand
        self.op = op

    def __str__(self):
        return f"({self.op} {self.operand})"


PREFIX_PRECEDENCE = 100

# Number and identifier tokens are special because their
# expression is just evaluated to their value.
# The "parser" argument in the prefix_expression() function
# is there because the parser needs to be passed to other
# tokens' prefix_expression() functions, but here it isn't needed
# and every prefix_expression() function needs to have the same
# signature
class NumberToken:
    def __init__(self, value):
        self.value = int(value)

    def prefix_expression(self, parser):
        return self.value


    def __str__(self):
        return str(self.value)


class IdentifierToken:
    def __init__(self, value):
        self.value = value

    def prefix_expression(self, parser):
        return self.value


    def __str__(self):
        return str(self.value)


# Operations like '+' and '-' can be both infix and prefix.
# They are in infix form when there are two operands (1 + 2, 3 - 14)
# and in prefix form when there is only one operand (5 + -3, 4 - +5).
# Order of infix operations is based on the precedences of operators,
# the bigger the precedence, the earlier that subexpression needs to be calculated.
# PREFIX_PRECEDENCE is a global variable, because it is the same for every
# prefix operator. Order of prefix operations is based strictly on their
# order in the expression, so precedence can be a constant.
class AddToken:
    infix_precedence = 25
    def prefix_expression(self, parser):
        return UnaryOperation(parser.build_expression(PREFIX_PRECEDENCE), "+")


    def infix_expression(self, left, parser):
        return BinaryOperation(left, "+", parser.build_expression(self.infix_precedence))


class SubToken:
    infix_precedence = 25
    def prefix_expression(self, parser):
        return UnaryOperation(parser.build_expression(PREFIX_PRECEDENCE), "-")


    def infix_expression(self, left, parser):
        return BinaryOperation(left, "-", parser.build_expression(self.infix_precedence))


# Division, multiplication, greater than, less than, equals, or, and are pretty much the same,
# the one difference being their precedences.
class DivToken:
    infix_precedence = 30
    def infix_expression(self, left, parser):
        return BinaryOperation(left, "/", parser.build_expression(self.infix_precedence))


class MulToken:
    infix_precedence = 30
    def infix_expression(self, left, parser):
        return BinaryOperation(left, "*", parser.build_expression(self.infix_precedence))


class GreaterThanToken:
    infix_precedence = 20
    def infix_expression(self, left, parser):
        return BinaryOperation(left, ">", parser.build_expression(self.infix_precedence))


class LessThanToken:
    infix_precedence = 20
    def infix_expression(self, left, parser):
        return BinaryOperation(left, "<", parser.build_expression(self.infix_precedence))


class EqualsToken:
    infix_precedence = 20
    def infix_expression(self, left, parser):
        return BinaryOperation(left, "=", parser.build_expression(self.infix_precedence))


class OrToken:
    infix_precedence = 15
    def infix_expression(self, left, parser):
        return BinaryOperation(left, "OR", parser.build_expression(self.infix_precedence))


class AndToken:
    infix_precedence = 10
    def infix_expression(self, left, parser):
        return BinaryOperation(left, "AND", parser.build_expression(self.infix_precedence))


# The not operator is never used in infix operations, so it doesn't have
# an infix operation function.
class NotToken:
    def prefix_expression(self, parser):
        return UnaryOperation(parser.build_expression(PREFIX_PRECEDENCE), "NOT")


# ASC and DESC operators are 'postfix' operators,
# a special variation of the 'infix' operators
# where there is no right side of the operation.
# This means they must be the final operation in the sequence.
class AscToken:
    infix_precedence = 5
    def infix_expression(self, left, _):
        return UnaryOperation(left, "ASC")


class DescToken:
    infix_precedence = 5
    def infix_expression(self, left, _):
        return UnaryOperation(left, "DESC")


class LeftParenthesesToken:
    infix_precedence = 0
    def prefix_expression(self, parser):
        expr = parser.build_expression(0)
        parser.check_next_token(RightParenthesesToken)
        return expr


# Right parentheses token needs to have an infix_precedence
# defined as zero because when the parser encounters it,
# it needs to know to stop parsing
class RightParenthesesToken:
    infix_precedence = 0


class EofToken:
    infix_precedence = 0


# The Parser class, used for holding global variables
# such as the current token and the tokenizer.
# In my opinion, this looks better than the 'global'
# variables provided in the resource below.
class Parser:
    def __init__(self, query):
        self.token = None
        self.token_iterator = tokenize(query)

    # sets the class' token to the first tokenized one,
    # and calls the first build_expression()
    def parse(self):
        self.advance_token()
        return self.build_expression(0)


    # the main loop of the Pratt parsing technique
    # read the article below (or any other given in the materials section)
    # for full understanding
    def build_expression(self, right_binding_power):
        current_token = self.token
        self.advance_token()
        left = current_token.prefix_expression(self)
        while right_binding_power < self.token.infix_precedence:
            current_token = self.token
            self.advance_token()
            left = current_token.infix_expression(left, self)
        return left


    # checks if the next token is the one that is being
    # asked for, if not, throws an error
    # useful only for parentheses checking
    # isn't important for the Pratt parsing technique
    # but is a necessary technicality
    def check_next_token(self, next_token=None):
        if next_token and next_token != type(self.token):
            raise SyntaxError(f'Expected {next_token}')
        # consume the token if it passed the check
        self.advance_token()


    def advance_token(self):
        self.token = next(self.token_iterator)


if __name__ == "__main__":
    # demonstrating arithmetic operator precedence
    print(Parser("(-i * 2) / 15 - ((44 * i) - 15 * 2)").parse())

    # demonstrating logic operator precedence
    print(Parser("!i & i | (i & i)").parse())

    # mixing the logic and arithmetic operators
    print(Parser("i > 15 * (44 - i / 7) | i < 0").parse())

    # demonstrating the prefix operators
    print(Parser("4 - +5").parse())

    # DESC and ASC SQL operators have the lowest precedence
    print(Parser("i * 2 D").parse())

    # this expression is here to demonstrate that if two prefix
    # expressions are right one to another, the order
    # is strictly dependent on the order of operators
    print(Parser("!-i + -!i").parse())


"""
This is a simplified Python tokenizer and parser 
used for easier learning of the PRATT parsing method.

Adapted to Python3 and reformated for better code structure:
https://eli.thegreenplace.net/2010/01/02/top-down-operator-precedence-parsing
(nud is equivalent to prefix_expression, led is equivalent to infix_expression).

This parser builds the operation tree instead of calculating the value,
the difference is that when calling the functions in tokens, one should evaluate 
expressions on the spot instead of creating binary and unary operation objects,
to get a calculated result. However, this is only possible when there are only
arithmetic operators.
Added SQL statements like ASC and DESC.
"""