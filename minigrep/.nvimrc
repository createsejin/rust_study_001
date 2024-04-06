version 6.0
let s:cpo_save=&cpo
set cpo&vim
cnoremap <silent> <C-R> <Cmd>lua require("which-key").show("\18", {mode = "c", auto = true})
inoremap <silent> <C-R> <Cmd>lua require("which-key").show("\18", {mode = "i", auto = true})
inoremap <C-L> <Right>
inoremap <C-B> ^i
inoremap <C-K> <Up>
inoremap <C-H> <Left>
inoremap <C-J> <Down>
imap <M-C-Right> <Plug>(copilot-accept-line)
imap <M-Right> <Plug>(copilot-accept-word)
imap <M-Bslash> <Plug>(copilot-suggest)
imap <M-[> <Plug>(copilot-previous)
imap <M-]> <Plug>(copilot-next)
imap <Plug>(copilot-suggest) <Cmd>call copilot#Suggest()
imap <Plug>(copilot-previous) <Cmd>call copilot#Previous()
imap <Plug>(copilot-next) <Cmd>call copilot#Next()
imap <Plug>(copilot-dismiss) <Cmd>call copilot#Dismiss()
cnoremap <silent> <Plug>(TelescopeFuzzyCommandSearch) e "lua require('telescope.builtin').command_history { default_text = [=[" . escape(getcmdline(), '"') . "]=] }"
inoremap <C-W> u
inoremap <C-U> u
nnoremap  <Cmd> %y+ 
tnoremap  h
nnoremap  h
tnoremap <NL> j
nnoremap <NL> j
tnoremap  k
nnoremap  k
tnoremap  l
nnoremap  l
nnoremap  <Cmd> NvimTreeToggle 
nnoremap  <Cmd> w 
nnoremap <silent> Þ <Nop>
nnoremap <silent>  <Cmd>lua require("which-key").show("\23", {mode = "n", auto = true})
tnoremap  
nnoremap  <Cmd> noh 
xnoremap <silent>  Þ <Nop>
xnoremap <silent>   <Cmd>lua require("which-key").show(" ", {mode = "v", auto = true})
nnoremap <silent>  pÞ <Nop>
nnoremap <silent>  fÞ <Nop>
nnoremap <silent>  cÞ <Nop>
nnoremap <silent>  tÞ <Nop>
nnoremap <silent>  dÞ <Nop>
nnoremap <silent>  rcÞ <Nop>
nnoremap <silent>  rÞ <Nop>
nnoremap <silent>  mÞ <Nop>
nnoremap <silent>  gÞ <Nop>
nnoremap <silent>  wÞ <Nop>
nnoremap <silent>  Þ <Nop>
nnoremap <silent>   <Cmd>lua require("which-key").show(" ", {mode = "n", auto = true})
xnoremap <silent>  cÞ <Nop>
nnoremap  B <Cmd> DapToggleBreakpoint 
nnoremap  th <Cmd> Telescope themes 
nnoremap  fh <Cmd> Telescope help_tags 
nnoremap  ma <Cmd> Telescope marks 
nnoremap  pt <Cmd> Telescope terms 
nnoremap  fb <Cmd> Telescope buffers 
nnoremap  gt <Cmd> Telescope git_status 
nnoremap  fw <Cmd> Telescope live_grep 
nnoremap  cm <Cmd> Telescope git_commits 
nnoremap  fa <Cmd> Telescope find_files follow=true no_ignore=true hidden=true 
nnoremap  fz <Cmd> Telescope current_buffer_fuzzy_find 
nnoremap  ff <Cmd> Telescope find_files 
nnoremap  fo <Cmd> Telescope oldfiles 
nnoremap  e <Cmd> NvimTreeFocus 
vnoremap  / <Cmd>lua require('Comment.api').toggle.linewise(vim.fn.visualmode())
nnoremap  ws <Cmd> SessionSave 
nnoremap  cg <Cmd> Copilot enable 
nnoremap  cb <Cmd> Copilot disable 
nnoremap  wr <Cmd> SessionRestore 
nnoremap  ct <Cmd> CopilotChatToggle 
vnoremap  ct <Cmd> CopilotChatToggle 
vnoremap  cf <Cmd> CopilotChatFix 
nnoremap  rn <Cmd> set rnu! 
nnoremap  n <Cmd> set nu! 
nnoremap  ch <Cmd> NvCheatsheet 
nnoremap  b <Cmd> enew 
nnoremap <silent> !iÞ <Nop>
nnoremap <silent> !aÞ <Nop>
nnoremap <silent> !Þ <Nop>
nnoremap <silent> ! <Cmd>lua require("which-key").show("!", {mode = "n", auto = true})
nnoremap <silent> " <Cmd>lua require("which-key").show("\"", {mode = "n", auto = true})
xnoremap <silent> " <Cmd>lua require("which-key").show("\"", {mode = "v", auto = true})
xnoremap # y?\V"
nnoremap & :&&
nnoremap <silent> ' <Cmd>lua require("which-key").show("'", {mode = "n", auto = true})
xnoremap * y/\V"
nnoremap <silent> <iÞ <Nop>
nnoremap <silent> <aÞ <Nop>
nnoremap <silent> <Þ <Nop>
nnoremap <silent> < <Cmd>lua require("which-key").show("<", {mode = "n", auto = true})
vnoremap < <gv
nnoremap <silent> >iÞ <Nop>
nnoremap <silent> >aÞ <Nop>
nnoremap <silent> >Þ <Nop>
nnoremap <silent> > <Cmd>lua require("which-key").show(">", {mode = "n", auto = true})
vnoremap > >gv
nnoremap Y y$
nnoremap <silent> [Þ <Nop>
nnoremap <silent> [ <Cmd>lua require("which-key").show("[", {mode = "n", auto = true})
nnoremap <silent> ]Þ <Nop>
nnoremap <silent> ] <Cmd>lua require("which-key").show("]", {mode = "n", auto = true})
nnoremap <silent> ` <Cmd>lua require("which-key").show("`", {mode = "n", auto = true})
nnoremap <silent> caÞ <Nop>
nnoremap <silent> ca <Cmd>lua require("which-key").show("ca", {mode = "n", auto = true})
nnoremap <silent> cÞ <Nop>
nnoremap <silent> c <Cmd>lua require("which-key").show("c", {mode = "n", auto = true})
nnoremap <silent> ciÞ <Nop>
nnoremap <silent> dÞ <Nop>
nnoremap <silent> d <Cmd>lua require("which-key").show("d", {mode = "n", auto = true})
nnoremap <silent> diÞ <Nop>
nnoremap <silent> daÞ <Nop>
xnoremap gb <Plug>(comment_toggle_blockwise_visual)
xnoremap gc <Plug>(comment_toggle_linewise_visual)
nnoremap gb <Plug>(comment_toggle_blockwise)
nnoremap gc <Plug>(comment_toggle_linewise)
nnoremap <silent> gÞ <Nop>
nnoremap <silent> g <Cmd>lua require("which-key").show("g", {mode = "n", auto = true})
nnoremap <silent> g~iÞ <Nop>
nnoremap <silent> g~aÞ <Nop>
nnoremap <silent> g~Þ <Nop>
nnoremap <silent> guiÞ <Nop>
nnoremap <silent> guaÞ <Nop>
nnoremap <silent> guÞ <Nop>
nnoremap <silent> gUiÞ <Nop>
nnoremap <silent> gUaÞ <Nop>
nnoremap <silent> gUÞ <Nop>
xnoremap <silent> gÞ <Nop>
xnoremap <silent> g <Cmd>lua require("which-key").show("g", {mode = "v", auto = true})
xnoremap <expr> j v:count || mode(1)[0:1] == "no" ? "j" : "gj"
nnoremap <expr> j v:count || mode(1)[0:1] == "no" ? "j" : "gj"
xnoremap <expr> k v:count || mode(1)[0:1] == "no" ? "k" : "gk"
nnoremap <expr> k v:count || mode(1)[0:1] == "no" ? "k" : "gk"
xnoremap <silent> p p:let @+=@0:let @"=@0
nnoremap <silent> vÞ <Nop>
nnoremap <silent> v <Cmd>lua require("which-key").show("v", {mode = "n", auto = true})
nnoremap <silent> viÞ <Nop>
nnoremap <silent> vaÞ <Nop>
nnoremap <silent> yiÞ <Nop>
nnoremap <silent> yaÞ <Nop>
nnoremap <silent> yÞ <Nop>
nnoremap <silent> y <Cmd>lua require("which-key").show("y", {mode = "n", auto = true})
nnoremap <silent> zÞ <Nop>
nnoremap <silent> z <Cmd>lua require("which-key").show("z", {mode = "n", auto = true})
nnoremap <silent> zfiÞ <Nop>
nnoremap <silent> zfaÞ <Nop>
nnoremap <silent> zfÞ <Nop>
xnoremap <Plug>(comment_toggle_blockwise_visual) <Cmd>lua require("Comment.api").locked("toggle.blockwise")(vim.fn.visualmode())
xnoremap <Plug>(comment_toggle_linewise_visual) <Cmd>lua require("Comment.api").locked("toggle.linewise")(vim.fn.visualmode())
nnoremap <silent> <C-W>Þ <Nop>
nnoremap <silent> <C-W> <Cmd>lua require("which-key").show("\23", {mode = "n", auto = true})
nnoremap <silent> <SNR>75_Þ <Nop>
nnoremap <silent> <SNR>75Þ <Nop>
nnoremap <silent> <SNR>7Þ <Nop>
nnoremap <silent> <SNR>Þ <Nop>
nnoremap <silent> <SNR> <Cmd>lua require("which-key").show("��R", {mode = "n", auto = true})
nnoremap <SNR>75_: :=v:count ? v:count : ''
nnoremap <F5> <Cmd> DapContinue 
nnoremap <C-N> <Cmd> NvimTreeToggle 
tnoremap <C-L> l
tnoremap <C-J> j
tnoremap <C-K> k
tnoremap <C-H> h
tnoremap <C-X> 
nnoremap <C-C> <Cmd> %y+ 
nnoremap <C-H> h
nnoremap <expr> <Down> v:count || mode(1)[0:1] == "no" ? "j" : "gj"
nnoremap <expr> <Up> v:count || mode(1)[0:1] == "no" ? "k" : "gk"
nnoremap <C-K> k
nnoremap <C-S> <Cmd> w 
nnoremap <C-J> j
vnoremap <expr> <Down> v:count || mode(1)[0:1] == "no" ? "j" : "gj"
vnoremap <expr> <Up> v:count || mode(1)[0:1] == "no" ? "k" : "gk"
nnoremap <Plug>PlenaryTestFile :lua require('plenary.test_harness').test_file(vim.fn.expand("%:p"))
nnoremap <C-L> l
inoremap  ^i
inoremap  <Left>
inoremap <NL> <Down>
inoremap  <Up>
inoremap  <Right>
cnoremap <silent>  <Cmd>lua require("which-key").show("\18", {mode = "c", auto = true})
inoremap <silent>  <Cmd>lua require("which-key").show("\18", {mode = "i", auto = true})
inoremap  u
inoremap  u
let &cpo=s:cpo_save
unlet s:cpo_save
set clipboard=unnamedplus
set expandtab
set fillchars=eob:\ 
set guicursor=n-v-c:block,i-ci-ve:ver25,r-cr:hor20,o:hor50,a:blinkwait700-blinkoff400-blinkon250-Cursor/lCursor,sm:block-blinkwait175-blinkoff150-blinkon175
set guifont=SauceCodePro\ Nerd\ Font:h12
set helplang=en
set ignorecase
set indentkeys=0{,0},!^F,o,O,0[,0],0(,0)
set laststatus=3
set noloadplugins
set mouse=a
set packpath=/usr/share/nvim/runtime
set noruler
set runtimepath=~/.config/nvim,~/.local/share/nvim/site,~/.local/share/nvim/lazy/lazy.nvim,~/.local/share/nvim/lazy/Comment.nvim,~/.local/share/nvim/lazy/rust-tools.nvim,~/.local/share/nvim/lazy/crates.nvim,~/.local/share/nvim/lazy/rust.vim,~/.local/share/nvim/lazy/which-key.nvim,~/.local/share/nvim/lazy/vim-fugitive,~/.local/share/nvim/lazy/nvim-colorizer.lua,~/.local/share/nvim/lazy/indent-blankline.nvim,~/.local/share/nvim/lazy/gitsigns.nvim,~/.local/share/nvim/lazy/cmp-path,~/.local/share/nvim/lazy/cmp-buffer,~/.local/share/nvim/lazy/cmp-nvim-lsp,~/.local/share/nvim/lazy/cmp-nvim-lua,~/.local/share/nvim/lazy/cmp_luasnip,~/.local/share/nvim/lazy/nvim-autopairs,~/.local/share/nvim/lazy/friendly-snippets,~/.local/share/nvim/lazy/LuaSnip,~/.local/share/nvim/lazy/nvim-cmp,~/.local/share/nvim/lazy/dressing.nvim,~/.local/share/nvim/lazy/null-ls.nvim,~/.local/share/nvim/lazy/nvim-web-devicons,~/.local/share/nvim/lazy/trouble.nvim,~/.local/share/nvim/lazy/nvim-nio,~/.local/share/nvim/lazy/nvim-dap-ui,~/.local/share/nvim/lazy/nvim-lspconfig,~/.local/share/nvim/lazy/mason-lspconfig.nvim,~/.local/share/nvim/lazy/nvim-dap,~/.local/share/nvim/lazy/mason.nvim,~/.local/share/nvim/lazy/mason-nvim-dap.nvim,~/.local/share/nvim/lazy/alpha-nvim,~/.local/share/nvim/lazy/suda.vim,~/.local/share/nvim/lazy/copilot.vim,~/.local/share/nvim/lazy/nvim-treesitter-context,~/.local/share/nvim/lazy/vim-rainbow,~/.local/share/nvim/lazy/copilot.lua,~/.local/share/nvim/lazy/CopilotChat.nvim,~/.local/share/nvim/lazy/nvim-treesitter,~/.local/share/nvim/lazy/telescope.nvim,~/.local/share/nvim/lazy/plenary.nvim,~/.local/share/nvim/lazy/auto-session,~/.local/share/nvim/lazy/netrw.nvim,~/.local/share/nvim/lazy/base46,~/.local/share/nvim/lazy/nvterm,~/.local/share/nvim/lazy/ui,/usr/share/nvim/runtime,/usr/lib/nvim,~/.local/state/nvim/lazy/readme,~/.local/share/nvim/lazy/rust.vim/after,~/.local/share/nvim/lazy/cmp-path/after,~/.local/share/nvim/lazy/cmp-buffer/after,~/.local/share/nvim/lazy/cmp-nvim-lsp/after,~/.local/share/nvim/lazy/cmp-nvim-lua/after,~/.local/share/nvim/lazy/cmp_luasnip/after
set scrolloff=9
set shiftwidth=2
set shortmess=otIsTliOFfxn
set noshowmode
set showtabline=2
set smartcase
set smartindent
set softtabstop=2
set splitbelow
set splitright
set statusline=%!v:lua.require('nvchad.statusline.default').run()
set tabline=%!v:lua.require('nvchad.tabufline.modules').run()
set tabstop=2
set termguicolors
set timeoutlen=400
set undofile
set updatetime=250
set whichwrap=<>[]hl,b,s
set window=54
set winminheight=0
set winminwidth=0
set winwidth=1
" vim: set ft=vim :
