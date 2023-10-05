# Inban (WIP)

Inban is a WIP cross-plattform app based on Tauri designed to support you with your houshold accounting.

## Features

- [] Manage spending statements in bank accounts and physical wallets
- [] Manage spending categories and budgets for categories
- [] Import bank statements from online banking and automatically or manually assign categories
- [] Allow automatic assignment of categories using regex
- [] Allow automatic tracking of money withdrawn from ATM machines and placed in a physical wallet

## Todo

### Backend: Rust

- [] Implement account types and storing/fetching of DB data
- [] Implement encrypted database storage
- [] Implement FinTS based import of bank statements

### Frontend: React.js/TypeScript

- [] Implement locking/unlocking of the app
- [] Implement display of account statements and balance, filterable by categories
- [] Implement display of graphical spending summaries based on categories
