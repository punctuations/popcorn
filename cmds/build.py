output = ["-o", "--output"]


# strawberry build
def build(args):
    PROD_DIR = "$HOME/.local/bin"
    # when file is placed here make sure to set path variable to file containing accessible target
    # ex. (unpacked) export PATH = $PATH:$HOME/.local/bin/strawberry
    # ex. (packed) export PATH  = $PATH:$HOME/.local/bin

    has_output_flag = [element for element in output if (element in args)]
    output_index = args.index(has_output_flag[0]) if len(has_output_flag) >= 1 else 0

    if has_output_flag:
        try:
            if output_index == 0:
                print(f"Building from . outputting to {args[output_index + 1]}")
            else:
                print(f"Building from {args[0]} outputting to {args[output_index + 1]}")
        except IndexError:
            print("Please specify a directory.")
    else:
        if len(args) >= 1:
            print(f"Building from {args[0]}")
        else:
            print("Building from .")
