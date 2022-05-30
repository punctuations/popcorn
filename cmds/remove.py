from _utils import styled_print


# strawberry build
def remove(args):
    if len(args) >= 1:
        styled_print.info(f"Removing {args[0]}...")
    else:
        styled_print.error("Please specify a berry to remove.")
