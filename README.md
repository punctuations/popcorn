# 🍿 Popcorn
### Say goodbye to complicated installation instructions and build steps.
#### So easy you can sit back and enjoy some popcorn.
![](demo.gif)

## Contents
- [Requirements](#requirements)
- [Terminology](#terminology)
- [Installation](#install)
- [Updating](#updating)
- [How it works](#how-it-works)
- [Commands](#commands)
- [Config](#config)

### Requirements
- python 3.7+
- pip
- bash

#### [Back to contents](#contents)

### Terminology

- `kernel`: A filesystem that gets compiled by popcorn.
- Types:
  - `packed`: Refers to the type of output as a single binary from the filesystem.
  - `unpacked`: Refers to the type of output as a multi-file structure from the filesystem.
- `unpacked_husk`: The command to enter the main file in an unpacked kernel. (See example [config](https://github.com/punctuations/popcorn/blob/main/.kernelrc))
- `seed_cmd`: The command to compile the kernel.
- `dev_node`: The naming scheme given to a development kernel.

#### [Back to contents](#contents)

### Install

**Linux/Mac:**
```bash
curl -sSL cmdf.at/popcorn | bash
```
or with homebrew
```bash
brew tap punctuations/tap
brew install punctuations/tap/popcorn
```

**Windows:**
```powershell
iwr -useb raw.githubusercontent.com/punctuations/popcorn/main/install.ps1 | iex
```
or with Chocolately
```powershell
coming soon...
```

#### [Back to contents](#contents)

### Updating
If installed with the installation script just re-run the script, and it will update the program.

#### [Back to contents](#contents)

### How it works

popcorn is a cli tool to create a streamline development process for other command-line tools.

popcorn aims to help with the development process of not only clis using frameworks but also cli tools that have no frameworks, by creating and compiling an entry point to these types of files using the "unpacked" type.

### Commands

##### Build:
The build command will compile the code in the current directory to the production directory, making a cli tool compiled and stored as a kernel.

Ex.
```
> popcorn build
```

output flag:

Using the output flag will fix any conflicts between kernel names by storing it a different parent directory.
```
> popcorn build -o raspberry
```

##### Dev:
The dev command will launch a development environment which will listen to the specified directory for any changes and hot reload the development version of the command, to test out the command up-to-date and without having to manually compile every time.

The development command will be accessible through the `{kernel_name}{dev_node}`, for example: `popcorn-dev`

Ex.
```
> popcorn dev
```

listen flag:

The listen flag specifies the directory that it is listening to for changes.
```
> popcorn dev -l ./foo/bar
```

##### Install:
The install command is to install your kernel to the path and initialize the environment for it, this command should be run if the kernel has not been installed yet.

Ex.
```
> popcorn install
```

dev flag:

the dev flag will install the **development** version of the command, the path will be saved to either shellrc file or .profile (on windows it will go straight to path)
```
> popcorn install -d
```

##### Remove:
The remove command is to remove already installed kernels.

Ex.
```
> popcorn remove foo
```

dev flag:

The dev flag is used to indicate the requested kernel is a development kernel, which will be removed from a separate place from the production kernels.
```
> popcorn remove foo -d
```

##### Init:
The init command initializes the config file for popcorn.

Ex.
```
> popcorn init
```

force flag:

The force flag will replace any existing config file with the base/skeleton one.

```
> popcorn init --force
```

packed flag:

The packed flag generates a different skeleton to that of the unpacked or default one and will only have necessary values.
````
> popcorn init --packed
````


##### Help:
The help command provides help for the commands, providing more detail about them.

Ex.
```
> popcorn help dev
```

#### [Back to contents](#contents)

### Config
The config (.kernelrc) is used for all kernels, and is required, you can use the init command which will generate a skeleton config to edit for the user.

##### Skeleton config file:
```json
{
    "kernel_name": "popcorn",
    "kernel_type": "unpacked",
    "unpacked_husk": "python @local/popcorn.py @args",
    "dev_cmd": "popcorn dev",
    "seed_cmd": "cp -r * @dest",
    "advanced": {
        "os": ["mac", "windows", "linux"],
        "dev_node":  "-dev"
    }
}
```

##### Values:
- `kernel_name`: The name that will be used to access the program.
- `kernel_type`: Two possible values 'packed' or 'unpacked' which determine the file structure of the project.
  - `unpacked`: Requires entrypoint that bounces to another file. (directory)
  - `packed`: Is a binary or single executable ready to be ran, which does not require other files as dependencies. (one file)
- `unpacked_husk`: The unpacked_husk is optional and only used if the program is an unpacked type, this will be the command that is put into the entry point to the rest of the files.
- `dev_cmd`: The command that is ran when `popcorn -d` or `popcorn --dev` is run.
- `seed_cmd`: The command that transfers (or compiles) all files into the `@dest`
- `advanced/dev_node`: The naming scheme applied for the development command.

##### Variables:
- `@local`: used for the local directory for the program.
- `@dest`: used to specify the destination of the kernel (changes per command)
- `@args`: used only in `unpacked_husk`, will be all arguments passed to command.

#### [Back to contents](#contents)