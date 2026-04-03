#!/usr/bin/env python3
import tiexiu


def main():
    model = tiexiu.compile("a grammar")
    print(model)

    out = tiexiu.parse("g", "hello world")
    print(out)


if __name__ == "__main__":
    main()
