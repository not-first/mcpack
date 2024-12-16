// Successful Creation with All Arguments Provided

// All required arguments (name, description) are provided.
// Optional arguments (icon_path, pack_formats, etc.) are also provided.
// Creation with Missing Required Arguments

// Missing name.
// Missing description.
// Invalid Pack Format Values

// Providing pack formats that are not supported.
// Providing non-integer values for pack formats.
// Invalid Icon Path

// Providing a path that does not exist.
// Providing a path to a non-PNG file.
// Conflicting Flags

// Using --skip-icon flag along with providing an icon_path.
// Using conflicting namespace flags.
// Non-Existent Output Directory

// Specifying an output directory that does not exist.
// Providing a path without necessary write permissions.
// Existing Output Directory Without Force Flag

// Attempting to create a datapack in a directory that already exists without using the --force flag.
// User declines to overwrite the existing directory when prompted.
// Existing Output Directory With Force Flag

// Using the --force flag to overwrite an existing directory.
// Ensuring that the existing directory is properly overwritten.
// No Pack Formats Selected

// Not selecting any pack formats when prompted.
// Verifying that the command handles the absence of pack formats gracefully.
// Custom Namespace Without Specifying Folders

// Providing a custom namespace without selecting any starter folders.
// Ensuring default behavior when no folders are specified.
// Invalid Custom Namespace

// Providing an empty string or whitespace as a namespace.
// Using special characters or reserved keywords in the namespace.
// Conflicting Namespace Flags

// Combining flags that lead to ambiguous namespace configurations.
// Ensuring the command resolves conflicts appropriately.
// Skipping Starter Files with Provided Namespace Folders

// Using the --skip-starter-files flag while also specifying namespace folders.
// Verifying that starter files are not created despite provided folders.
// All Prompts Accepted

// User accepts all interactive prompts.
// Ensuring full datapack creation with all optional components.
// All Prompts Declined

// User declines optional prompts such as adding an icon or including the Minecraft namespace.
// Verifying that the datapack is created with minimal configurations.
// Partial Prompt Interactions

// User accepts some prompts and declines others.
// Mixed configurations based on user responses.
// Invalid Namespace Folders

// Providing folder names that are not part of ELEMENT_TYPES.
// Ensuring the command handles invalid folder names appropriately.
// Multiple Pack Formats Selection

// Selecting multiple valid pack formats.
// Verifying the correct handling and serialization of multiple formats.
// Single Pack Format Selection

// Selecting only one pack format.
// Ensuring that the supported_formats field is correctly set to None when only one format is selected.
// Maximum Supported Pack Formats Range

// Selecting the maximum range of pack formats.
// Verifying that the range is correctly serialized in the pack.mcmeta file.
// Minimum Supported Pack Formats Range

// Selecting the minimum range of pack formats.
// Ensuring proper serialization and validation.
// Boundary Conditions for Pack Formats

// Selecting pack formats at the boundary of supported ranges.
// Verifying correct handling of edge cases.
// Icon File Permissions Issues

// Providing an icon file with restricted permissions.
// Ensuring the command handles permission errors gracefully.
// Simultaneous Flag Usage

// Using multiple flags simultaneously and verifying their combined effects.
// Ensuring no unintended side effects occur.
// Invalid Output Directory Path Formats

// Providing malformed or incorrectly formatted paths.
// Ensuring the command validates and handles path formats correctly.
// Interruption During Datapack Creation

// Simulating interruptions (e.g., Ctrl+C) during the creation process.
// Verifying that partial creations are handled appropriately.
// Large Number of Minecraft Tags

// Selecting a large number of Minecraft tags.
// Ensuring that the command can handle bulk operations without issues.
// Custom Namespace with Special Characters

// Using namespaces that include special characters.
// Ensuring that directory structures are correctly created and handled.
// Repeated Executions with Same Parameters

// Running the create command multiple times with the same parameters.
// Verifying consistent behavior and handling of existing directories.
// Integration with Other Commands

// Using the create command in conjunction with other CLI commands.
// Ensuring that there are no conflicts or unintended interactions.
