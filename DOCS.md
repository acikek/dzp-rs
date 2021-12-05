# Advanced Documentation

## File Rules

You can specify how dzp should treat a file with File Rules. These are comments on the very first lines that start with `#:`. As of now, the only available rule is `ignore`, which has dzp ignore all the file's scripts, but this will be expanded in the future.

## kDoc

kDoc (Key-Doc) is the second iteration of dDoc, a doc comment system designed for the original JavaScript dzp. Instead of using YAML comments, kDoc is ingrained into the keys of the script, which allows for more complex comment data.

You can attach kDoc to any script using the `data` key. For data scripts, the `doc` key is also accepted as to avoid confusion.

#### deprecated

This key accepts a boolean value (e.g. true or false). If set to true, the script analysis will display a deprecation warning.

#### description

**Aliases**: desc, about

This key explains the purpose of the script. This should be a brief overview; include a more in-depth explanation if necessary with regular YAML comments.

#### usage

This key provides an example usage of the script, if possible. As of now, this is a single line, but further formatting options will be supported in the future.

#### use

**Alias**: uses

This key takes a list of scripts that this script uses internally.

#### defs

**Alias**: definitions

This key is for the definitions to be passed to task or procedure script. This is a map, where the key is the def name and the value is an "argument" type. 

An argument type takes a `type` key, which is what tag-type the argument should be, and a description (with the same aliases as the script description key).

#### keys

This key is for explaining the keys of a data script. This functions exactly how `defs` does except under a different label.

#### determine

**Alias**: determines

This key is for the value task and procedure scripts may determine. It accepts a standalone argument type.

### Example

```yml
my_task:
  type: task
  # Documentation
  doc:
    description: This does something
    usage: "- run my_task"
    defs:
      thing: 
        about: Some definition
        type: ObjectTag
    determine:
      desc: It's just untrue
      type: ElementTag(Boolean)
    deprecated: true
  # Actual content
  definitions: thing
  script:
  - debug log <[thing]>
  - determine false
```