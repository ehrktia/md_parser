### md parser

parser is used for checking language grammar defined by language

**initial thoughts**

- build a tree with markdown grammar (rules for markdown)

- read a file split in to words

- check for symbols

- when symbols are present pass it through the tree

- traverse the words in a loop through the tree

- output equivalent html for symbols

**unknown**

- [x] lexical analysis

- [ ] Definitive fine automaton (state machine)

**Todo**

- [x] learn about lexical analysis
  
  tokenize input words
  
      -  split the file into individual words
  
      -  form a pair for each incoming word using key : classification of word and value : actual word

```shell
      input=#
      token=(keyword,#)
```

`token` provides help to parser in next stage

- [ ] read file into buffer

- [ ] read lines and split by word

**research materials**

- ([Compilers](https://pgrandinetti.github.io/compilers/))
- [Basic Syntax | Markdown Guide](https://www.markdownguide.org/basic-syntax/)
  
  

**lexical analysis**

- build a map with all keywords involved in language

- read a file and split the content by word

- pass the word through map compare the values and tag the word with (keyword - key, value-word)
