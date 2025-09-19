# CSV_MD
Struct to convert CSV to Markdown..


## Example

input text:
```shell
ID, Name, Description
1, Language, Rust
2, CSV_MD, Convert CSV to MD
3, License, MIT
4, Author, Roman Chumak
```

output text:
```shell
 | ID | Name     | Description       |
 | -- | -------- | ----------------- |
 | 1  | Language | Rust              |
 | 2  | CSV_MD   | Convert CSV to MD |
 | 3  | License  | MIT               |
 | 4  | Author   | Roman Chumak      |
```

output MD:
 | ID | Name     | Description       |
 | -- | -------- | ----------------- |
 | 1  | Language | Rust              |
 | 2  | CSV_MD   | Convert CSV to MD |
 | 3  | License  | MIT               |
 | 4  | Author   | Roman Chumak      |
