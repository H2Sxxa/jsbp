# JSBP (Jar String Bytes Patcher)

A tool to modify the constant String value in `jar` file

## Performance

Written in Rust and Support Asynchronous Patch, so it's fast.

## Principle

Jar is usually encoded by Zip and constant String is usually denoted by `SIZE_BYTE SIZE_BYTE UTF_BYTES`.

Use `ZipDecoder` to find a file in jar and find use new `SIZE_BYTE SIZE_BYTE UTF_BYTES` to replace original data to modify the constant String.

> [!WARNING]  
> Because the Bytes in a file are unpredictable.
> So there's a risk that using the tool will break the file. 

## How to use

See `./jsbp --help`
