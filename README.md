# <img src="https://minecraft.wiki/images/Anvil_%28N%29_JE3.png?d438e" alt="Anvil Image" width="40"/> mcpack

A rust CLI to initialize, add files to, view information about and zip minecraft datapacks through interactive terminal prompts (or optional flags).
*Only supports the newer datapack folder structure: pack format 48 (minecraft 1.21) and newer.*

![Create command demo](https://vhs.charm.sh/vhs-1pytgMLLVTXDuSljotfYUP.gif)

## Installation

To get started, install the crate using cargo.

````bash
cargo install mcpack
````

For a comprehensive list of available commands and their usage, refer to the section below.

## Commands

If a command is executed without arguments or flags, it leads the user through an interactive prompt to gain required information for that action.

Each command has the ability to skip these interactive prompts (for automation purposes) by providing information through flags and arguments.

**All arguments and flags are optional.** If required information is not given through flags in a command, it will be asked through the interactive prompt.

### `create`

Initialises a new datapack with customizable properties in the current directory.

#### Usage (create)

````bash
mcpack create [NAME] [OPTIONS]
````

#### Arguments (create)

- `name`: Name of the datapack folder

#### Options (create)

- `-d, --description <DESCRIPTION>`: Description of the datapack
- `-i --icon <ICON_PATH>`: Path to pack icon (must be a PNG). *This will be copied into the datapack directory and correctly renamed to pack.png*
- `-f, --format <FORMAT(S)>`: Pack format(s) to support, as a space separated list (e.g. `-f 48, 61`)
- `-m, --minecraft`: Include the minecraft namespace folder (true if flag is provided, false if the flag is not)
  - `--load`: Include template load.mcfunction file
  - `--tick`: Include template tick.mcfunction file
- `-n, --namespace <NAMESPACE>`: Custom namespace name
  - `-s, --starters`: Starter files to create in custom namespace as a space separated list (e.g. `function advancement`). Refer to [here](#supported-starter-folders-and-element-types) for all possible values
- `-o, --output-dir <DIR>`: Output directory for the datapack
- `-F, --force`: Force overwriting previous folder contents without confirmation
- `--no-icon`: Skip pack icon prompt
- `--no-minecraft-tags`: Skip minecraft load/tick template file prompt
- `--no-starters`: Skip start file prompt

- `-h, --help`: View information about the command

#### Examples (create)

- **Create a datapack**

  ````bash
  mcpack create my_datapack
  ````

  Initialize a new datapack named `my_datapack`, prompting for information about contents and properties.

- **Create a datapack partially using flags**

  ````bash
  mcpack create example_name -f 48 61 -n example_namespace -s function advancement loot_table --no-icon
  ````

  Create a datapack named `example_name` supporting pack formats 48 and 61, including the specified starter files in a created `example_namespace`. Description, and the option to include minecraft tags are prompted. Icon path prompting is skipped.

- **Create a data without prompts**

  ````bash
  mcpack create example_name -d "example datapack description" -f 48 -n example_namespace -i /path/to/icon.png --minecraft --load --no-starters --force
  ````

  Initialise a new datapack named `example_name` with the description "example datapack description", supports pack format 48, creates a `example_namespace` folder, includes the specified icon as pack.png located at `/path/to/icon.png`, includes template `load.mcfunction` in the `minecraft` namespace folder, skips the starter file prompts, and forces overwriting any existing directory without confirmation.

### `info`

View information about a datapack folder/zip archive, such as namespaces, descriptions and support minecraft versions.

#### Usage (info)

````bash
mcpack info [NAME/PATH] [OPTIONS]
````

#### Arguments (info)

- `path`: Name/path of datapack folder or zip file. [more info](#namepath-mechanism-in-commands)

#### Options (info)

- `--compact`: Show only basic information (name, description, pack format)
- `--pack-info`: Show only information in pack.mcmeta file (excludes namespaces)
- `--namespaces`: Show only namespace information

- `-h, --help`: View information about the command

#### Examples (info)

- **View datapack information**

  ````bash
  mcpack info
  ````

  Displays basic information about the datapack the command was executed in, including its name, description, supported pack formats, namespaces and more.

- **View pack metadata only**

  ````bash
  mcpack info my_datapack.zip --compact
  ````

  Shows compact information about the datapack zip file `my_datapack.zip` including name, description, and supported pack formats.

### `add`

Add a new template file to an existing datapack, creating required folders if necessary.

#### Usage (add)

````bash
mcpack add [OPTIONS]
````

Must be executed in the base folder of a minecraft datapack (folder which contains the data and pack.mcmeta) if the `--path` flag is not provided.

#### Options (add)

- `-p, --path`: Path to datapack directory.

- `-e, --element <ELEMENT>`: Type of element to add. Refer [here](#supported-starter-folders-and-element-types) for all possible values
- `-x, --namespace <NAMESPACE>`: Name of namespace to add element to. (Not required if only one namespace exists)
- `-n, --name <NAME>`: Name for new file (refer to examples for how to include subdirectories)
- `-F, --force`: Force overwriting existing files without prompting

- `-h --help`: View information about the command

#### Examples (add)

- **Add a function to the default namespace**

  ````bash
  mcpack add -e function -n new_function
  ````

  Adds a new `function` file named `new_function.mcfunction` to the default namespace of the current datapack.

- **Add a loot table to a specific namespace within a subfolder**

  ````bash
  mcpack add -p /path/to/datapack -e loot_table -x custom_namespace -n entities/custom_loot
  ````

  Adds a new `loot_table` file named `custom_loot.json` to a new `custom_loot` folder inside the `custom_namespace` within the datapack located at `/path/to/datapack`.

- **Force overwrite an existing advancement file**

  ````bash
  mcpack add -e advancement -n existing_advancement --force
  ````

  Adds a new `advancement` file named `existing_advancement.json` to the default namespace, forcibly overwriting it if it already exists.

### `zip`

Zips a datapack into a zip archive file, useful for distribution to services such as [modrinth](https://modrinth.com/).

#### Usage (zip)

````bash
mcpack zip [NAME/PATH] [OPTIONS]
````

#### Arguments (zip)

- `path`: Name/path of datapack folder or zip file. [more info](#namepath-mechanism-in-commands)

#### Options (zip)

- `-i, --input-dir <DIR>`: Datapack directory of the unzipped datapack (not required if in current directory)
- `-n, --name <NAME>`: Custom name for the output zip file
- `-o, --output-dir <DIR>`: Output directory for the zip file
- `-F, --force`: Force overwriting existing zip file without prompting

- `-h, --help`: View information about the command

#### Examples (zip)

- **Zip the current datapack and place zip in parent directory**

  ````bash
  cd path/to/datapack
  mcpack zip
  ````

  Zips the current datapack folder and places the resulting zip file in the parent directory with the default naming convention.

- **Zip a specific datapack folder**

  ````bash
  mcpack zip my_datapack
  ````

  Checks for a folder named `my_datapack` inside the current directory, zips it, and places the resulting zip file in the current directory.

- **Zip a datapack with a custom zip name**

  ````bash
  mcpack zip my_datapack -n custom_archive.zip
  ````

  Zips the datapack located in `my_datapack` and names the output file `custom_archive.zip` in the current directory.

- **Specify an output directory for the zip File**

  ````bash
  mcpack zip my_datapack -o ./output/directory
  ````

  Zips the datapack named `my_datapack` and saves the zip file to `./output/directory`.

## Supported Features

### Supported Pack Formats

- Pack Format 48 and all newer main version.
  - Minecraft 1.21 and newer

### Supported Starter Folders and Element Types

- **Supported starters/element types:**
  - `function` (.mcfunction)
  - `tag` (.json)
  - `advancement` (.json)
  - `banner_pattern` (.json)
  - `chat_type` (.json)
  - `damage_type` (.json)
  - `enchantment` (.json)
  - `enchantment_provider` (.json)
  - `instrument` (.json)
  - `item_modifier` (.json)
  - `jukebox_song` (.json)
  - `loot_table` (.json)
  - `painting_variant` (.json)
  - `predicate` (.json)
  - `recipe` (.json)
  - `trim_material` (.json)
  - `trim_pattern` (.json)
  - `walk_variant` (.json)

### Name/Path Mechanism in Commands

- **`info` and `zip` Commands:**
  - **Name/Path Argument:** Accepts either the name of the datapack folder/zip file in the current directory OR the path to the folder/zip file.
  - **Examples:**
    - `mcpack info my_datapack` will look for a folder called `my_datapack` in the current directory
    - `mcpack zip /path/to/datapack.zip` will use the file at the provided path.

---
