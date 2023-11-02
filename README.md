# YACT (YAML as Command Tree)

YACT is an open-source tool that allows you to load shell commands directly from YAML.

With YACT, you can:
- **Organize Commands Hierarchically**
  - Subcommands are supported by autocompletion.
  
- **Easily Define Custom Input Autocompletions**

- **Comfortable Command Editing**
  - No Compile, No Reloading.
  - Edit via Command Line Interface.
  - Divide Commands with Sub-commands that are Mutually Callable.

## Prerequisities
- Bash or Zsh shell
- `sed` command installed

## Installation
1. Clone this repository to your preferred location:
```
$ git clone https://github.com/Harukaze9/YACT.git
```

2. Enter the cloned directory and execute the installation script:
```
$ ./install.sh
```
Alternatively, you can add the following line directly to your shell's configuration file and then refresh your shell:
```
source /path/to/YACT/src/source-yact.sh
```

# About YACT
YACT is a system that converts YAML files into shell commands. In YACT, we use the following terminologies:

- **YACT File**: A file written in YAML format that describes the command definitions.
- **YACT Command**: A command loaded from the YACT file.

## YACT File
A YACT file is a YAML file named in the format `command-name.yaml` and written under specific rules. The filename represents the command name, while the contents of the YAML file embody the command's definitions, such as the executable command, sub-commands, input completions, and more.

By default, YACT files placed in the `YACT/Commands/` directory are automatically loaded as YACT commands.

### Setting Executable Commands
You can define the executable command by using the key `x`.
For instance, in the YACT file below, executing the `hello` command outputs `hello world!`.
```yaml: hello.yaml
x: echo "hello world!"
```
If no other key is present, the `x` key can be omitted.
```yaml: hello.yaml
echo "hello world!"
```
You can pass arguments to the command using placeholders in the format `{N}`.
```yaml: hello2.yaml
echo "given arguments are {0} and {1}!"
```

### Setting Sub-commands
The keys in the YAML represent the names of the sub-commands. The tree structure of the YAML keys directly translates to the command structure, without any restrictions on depth.

```yaml: greet.yaml
x: echo "hi!"
hello: echo "Hello World!"
good:
  morning: echo "Good morning, sunshine!"
  evening: echo "Good evening, had a good day?"
```

Executing each sub-command produces the following outputs:
```
$ greet
hi!

$ greet hello
Hello World!

$ greet good morning
Good morning, sunshine!

$ greet good evening
Good evening, had a good day?
```

### Configuring Overloads
By providing a list of values to the executable command, you can achieve command overloading.
```yaml: hello3.yaml
x:
  - echo hello
  - echo hello {0}!
  - echo hello {0} and {1}!
```
The appropriate command is executed based on the number of arguments provided.
```
$ hello3
hello

$ hello3 Alice
hello Alice!

$ hello3 Alice Bob
hello Alice and Bob!
```

### Setting Multi-line Commands
Commands spanning multiple lines can be written in the form of shell functions. Constructs like if-statements or for-loops, as well as local variable definitions, are supported.
```yaml: check.yaml
x: |
  if [ -f {0} ]; then
      echo "File {0} exists."
  else
      echo "File {0} does not exist."
  fi
```

## Behavior of YACT Commands
Loading a `command-name.yaml` YACT file results in the `command-name` YACT command being loaded as a shell function. Executing this command runs the commands described in the YACT file.

### Default Options
YACT commands come with built-in options that allow you to review or edit commands.

#### `-p`: Display a tree of sub-commands
Format: `[YACT command path] -p`
This displays all commands in a tree format, starting from the specified path.

#### `-a`: Add a command to the specified path
Format: `[YACT command path] -a [desired executable command]`
You can pass the desired executable command using the `-a` option to any YACT command path.
```
$ foo your another subcommand -a 'echo "you can add any commands here!"'
```
If the path doesn't exist, it gets added. If both the path and command already exist, the existing command gets overwritten.

#### `-r`: Remove the specified path
Format: `[YACT command path] -r`
You can delete a predefined YACT command path by passing the -r option.
```
$ foo unused-command pull -r
```
If the path doesn't exist, it's added. If both the path and command exist, the existing command is overwritten.

#### `-e`: Open the command in a text editor
Format: `[command-name] -e`
Executing a YACT command with the `-e` option opens the YACT file in the configured text editor.

## Utilities
As an auxiliary tool, the `yact` command is provided. With this, many operations can be completed directly via the CLI.

- `yact list`: Display a list of YACT commands.
- `yact new`: Add a new YACT command with the specified name.
- `yact edit`: Open the YACT file of the specified command in a text editor.
- `yact remove`: Delete the specified YACT command by name.
- `yact rename`: Change the name of the specified YACT command.
- `yact register`: Load the specified YAML file as a YACT file (internally, this creates a symbolic link in `YACT/Commands`).


## Contributing
If you like this tool, we welcome any and all contributions.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
