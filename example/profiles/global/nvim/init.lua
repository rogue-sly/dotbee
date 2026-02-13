-- 1. Basic Settings
vim.g.mapleader = " " -- Use space as leader key
vim.opt.number = true -- Show line numbers
vim.opt.relativenumber = true -- Relative numbers for easier jumping
vim.opt.mouse = "a" -- Enable mouse support
vim.opt.ignorecase = true -- Smart case searching
vim.opt.smartcase = true
vim.opt.shiftwidth = 4 -- 4 spaces per tab
vim.opt.expandtab = true
vim.opt.termguicolors = true -- Better colors

-- 2. Bootstrap Lazy.nvim (Plugin Manager)
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.uv.fs_stat(lazypath) then
    vim.fn.system({
        "git",
        "clone",
        "--filter=blob:none",
        "https://github.com/folke/lazy.nvim.git",
        "--branch=stable",
        lazypath,
    })
end
vim.opt.rtp:prepend(lazypath)

-- 3. Install Plugins
require("lazy").setup({
    -- Colorscheme
    { "catppuccin/nvim", name = "catppuccin", priority = 1000 },

    -- Syntax Highlighting
    { "nvim-treesitter/nvim-treesitter", build = ":TSUpdate" },

    -- Fuzzy Finder (The "Everything" Searcher)
    {
        "nvim-telescope/telescope.nvim",
        tag = "0.1.5",
        dependencies = { "nvim-lua/plenary.nvim" },
    },

    -- Simple File Explorer
    { "nvim-tree/nvim-web-devicons" },
})

-- 4. Plugin Configurations & Keymaps
vim.cmd.colorscheme("catppuccin-mocha")

local builtin = require("telescope.builtin")
vim.keymap.set("n", "<leader>ff", builtin.find_files, { desc = "Find Files" })
vim.keymap.set("n", "<leader>fg", builtin.live_grep, { desc = "Live Grep" })
vim.keymap.set("n", "<leader>fb", builtin.buffers, { desc = "Find Buffers" })

-- Treesitter Setup
require("nvim-treesitter.configs").setup({
    ensure_installed = { "lua", "rust", "javascript", "python" },
    highlight = { enable = true },
})
