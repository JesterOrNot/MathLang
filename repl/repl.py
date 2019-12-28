import sys
import tty
import re
import os


def syntax_highlight(input, keywords: dict):
    i = 0
    splitted = input.split(" ")
    for f in splitted:
        for keyword in keywords.keys():
            if re.match(keyword, f):
                splitted[i] = keywords.get(
                    keyword) + re.findall(keyword, f)[0] + u"\u001b[0m"
        i += 1
    return " ".join(splitted)


def command_line(keywords: dict):
    tty.setraw(sys.stdin)
    input = ""
    index = 0
    sys.stdout.write("\n")
    sys.stdout.write(u"\u001b[1000D")
    sys.stdout.write(u"\u001b[0K")
    do_print = True
    while True:
        if do_print:
            sys.stdout.write(">>> ")
            do_print = False
        sys.stdout.flush()
        char = sys.stdin.read(1)
        if char == ":":
            sys.stdout.write("\n")
            sys.stdout.write(u"\u001b[1000D")
            sys.stdout.write(u"\u001b[0K")
            return ""
        if 32 <= ord(char) <= 126:
            input += char
        if ord(char) == 127:
            input = input[:index-1]
        if ord(char) in {10, 13}:
            sys.stdout.write("\n")
            sys.stdout.write(u"\u001b[1000D")
            sys.stdout.write(u"\u001b[0K")
            sys.stdout.flush()
            with open("/tmp/.mlangrepl.temp", "w+") as f:
                f.write(input)
            os.system("mathlang /tmp/.mlangrepl.temp")
            input = ""
            sys.stdout.write("\n")
            sys.stdout.write(u"\u001b[1000D")
            sys.stdout.write(u"\u001b[0K")
            sys.stdout.flush()
            do_print = True
            continue
        sys.stdout.write(u"\u001b[1000D")
        sys.stdout.write(u"\u001b[4C")
        sys.stdout.write(u"\u001b[0K")
        sys.stdout.write(syntax_highlight(input, keywords))
        if index > 0:
            sys.stdout.write(u"\u001b[" + str(index) + "C")
        sys.stdout.flush()
    return input


command_line({"sqrt": "\u001b[38;5;198m",
              "log": "\u001b[38;5;198m",
              "say": "\u001b[38;5;198m",
              "sin": "\u001b[38;5;198m",
              "cos": "\u001b[38;5;198m",
              "tan": "\u001b[38;5;198m",
              "cbrt": "\u001b[38;5;198m",
              "asin": "\u001b[38;5;198m",
              "acos": "\u001b[38;5;198m",
              "atan": "\u001b[38;5;198m",
              "\d+": "\u001b[38;2;214;119;119m",
              "\+": "\u001b[38;2;112;112;112m",
              "-": "\u001b[38;2;112;112;112m",
              "\*\*?": "\u001b[38;2;112;112;112m",
              "/": "\u001b[38;2;112;112;112m",
              "%": "\u001b[38;2;112;112;112m",
              "\".*\"": "\u001b[38;2;26;166;228m",
              })
