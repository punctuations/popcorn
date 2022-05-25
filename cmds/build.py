from _utils import styled_print

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
                styled_print.info(f"Building from . outputting to {args[output_index + 1]}")
            else:
                styled_print.info(f"Building from {args[0]} outputting to {args[output_index + 1]}")
        except IndexError:
            styled_print.error("Please specify a directory.")
    else:
        if len(args) >= 1:
            styled_print.info(f"Building from {args[0]}")
        else:
            styled_print.info("Building from .")
