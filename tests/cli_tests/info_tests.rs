// Successful Info Retrieval for a Valid Datapack Directory

// Datapack directory contains a well-formed pack.mcmeta.
// All required files and directories are present.
// Successful Info Retrieval from a Valid Zip File

// Zip file contains a well-formed datapack with pack.mcmeta.
// Proper extraction and reading of the zip file.
// Info Retrieval When pack.mcmeta is Missing

// Datapack directory does not contain pack.mcmeta.
// Proper error message is displayed.
// Info Retrieval from a Zip Without pack.mcmeta

// Zip file lacks pack.mcmeta.
// Appropriate error handling.
// Info Retrieval with Invalid pack.mcmeta Content (Malformed JSON)

// pack.mcmeta contains invalid JSON.
// Error is caught and reported.
// Info Retrieval with Unsupported pack_format

// pack.mcmeta has an unsupported pack_format value.
// Validation and error messaging.
// Info Retrieval with supported_formats as Array

// pack.mcmeta includes supported_formats as an array.
// Correct parsing and display.
// Info Retrieval with supported_formats as Object (min_inclusive, max_inclusive)

// pack.mcmeta defines supported_formats with range.
// Accurate interpretation of the range.
// Info Retrieval with Different Description Formats (String, Array, Object)

// Descriptions provided as plain strings, arrays, or JSON objects.
// Consistent and accurate parsing.
// Info Retrieval with Enabled Features

// pack.mcmeta includes enabled features.
// Proper listing and validation of features.
// Info Retrieval with Filters

// Datapack includes filter configurations.
// Correct parsing and display of filters.
// Info Retrieval with Overlays

// Datapack specifies overlays in pack.mcmeta.
// Accurate identification and listing of overlays.
// Compact Mode Displaying Minimal Information

// Using the --compact flag.
// Only essential information is shown.
// pack_info Flag Displaying Detailed Pack Information

// Utilizing the --pack-info flag.
// Comprehensive pack details are presented.
// namespaces Flag Displaying Only Namespaces Information

// Leveraging the --namespaces flag.
// Focused display of namespace-related data.
// Info Retrieval from a Datapack with Multiple Namespaces

// Datapack contains several namespaces with various assets.
// Each namespace is accurately detailed.
// Handling Empty or Default Datapacks

// Datapacks with minimal or default configurations.
// Graceful handling and appropriate information display.
// Info Retrieval with Missing Data Directory

// data directory is absent from the datapack.
// Proper error handling and messaging.
// Testing Interruption During Info Retrieval

// Simulating interruptions (e.g., Ctrl+C) while running the command.
// Ensuring graceful termination and state consistency.
// Testing Handling of Large Datapacks

// Datapacks with extensive data and numerous files.
// Performance and accurate information retrieval.
// Testing Path Inputs with Invalid Formats

// Providing incorrectly formatted paths to the command.
// Validation and informative error messages.
// Testing Output When Run in a Non-Datapack Directory

// Executing the info command outside a datapack context.
// Appropriate error reporting.
// Testing Zip Files with Nested Directories

// Zip archives containing deeply nested folder structures.
// Correct traversal and information extraction.
// Testing Zip Files with Multiple pack.mcmeta Files

// Zip archives erroneously containing more than one pack.mcmeta.
// Handling duplicates and reporting issues.
// Testing Non-Zip Files as Zip Paths

// Supplying non-zip files to the zip path parameter.
// Validation and error handling.
// Handling of Zip Files with Various Compression Methods

// Zip archives using different compression algorithms.
// Ensuring compatibility and accurate reading.
// Testing info Retrieval When the Zip File is Corrupt

// Corrupted zip archives.
// Proper error detection and messaging.
// Testing info Retrieval with Symbolic Links

// Datapacks containing symbolic links within directories.
// Correct resolution and information display.
// Testing info Retrieval with Special Characters in Namespace or Paths

// Namespaces or file paths containing special or Unicode characters.
// Accurate handling and representation.
// Repeated Executions with Same Parameters

// Running the info command multiple times consecutively.
// Consistent results and system stability.
