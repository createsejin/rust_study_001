let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
arglocal
%argdel
$argadd ~/Projects/rust_study_001/study013/tab1
$argadd ~/Projects/rust_study_001/study013/src/company.rs
$argadd ~/Projects/rust_study_001/study013/src/study/study001.rs
$argadd ~/Projects/rust_study_001/study013/src/study/study13.rs
if bufexists(fnamemodify("~/Projects/rust_study_001/study013/src/study/study13.rs", ":p")) | buffer ~/Projects/rust_study_001/study013/src/study/study13.rs | else | edit ~/Projects/rust_study_001/study013/src/study/study13.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/rust_study_001/study013/src/study/study13.rs
endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 1 - ((0 * winheight(0) + 28) / 56)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
" vim: set ft=vim :
