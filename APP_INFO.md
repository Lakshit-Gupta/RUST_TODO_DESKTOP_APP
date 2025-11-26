# 📝 Todo Desktop App

A beautiful and functional Todo List desktop application built with **Dioxus** and **Rust**.

## Features

✅ **Add Todos** - Create new tasks with ease  
✅ **Complete Todos** - Check off completed tasks  
✅ **Delete Todos** - Remove tasks you no longer need  
✅ **Filter Todos** - View All, Active, or Completed tasks  
✅ **Clear Completed** - Remove all completed tasks at once  
✅ **Beautiful UI** - Modern gradient design with smooth animations  
✅ **Task Counter** - See how many tasks remain  

## Technologies Used

- **Dioxus 0.7.1** - Cross-platform UI framework for Rust
- **RSX** - React-like syntax for Rust (instead of JSX)
- **CSS3** - Modern styling with gradients and animations
- **Rust** - Safe, fast, and reliable systems programming

## How It Works

The app uses Dioxus signals for reactive state management:

- `use_signal` - For managing todos, input, filter state
- `use_memo` - For computed values (filtered todos, remaining count)
- `rsx!` macro - For declarative UI components

## Running the App

```bash
dx serve
```

This will build and launch the desktop application.

## Project Structure

```
todo-app/
├── src/
│   └── main.rs          # Main application code
├── assets/
│   ├── main.css         # Styling
│   └── favicon.ico      # App icon
├── Cargo.toml           # Rust dependencies
└── Dioxus.toml          # Dioxus configuration
```

## Key Features in Code

### State Management
- Reactive signals for todos, input, and filters
- Memoized computed values for performance

### UI Components
- Input section with keyboard support (Enter to add)
- Filter buttons (All/Active/Completed)
- Todo list with hover effects
- Footer with task counter

### Styling
- Gradient backgrounds
- Smooth transitions and animations
- Responsive design
- Custom scrollbar styling

Enjoy your new Todo app! 🚀
