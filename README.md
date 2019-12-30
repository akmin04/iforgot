# I Forgot
## Recall obscure commands by keyword
#### (Not tested on Linux/Windows)

![Example GIF](https://i.imgur.com/Uu48maI.gif)

## Installation
Using curl:

```
sudo curl -L "https://github.com/akmin04/iforgot/releases/download/v0.1.0/iforgot" -o /usr/local/bin/iforgot
sudo chmod a+rx /usr/local/bin/iforgot
```

## Usage

### howto
#### Get a list of commands with matching keywords
`iforgot howto update`

`iforgot howto reset git vcs`

### list
#### List all saved entries
`iforgot list`

### new
#### Create a new entry
`iforgot new -c 'git reset --hard HEAD' -k git vcs reset head`

### delete
#### Delete a specified entry by ID
`iforgot delete 7`

`iforgot delete --all`
