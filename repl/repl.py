from pygments.lexer import RegexLexer
from pygments.token import *
from prompt_toolkit.styles import Style
from prompt_toolkit.shortcuts import prompt
from prompt_toolkit.lexers import PygmentsLexer


class DiffLexer(RegexLexer):

    tokens = {
        'root': [
            ("sqrt", Keyword),
            ("log", Keyword),
            ("say", Keyword),
            ("sin", Keyword),
            ("cos", Keyword),
            ("tan", Keyword),
            ("cbrt", Keyword),
            ("asin", Keyword),
            ("acos", Keyword),
            ("atan", Keyword),
            (r"[0-9]", Number),
        ]
    }
    style = Style.from_dict(
        {
            'pygments.keyword': '#5db100 bold',
            "pygments.literal.number": '#ff6d3d'
        }
    )


def main():
    answer = prompt("Give me some input: ", lexer=PygmentsLexer(DiffLexer), style=DiffLexer.style)
    print("You said: %s" % answer)


main()
