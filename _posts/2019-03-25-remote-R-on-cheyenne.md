---
layout: post
title: "R with Remote Session"
description: "Setting up a remote R session within a terminal"
tags: R
giscus_comments: true
date: 2019-03-25T11:00:00
related_posts: false
---

<!-- vim-markdown-toc GFM -->

- [Description](#description)
- [Caveat](#caveat)
- [Appendix - vimrc](#appendix---vimrc)

<!-- vim-markdown-toc -->

## Description

I used to write and execute codes on my laptop. But recently I have been more frequently working with supercomputers and remote servers. HPCs and towers are definitely much powerful than my laptop. Therefore, I would like to have a way to easily program on those plotforms, for example, in R.

*If you have root permission on the remote server*, I recommend using [Rstudio server](https://www.rstudio.com/products/rstudio-server/). You can get a recoverable browser-based interface for R programing on the remote server.

*If you work with supercomputers*, for example [Cheyenne](https://www2.cisl.ucar.edu/resources/computational-systems/cheyenne), the goto solution is to start a remote desktop with VNC as instructed [here](https://www2.cisl.ucar.edu/resources/computational-systems/casper/software/turbovnc). However, a DAV allocation is required for this to work.

Based on my personal preferences for the VI-style, there is another solution with the following tools. **I have to be honest, VIM is hard to configure and use. If you are not comfortable working with VIM, please avoid this.**

- [Linux Homebrew](https://docs.brew.sh/Homebrew-on-Linux) for installing NeoVim on Cheyenne within my user space;
- [NeoVim](https://neovim.io/) as the primary IDE for working with R scripts;
- [Vundle.vim](https://github.com/VundleVim/Vundle.vim) is the plug-in manager for NeoVim;

**Note: When you configure the file `vimrc`, the file to create/modify is not the usual `~/.vimrc` which is for VIM, but `.config/nvim/init.vim` which is for NeoVim.**

- [Nvim-R](https://github.com/jalvesaq/Nvim-R) as the plug-in to support editting R code. Usage of this plug-in can be found [here](https://raw.githubusercontent.com/jalvesaq/Nvim-R/master/doc/Nvim-R.txt);
- [rmote](https://github.com/cloudyr/rmote) as the R package which provides remote render of R visualization;

With this solution, I can use a VI-style editor on Cheyenne to work with an R session and at the same time see the visualization locally using a browser.

Installation for these tools are detailed in their documentation. After all the tools are installed, here is what I do each time to set up my environment for working with R.

```
# Log onto Cheyenne with dedicated port number as indicated by rmote
ssh -L 4321:localhost:4321 -L 8100:localhost:8100 wuh20@cheyenne.ucar.edu

# Load modules for R
module purge && module load R/3.5.2

# Start NeoVim and coding
nvim script.R

#
# Start a browser locally to see visulization
# at localhost:4321
#
```

This is what my developing environment looks like.

<div class="row mt-6">
    <div class="col-sm mt-6 mt-md-0">
        {% include figure.html path="assets/data-for-posts/2019-03-25-remote-R-on-cheyenne/remote-R.png" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>


## Caveat

I do noticed some possible issues related with the compatibility on Cheyenne with LinuxBrew. These problems might just be relevant to Cheyenne.

- Man page does not work with LinuxBrew directory added to `PATH`. Every time when I type `man cp` I will receive a segmentation fault. So my solution is, since I just need `NeoVim` to function, I removed the directory of LinuxBrew from `PATH` and added an alias for `NeoVim` specifically.
- System default environment is changed. The most notable difference is that the system default loaded modules are changed. I resolved this by adding `module purge && module load [packages]` to `.bash_profile` to load the default modules manually.

## Appendix - vimrc

This is my final configuration file for NeoVim for reference.

```
" be iMproved, required
set nocompatible              

" required
filetype off                  

" set the runtime path to include Vundle and initialize
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()
" alternatively, pass a path where Vundle should install plugins
"call vundle#begin('~/some/path/here')

" let Vundle manage Vundle, required
Plugin 'VundleVim/Vundle.vim'

" Molokai color scheme
Plugin 'tomasr/molokai'

" For working with R
Plugin 'jalvesaq/Nvim-R'

" Tree
Plugin 'scrooloose/nerdtree'
Plugin 'xuyuanp/nerdtree-git-plugin'

" Indent guide
Plugin 'nathanaelkane/vim-indent-guides'

" All of your Plugins must be added before the following line
call vundle#end()            " required

" better and easier indentation
filetype plugin indent on    " required
filetype plugin on
filetype indent on

" To ignore plugin indent changes, instead use:
"filetype plugin on
"
" Brief help
" :PluginList       - lists configured plugins
" :PluginInstall    - installs plugins; append `!` to update or just :PluginUpdate
" :PluginSearch foo - searches for foo; append `!` to refresh local cache
" :PluginClean      - confirms removal of unused plugins; append `!` to auto-approve removal
"
" see :h vundle for more details or wiki for FAQ
" Put your non-Plugin stuff after this line

let mapleader = ','

" easy navigation
if has('macunix')
    nnoremap ∆ <C-W>j
    nnoremap ˚ <C-W>k
    nnoremap ˙ <C-W>h
    nnoremap ¬ <C-W>l
    nnoremap <Leader>= <C-W>+
    nnoremap <Leader>- <C-W>-
    nnoremap <Leader>. <C-W>>
    nnoremap <Leader>, <C-W><
else
    nnoremap <A-j> <C-W>j
    nnoremap <A-k> <C-W>k
    nnoremap <A-h> <C-W>h
    nnoremap <A-l> <C-W>l
endif

nnoremap <leader>j <C-W><S-j>
nnoremap <leader>k <C-W><S-k>
nnoremap <leader>l <C-W><S-l>
nnoremap <leader>h <C-W><S-h>

" easy tab navigation
nnoremap <leader>1 1gt
nnoremap <leader>2 2gt
nnoremap <leader>3 3gt
nnoremap <leader>4 4gt
nnoremap <leader>5 5gt
nnoremap <leader>6 6gt
nnoremap <leader>7 7gt
nnoremap <leader>8 8gt
nnoremap <leader>9 9gt

" move lines easily
if has('macunix')
    inoremap <D-j> <Esc>:m .+1<CR>==gi
    inoremap <D-k> <Esc>:m .-2<CR>==gi
    vnoremap <D-j> :m '>+1<CR>gv=gv
    vnoremap <D-k> :m '<-2<CR>gv=gv
    nnoremap <D-j> :m .+1<CR>==
    nnoremap <D-k> :m .-2<CR>==
else
    inoremap <C-j> <Esc>:m .+1<CR>==gi
    inoremap <C-k> <Esc>:m .-2<CR>==gi
    vnoremap <C-j> :m '>+1<CR>gv=gv
    vnoremap <C-k> :m '<-2<CR>gv=gv
    nnoremap <C-j> :m .+1<CR>==
    nnoremap <C-k> :m .-2<CR>==
endif

" basic settings
colorscheme molokai
set guifont=Monaco:h14
set gcr=a:block-blinkon0
set foldmethod=indent
set foldlevelstart=99
set background=dark
set encoding=utf-8
set relativenumber
set softtabstop=4
set cursorcolumn
set shiftwidth=4
set noignorecase
set nocompatible
let $LANG = 'en'
set ignorecase
set lazyredraw
set cursorline
set incsearch
set expandtab
set tabstop=4
set hlsearch
set autoread
set nowrap
set ruler
set noeb
set cul!


noremap <F2> :vsplit $HOME/.config/nvim/init.vim<CR>
autocmd FileType r inoremap ,- <-

" NERDTree settings
map <F3> :NERDTreeMirror<CR>
map <F3> :NERDTreeToggle<CR>

" Suggested setup for Nvim R

" Use Ctrl+Space to do omnicompletion:
if has('nvim') || has('gui_running')
autocmd FileType r inoremap ,- <-
    inoremap <Tab> <C-x><C-o>
else
autocmd FileType r inoremap ,- <-
    inoremap <Nul> <C-x><C-o>
endif

" Press the space bar to send lines and selection to R:
vmap <Space> <Plug>RDSendSelection
nmap <Space> <Plug>RDSendLine
```
