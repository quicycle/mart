" Vim syntax file
" Language: arc calculation files
" Maintainer: Innes Anderson-Morrison
" Latest Revision: 25 May 2021

if exists("b:current_syntax")
  finish
endif

" Force vim mode for applying the syntax file
let s:cpo_save = &cpo
set cpo&vim


" NOTE: This list needs to be kept up to date as more directives are added
syn keyword arcDirective        SIMPLIFY_MULTIVECTORS
                                \ SHOW_COMMENTS
                                \ SHOW_INPUT
syn match   arcDirectiveMark    contained '#'
syn match   arcDirectiveLine    '^#.*$' contains=arcDirectiveMark,arcDirective

" syn keyword arcDefine           define nextgroup=arcMacroName
" syn match   arcMacroName        '\<\h\w*' nextgroup=arcMacroArgs
" syn region  arcMacroArgs        contained start=/(/ end=/)/ matchgroup=Function
"                                 \ contains=arcIdent,arcComma
"                                 \ nextgroup=arcMacroDefinition
" syn region  arcMacroDefinition  start='{' end='}' matchgroup=Ignore
"                                 \ contains=arcMacroName,arcMacroArgs,arcComma,arcMvec,arcDifferential

" syn region  arcMacroCall        start='\s?\w*!\[' end='\]'
"                                 \ contains=arcComma,arcDifferential,arcMvec,arcTerm,arcAlpha

" syn match   arcIdent            contained '\<\h\w*'
" syn match   arcComma            contained ','

syn match   arcShebang          '^\%1l#!.*$'
syn keyword arcTodo             contained TODO FIXME XXX NOTE
syn region  arcComment          oneline start='\%(^\|\s\+\)//' end='$'
                                \ contains=arcTodo,@Spell
syn region  arcComment          start='\%(^\|\s\+\)/*' end='*/'
                                \ contains=arcTodo,@Spell

syn region  arcMvec             start='{' end='}' contains=arcShortAlpha
syn region  arcDifferential     start='<' end='>' contains=arcShortAlpha
syn match   arcShortAlpha       contained '\d\+\|p'

syn match   arcAlpha            'a\d\+\|ap'
syn match   arcAlpha            '-a\d\+\|-ap'
syn match   arcTerm             't\d\+\|tp'
syn match   arcTerm             '-t\d\+\|-tp'


" Binding each of the above rules to a given syntax group
" :help group-name to quickly check how these currently apply

hi def link arcShebang              PreProc
hi def link arcTodo                 Todo
hi def link arcComment              Comment

" hi def link arcDefine               Keyword
" hi def link arcMacroName            Function
" hi def link arcIdent                Identifier
" hi def link arcMacroCall            PreProc

" Invalid directives will end up being error highlighted
hi def link arcDirective            Underlined
hi def link arcDirectiveMark        Special
hi def link arcDirectiveLine        Error

hi def link arcTerm                 Number
hi def link arcAlpha                Number
hi def link arcShortAlpha           Type

hi def link arcMvec                 Function
hi def link arcDifferential         Function


let b:current_syntax = "arc"

" Restore compatability mode if needed
let &cpo = s:cpo_save
unlet s:cpo_save
