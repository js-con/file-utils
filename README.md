## About
A command line tool, making it easier to deal with files/directories.

## Usage

### flatten
```shell
./fu flat target_dir [new_dir] [--deep] 
```

### rename

- change suffix

shell: 
```shell
./fu rn {glob} {new_suffix}
```
bash:
```bash
./fu rn "{glob}" {new_suffix}
```

**notice!**
- argument {new_suffix} needn't start with ".", if that is, "." would be ignored.
- if argument {new_suffix} doesn't exist, the suffix of files would be removed. 

## Features

- [x] flatten folder
  - [x] flatten folder in target dir
  - [x] rename/move after flatten
  - [x] deep mode 
- [x] rename files 
  - [x] batch change suffix of files(glob rules)
