set shiftwidth=4
set smarttab
set number
set laststatus=2
set statusline=%F
set tabstop=8
set sw=4
set expandtab

highlight ExtraWhitespace ctermbg=red guibg=red
match ExtraWhitespace /\s\+$/

hi clear SpellBad
hi SpellBad cterm=underline
" Set style for gVim
hi SpellBad gui=undercurl

