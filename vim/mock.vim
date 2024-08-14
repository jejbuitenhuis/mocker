if exists("b:current_syntax")
	finish
endif

" comments
syntax match mockComment "//.*$"

" common highlights
syntax keyword mockBoolean true false
syntax match mockNumber "-\?\d\+"

syntax keyword mockType int uint float boolean string

syntax region mockString start="\"" end="\"" skip="\\\"" contains=mockCharacter
syntax match mockCharacter "\\." contained

" table definition
syntax match mockStatement "^\<table\> \+\ze\w\+" nextgroup=mockTableName
syntax match mockTableName "\w\+" contained

" column definition
syntax region mockTableDefinition start="{" end="}" fold transparent contains=mockColumn,mockFunction,mockProvider,mockBoolean,mockNumber,mockString,mockType,mockComment
syntax match mockColumn "^\s*\w\+\ze" contained
syntax match mockFunction "\$\w\+\ze\(\w*\)"
syntax match mockProvider "#\w\+\ze\(\w*\)"

let b:current_syntax = "mock"

hi def link mockComment Comment

hi def link mockBoolean Boolean
hi def link mockNumber Number
hi def link mockFloat Float
hi def link mockString String
hi def link mockCharacter Character
hi def link mockType Type

hi def link mockStatement Statement
hi def link mockTableName Identifier
hi def link mockColumn Identifier
hi def link mockFunction Function
hi def link mockProvider Macro
