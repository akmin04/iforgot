# I Forgot
## Recall obscure commands by keyword
#### (Not tested on Linux/Windows)

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