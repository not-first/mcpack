# mcpack CLI

A rust CLI to initialize and view info about minecraft datapacks through interactive terminal prompts.
Only supports the newer datapack folder structure: pack format 48 (minecraft 1.21) and newer.

## Features

`create` to create a new datapack.

- Use current directory (y/N)
- If no, enter datapack folder name

**pack.mcmeta creation**

- Datapack name
- Datapack description
- Icon path (optional) - will copy the icon and rename it to pack.png
- Pack format (multiselect) - will automatically decide between a set format, multiple formats or a range. If it is multiple or a range, the newest is automatically selected as the pack_format key,

**datapack settings**

- Include minecraft namespace files (select for load and tick)
- Custom namespace name
- Starter folders (select of function, advancement, tags, recipe, loot_table, predicate)

---

`info` to view information of an existing datapack folder.

- Datapack folder/file location (it verifies by checking for pack.mcmeta file)

It shows name, description, minecraft version (with pack format). It shows the number of advancements, functions, loot tables etc.

---
`add` to add a specific datapack element to the datapack.

- Element type (e.g advancement, function, loot_table)
- Namespace (uses the first one it finds by default, but can be specified)

---

`zip` command to pack the datapack into a distributable zip file.

- folder name (optional, only if not in a datapack folder)

---
Features:

- interactive prompts
- automatic supported format inclusion
- automatic pack.ong copying and renaming
- flag option for all arguments
