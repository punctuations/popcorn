import json
import os
import re
import shutil
import stat
import sys
import threading
from zipfile import ZipFile

from _utils import styled_print

PROD_DIR = f"{os.path.expanduser('~')}{os.sep}.kernels"
TMP_DIR = f"{os.sep}tmp" if os.name != 'nt' else os.environ['TEMP']
TMP_DIR += f"{os.sep}popcorn-kernel{os.sep}"

rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
dotprofile = f"{os.environ['HOME']}{os.sep}.profile"


def install(config):
    if os.path.exists(rcfile):
        shellrc = open(rcfile, "r+")
        if f". $HOME{os.sep}.kernels{os.sep}butter.sh\n" not in shellrc.readlines():
            rclines = "\n".join(shellrc.readlines())
            shellrc.write(f"\n. $HOME{os.sep}.kernels{os.sep}butter.sh\n{rclines}")
            shellrc.close()
            styled_print.success(f"Successfully added {config['kernel_name']}")
        else:
            styled_print.info("kernel already installed.")
            shellrc.close()
    else:
        profile = open(dotprofile, "r+")
        if f". $HOME{os.sep}.kernels{os.sep}butter.sh\n" not in profile.readlines():
            proflines = "\n".join(profile.readlines())
            profile.seek(0)
            profile.write(f"\n. $HOME{os.sep}.kernels{os.sep}butter.sh\n{proflines}")
            profile.truncate()
            profile.close()
            styled_print.success(f"Successfully added {config['kernel_name']}")
        else:
            styled_print.info("kernel already installed.")
            profile.close()


def build_thread(output, location, config):
    os.chdir(f'{TMP_DIR}{location}')
    kernel_type = config['kernel_type']
    kernel_name = config['kernel_name']
    path_init = False

    try:
        shutil.rmtree(f"{PROD_DIR}{os.sep}{output}")
        os.mkdir(PROD_DIR)
        if kernel_type.lower() == "unpacked":
            os.mkdir(f"{PROD_DIR}{os.sep}{output}")
    except FileExistsError:
        pass
    except FileNotFoundError:
        pass

    try:
        # run seed_cmd
        if kernel_type.lower() == "unpacked":
            try:
                os.mkdir(f"{PROD_DIR}{os.sep}{output}")
            except FileExistsError:
                pass
            seed_cmd = re.compile(re.escape("@dest"), re.IGNORECASE) \
                .sub(f"{PROD_DIR}{os.sep}{output}", config["seed_cmd"])
        else:
            seed_cmd = re.compile(re.escape("@dest"), re.IGNORECASE).sub(f"{PROD_DIR}", config["seed_cmd"])
        exit_status = os.WEXITSTATUS(os.system(seed_cmd))

        # create unpacked kernel with unpacked stem.
        if kernel_type.lower() == "unpacked":
            try:
                arg_stem = re.compile(re.escape("@args"), re.IGNORECASE).sub("\"$@\"", config["unpacked_husk"])
                unpacked_husk = re.compile(re.escape("@local"), re.IGNORECASE) \
                    .sub(f"{PROD_DIR}{os.sep}{output[:-1] if output.endswith('/') else output}", arg_stem)
                with open(f"{PROD_DIR}{os.sep}{output}{kernel_name}", "w") as kernel:
                    kernel.write(f"#!/bin/bash\n{unpacked_husk}")
                    kernel.close()
            except KeyError:
                styled_print.error("Please include the unpacked_husk in the config.")
                kernel.close()
                sys.exit(0)

        # change permissions
        try:
            os.chmod(os.path.join(PROD_DIR, output if kernel_type.lower() == "packed" else f"{output}{kernel_name}"),
                     stat.S_IRWXO | stat.S_IRWXU | stat.S_IRWXG)
        except FileNotFoundError:
            styled_print.error("Unable to amend permission of file; file not found.")
            sys.exit(0)

        if exit_status == 0:
            styled_print.success("Compiled successfully!")

    except KeyError:
        styled_print.error("Orchard does not contain a seed_cmd")
        sys.exit(0)

    # add to buttered
    if os.name != "nt":
        try:
            buttered = open(f'{PROD_DIR}{os.sep}butter.sh', 'r')
            lines = buttered.readlines()
            buttered.close()

            if kernel_type == "packed":
                if f"export PATH=$PATH{os.pathsep}{PROD_DIR}\n" not in lines:
                    with open(f'{PROD_DIR}{os.sep}butter.sh', "a") as builds:
                        builds.write(f"export PATH=$PATH{os.pathsep}{PROD_DIR}\n")
                        builds.close()
            else:
                if f"export PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n" not in lines:
                    with open(f'{PROD_DIR}{os.sep}butter.sh', "a") as builds:
                        builds.write(f"export PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n")
                        builds.close()
        except FileNotFoundError:
            with open(f'{PROD_DIR}{os.sep}butter.sh', "w") as butter:
                if kernel_type == "packed":
                    butter.write(f"#!/bin/bash\nexport PATH=$PATH{os.pathsep}{PROD_DIR}\n")
                else:
                    butter.write(f"#!/bin/bash\nexport PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n")
                butter.close()
    else:
        # is windows, add to path
        if PROD_DIR not in os.environ["PATH"] and kernel_type.lower() == "packed":
            path_init = True
            os.system(f"[Environment]::SetEnvironmentVariable('PATH', '$env:PATH{os.pathsep}{PROD_DIR}', 'User')")
            os.system(f"$env:PATH += '{os.pathsep}{PROD_DIR}'")
            styled_print.success("Added to path")

        if f"{PROD_DIR}{os.sep}{output}" not in os.environ["PATH"] and kernel_type.lower() == "unpacked":
            path_init = True
            os.system(
                f"[Environment]::SetEnvironmentVariable('PATH', '$env:PATH{os.pathsep}{PROD_DIR}{output}', 'User')")
            os.system(f"$env:PATH += '{os.pathsep}{PROD_DIR}{output}'")
            styled_print.success("Added to path")

    # add buttered execution to rcfile/dotprofile
    if os.name != "nt":
        if os.path.exists(rcfile):
            shellrc = open(rcfile, "r")
            rclines = shellrc.read()
            shellrc.close()
            if f". \"$HOME{os.sep}.kernels{os.sep}butter.sh\"\n" not in rclines:
                with open(rcfile, "w") as init:
                    init.write(f". \"$HOME{os.sep}.kernels{os.sep}butter.sh\"\n")
                    init.write(rclines)
                    init.close()
        else:
            profile = open(dotprofile, "r")
            proflines = profile.read()
            profile.close()
            if f". \"$HOME{os.sep}.kernels{os.sep}butter.sh\"\n" not in proflines:
                with open(dotprofile, "w") as init:
                    init.write(f". \"$HOME{os.sep}.kernels{os.sep}butter.sh\"\n")
                    init.write(proflines)
                    init.close()
        if path_init:
            if os.name == 'nt':
                os.system(". $profile")
            else:
                if os.path.exists(rcfile):
                    os.system(f"source {rcfile}")
                else:
                    os.system(f"source {dotprofile}")


def build(config, is_unpacked, location):
    if not os.path.exists(PROD_DIR):
        os.mkdir(PROD_DIR)

    try:
        output = config["kernel_name"]
    except KeyError:
        styled_print.error("Orchard has no kernel_name.")
        sys.exit(0)

    if is_unpacked:
        output = output + os.sep
        thread = threading.Thread(target=build_thread, args=(output, location, config,))
        thread.start()
    else:
        thread = threading.Thread(target=build_thread, args=(output, location, config,))
        thread.start()


# popcorn orchard
# flags: -u, --unpacked: get a unpacked remote kernel; --url: specify the download URL
def orchard(args):
    """
    Used to get remote kernels.

    :param args: arguments passed to command
    """

    unpacked = ["-u", "--unpacked"]
    has_unpacked_flag = [element for element in unpacked if (element in args)]
    url = ["--url"]
    has_url_flag = [element for element in url if (element in args)]
    url_index = args.index(has_url_flag[0]) if len(has_url_flag) >= 1 else 0

    if len(args) >= 1:
        if not has_url_flag and len(args.split("/")) != 2:
            styled_print.error("Please follow the scheme of user/repo")
            sys.exit(0)

        if has_url_flag:
            hash_name = args[url_index + 1].encode('utf-8').hex()
            if os.name == 'nt':
                file_ext = '.zip'
                os.system(f"Invoke-WebRequest {args[url_index + 1]} -Out {TMP_DIR}url-{hash_name}{file_ext}")
            else:
                file_ext = '.tar.gz'
                os.system(f"curl --silent {args[url_index + 1]} -L --output {TMP_DIR}url-{hash_name}{file_ext}")

            with ZipFile(f'{TMP_DIR}url-{hash_name}{file_ext}', "r") as zip_obj:
                zip_obj.extractall(f'{TMP_DIR}url-{hash_name}')

            # load .kernelrc
            try:
                f = open(f"{TMP_DIR}url-{hash_name}{os.sep}.kernelrc")
                config = json.load(f)
                f.close()
            except FileNotFoundError:
                styled_print.error("Orchard does not contain .kernelrc")
                shutil.rmtree(f"{TMP_DIR}url-{hash_name}")
                sys.exit(0)

            # check to see if it is compatible with current os
            os_type = 'windows' if os.name == 'nt' else 'mac' if sys.platform == 'darwin' else 'linux'
            if os_type not in config["os"]:
                styled_print.warning("Unsupported os type.")
                shutil.rmtree(f"{TMP_DIR}url-{hash_name}")
                sys.exit(0)

            # find .kernelrc and make sure the kernel names aren't a conflict.
            if os.path.exists(f"{PROD_DIR}{os.sep}{args.splt('/')[1]}"):
                styled_print.warning("A kernel with that name already exists.")
                # delete the conflicted kernel here.
                shutil.rmtree(f"{TMP_DIR}url-{hash_name}")
                sys.exit(0)

            build(config, has_unpacked_flag, location=f"url-{hash_name}")
            if os.name != 'nt':
                install(config)
            else:
                styled_print.success(f"Successfully added {config['kernel_name']}")

        else:
            # unpacked from github.
            if os.name == 'nt':
                os.system("iwr")
            else:
                release = 'get release version here'
                os.system(f"curl --silent https://github.com/{args[url_index + 1]}/releases/download/{release}/kernel"
                          f".tar.gz -L --output {TMP_DIR}{args}.tar.gz")
            print("fetched from github.")

            with ZipFile(f'{TMP_DIR}{args}.tar.gz', "r") as zip_obj:
                zip_obj.extractall(f'{TMP_DIR}{args}')

            # load .kernelrc
            try:
                f = open(f"{TMP_DIR}{args}{os.sep}.kernelrc")
                config = json.load(f)
                f.close()
            except FileNotFoundError:
                styled_print.error("Orchard does not contain .kernelrc")
                shutil.rmtree(f"{TMP_DIR}{args}")
                sys.exit(0)

            # check to see if it is compatible with current os
            os_type = 'windows' if os.name == 'nt' else 'mac' if sys.platform == 'darwin' else 'linux'
            if os_type not in config["os"]:
                styled_print.warning("Unsupported os type.")
                shutil.rmtree(f"{TMP_DIR}{args}")
                sys.exit(0)

            # find .kernelrc and make sure the kernel names aren't a conflict.
            if os.path.exists(f"{PROD_DIR}{os.sep}{config['kernel_name']}"):
                styled_print.warning("A kernel with that name already exists.")
                # delete the conflicted kernel here.
                shutil.rmtree(f"{TMP_DIR}{args}")
                sys.exit(0)

            build(config, has_unpacked_flag, location=f"{args}")
            if os.name != 'nt':
                install(config)
            else:
                styled_print.success(f"Successfully added {config['kernel_name']}")

    else:
        styled_print.error("Please specify an orchard.")
