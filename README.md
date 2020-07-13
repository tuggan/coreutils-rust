# coreutils-rust

This is just a fun project trying to reeimplement some of the gnu coreutils in Rust.

## Status
| Name                                                | Category        | Description                                                                                 | Status      |
| --------------------------------------------------- | --------------- | ------------------------------------------------------------------------------------------- | ----------- |
| arch                                                | Shell utilities | Prints machine hardware name (same as uname -m)                                             | Not started |
| b2sum                                               | Text utilities  | Computes and checks BLAKE2b message digest                                                  | Not started |
| base32                                              | Text utilities  | Encodes or decodes Base32, and prints result to standard output                             | Not started |
| base64                                              | Text utilities  | Encodes or decodes Base64, and prints result to standard output                             | Not started |
| basename                                            | Shell utilities | Removes the path prefix from a given pathname                                               | Not started |
| cat                                                 | Text utilities  | Concatenates and prints files on the standard output                                        | Partial     |
| chcon                                               | File utilities  | Changes file security context (SELinux)                                                     | Not started |
| chgrp                                               | File utilities  | Changes file group ownership                                                                | Not started |
| chmod                                               | File utilities  | Changes the permissions of a file or directory                                              | Not started |
| chown                                               | File utilities  | Changes file ownership                                                                      | Not started |
| chroot                                              | Shell utilities | Changes the root directory                                                                  | Not started |
| cksum                                               | Text utilities  | Checksums and count the bytes in a file                                                     | Not started |
| comm                                                | Text utilities  | Compares two sorted files line by line                                                      | Not started |
| cp                                                  | File utilities  | Copies a file or directory                                                                  | Not started |
| csplit                                              | Text utilities  | Splits a file into sections determined by context lines                                     | Not started |
| cut                                                 | Text utilities  | Removes sections from each line of files                                                    | Not started |
| date                                                | Shell utilities | Prints or sets the system date and time                                                     | Not started |
| dd                                                  | File utilities  | Copies and converts a file                                                                  | Not started |
| df                                                  | File utilities  | Shows disk free space on file systems                                                       | Not started |
| dir                                                 | File utilities  | Is exactly like "ls -C -b". (Files are by default listed in columns and sorted vertically.) | Not started |
| dircolors                                           | File utilities  | Set up color for ls                                                                         | Not started |
| dirname                                             | Shell utilities | Strips non-directory suffix from file name                                                  | Not started |
| du                                                  | Shell utilities | Shows disk usage on file systems                                                            | Not started |
| echo                                                | Shell utilities | Displays a specified line of text                                                           | Not started |
| env                                                 | Shell utilities | Displays and modifies environment variables                                                 | Not started |
| expand                                              | Text utilities  | Converts tabs to spaces                                                                     | Not started |
| expr                                                | Shell utilities | Evaluates expressions                                                                       | Not started |
| factor                                              | Shell utilities | Factors numbers                                                                             | Not started |
| false                                               | Shell utilities | Does nothing, but exits unsuccessfully                                                      | Not started |
| fmt                                                 | Text utilities  | Simple optimal text formatter                                                               | Not started |
| fold                                                | Text utilities  | Wraps each input line to fit in specified width                                             | Not started |
| groups                                              | Shell utilities | Prints the groups of which the user is a member                                             | Not started |
| head                                                | Text utilities  | Outputs the first part of files                                                             | Not started |
| hostid                                              | Shell utilities | Prints the numeric identifier for the current host                                          | Not started |
| id                                                  | Shell utilities | Prints real or effective UID and GID                                                        | Not started |
| install                                             | File utilities  | Copies files and set attributes                                                             | Not started |
| join                                                | Text utilities  | Joins lines of two files on a common field                                                  | Not started |
| link                                                | Shell utilities | Creates a link to a file                                                                    | Not started |
| ln                                                  | File utilities  | Creates a link to a file                                                                    | Not started |
| logname                                             | Shell utilities | Print the user's login name                                                                 | Not started |
| ls                                                  | File utilities  | Lists the files in a directory                                                              | Not started |
| md5sum                                              | Text utilities  | Computes and checks MD5 message digest                                                      | Not started |
| mkdir                                               | File utilities  | Creates a directory                                                                         | Not started |
| mkfifo                                              | File utilities  | Makes named pipes (FIFOs)                                                                   | Not started |
| mknod                                               | File utilities  | Makes block or character special files                                                      | Not started |
| mktemp                                              | File utilities  | Creates a temporary file or directory                                                       | Not started |
| mv                                                  | File utilities  | Moves files or rename files                                                                 | Not started |
| nice                                                | Shell utilities | Modifies scheduling priority                                                                | Not started |
| nl                                                  | Text utilities  | Numbers lines of files                                                                      | Not started |
| nohup                                               | Shell utilities | Allows a command to continue running after logging out                                      | Not started |
| nproc                                               | Shell utilities | Queries the number of (active) processors                                                   | Not started |
| numfmt                                              | Text utilities  | Reformat numbers                                                                            | Not started |
| od                                                  | Text utilities  | Dumps files in octal and other formats                                                      | Not started |
| paste                                               | Text utilities  | Merges lines of files                                                                       | Not started |
| pathchk                                             | Shell utilities | Checks whether file names are valid or portable                                             | Not started |
| pinky                                               | Shell utilities | A lightweight version of finger                                                             | Not started |
| pr                                                  | Text utilities  | Converts text files for printing                                                            | Not started |
| printenv                                            | Shell utilities | Prints environment variables                                                                | Not started |
| printf                                              | Shell utilities | Formats and prints data                                                                     | Not started |
| ptx                                                 | Text utilities  | Produces a permuted index of file contents                                                  | Not started |
| pwd                                                 | Shell utilities | Prints the current working directory                                                        | Not started |
| readlink                                            | Shell utilities | Displays value of a symbolic link                                                           | Not started |
| realpath                                            | File utilities  | Returns the resolved absolute or relative path for a file                                   | Not started |
| rm                                                  | File utilities  | Removes (deletes) files, directories, device nodes and symbolic links                       | Not started |
| rmdir                                               | File utilities  | Removes empty directories                                                                   | Not started |
| runcon                                              | Shell utilities | Run command with specified security context                                                 | Not started |
| sed                                                 | Text utilities  | Stream editor                                                                               | Not started |
| seq                                                 | Shell utilities | Prints a sequence of numbers                                                                | Not started |
| sha1sum, sha224sum, sha256sum, sha384sum, sha512sum | Text utilities  | Computes and checks SHA-1/SHA-2 message digests                                             | Not started |
| shred                                               | File utilities  | Overwrites a file to hide its contents, and optionally deletes it                           | Not started |
| shuf                                                | Text utilities  | generate random permutations                                                                | Not started |
| sleep                                               | Shell utilities | Delays for a specified amount of time                                                       | Not started |
| sort                                                | Text utilities  | sort lines of text files                                                                    | Not started |
| split                                               | Text utilities  | Splits a file into pieces                                                                   | Not started |
| stat                                                | Shell utilities | Returns data about an inode                                                                 | Not started |
| stdbuf                                              | Shell utilities | Controls buffering for commands that use stdio                                              | Not started |
| stty                                                | Shell utilities | Changes and prints terminal line settings                                                   | Not started |
| sum                                                 | Text utilities  | Checksums and counts the blocks in a file                                                   | Not started |
| sync                                                | File utilities  | Flushes file system buffers                                                                 | Not started |
| tac                                                 | Text utilities  | Concatenates and prints files in reverse order line by line                                 | Not started |
| tail                                                | Text utilities  | Outputs the last part of files                                                              | Not started |
| tee                                                 | Shell utilities | Sends output to multiple files                                                              | Not started |
| test                                                | Shell utilities | Evaluates an expression                                                                     | Not started |
| timeout                                             | Shell utilities | Run a command with a time limit                                                             | Not started |
| touch                                               | File utilities  | Changes file timestamps                                                                     | Not started |
| tr                                                  | Text utilities  | Translates or deletes characters                                                            | Not started |
| true                                                | Shell utilities | Does nothing, but exits successfully                                                        | Not started |
| truncate                                            | File utilities  | Shrink or extend the size of a file to the specified size                                   | Not started |
| tsort                                               | Text utilities  | Performs a topological sort                                                                 | Not started |
| tty                                                 | Shell utilities | Prints terminal name                                                                        | Not started |
| uname                                               | Shell utilities | Prints system information                                                                   | Not started |
| unexpand                                            | Text utilities  | Converts spaces to tabs                                                                     | Not started |
| uniq                                                | Text utilities  | Removes duplicate lines from a sorted file                                                  | Not started |
| unlink                                              | Shell utilities | Removes the specified file using the unlink function                                        | Not started |
| uptime                                              | Shell utilities | Tells how long the system has been running                                                  | Not started |
| users                                               | Shell utilities | Prints the user names of users currently logged into the current host                       | Not started |
| vdir                                                | File utilities  | Is exactly like "ls -l -b". (Files are by default listed in long format.)                   | Not started |
| wc                                                  | Text utilities  | Prints the number of bytes, words, and lines in files                                       | Not started |
| who                                                 | Shell utilities | Prints a list of all users currently logged in                                              | Not started |
| whoami                                              | Shell utilities | Prints the effective userid                                                                 | Complete    |
| yes                                                 | Shell utilities | Prints a string repeatedly                                                                  | Complete    |
