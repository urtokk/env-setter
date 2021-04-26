# Env setter

## The solution to which problem?

I have several projects that need specific environment variables in order to run, which differ between projects.
To avoid to make errors everytime or set variables from a different project, I want a specific set of environment variables with possible default values.

## Usage

The easiest way is currently to use the pipe parameter and source the result. Currently that would look like:

```sh
env-setter --config resources/test.yaml --env-set test-set && source /tmp/set-env && rm /tmp/set-env
```

That uses the example config in resources and has the following content:

```yaml
shell: Fish

sets:
  test-set:
    - name: "TESTKEY"
      value: "123"
    - name: "TESTTOKEN"
  another-set:
    - name: "ANOTHERTEST"
```

The shell supports Posix and fish shell.
Under *sets* you can find two sets with one variable defining a default value.

The example yaml shows the possibilities for variables. There we have two sets named _test-set_ and _another-set_. Our example execution called the _test-set_, which has two variables.
First one with the name _TESTKEY_ and a default value of "123". The default value is optional.

## Arguments

The help flag gives:

```sh
USAGE:
    env-setter [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -s, --stdout     print variable commands to stdout
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>      path to configfile [default: .config/env-setter.yaml]
    -e, --env-set <env-set>    Env set to use
    -f, --file <pipe>          output file with env set [default: /tmp/set-env]
```

The options are self explanatory, but one point to remember is that the stdout flag superseeds the file argument.
