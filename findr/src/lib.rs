// Findr
// Rust Port of Find.
// Finds entries in one or more Paths; these entries can be filtered by files, links, directories,
// or names that match an optional pattern.
//
// -Findr must have one or more positional arguments that indicate tho paths to search.
// -For each path, Findr will recursively search for all files and directories found therein.
// E.G. if I am in the tests/inputs directory, and indicate '.', Findr will list all the contents.
// -Use the '-type' option to specify the type of output to be displayed:
// f = files
// l = links
// d = directories
//-Use the '-name' option to locate items matching a file glob pattern.
//E.G. -name *.csv will find all entries ending in .csv
//must be escaped or put in quotes: '-name \*.csv' or '-name "*.csv"'
//-Use the '-o' option to or together multiple options.
//-Error if path does not exist
//-Print filename if path exists
