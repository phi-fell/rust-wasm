const rust = import('./pkg');

rust
    .then(m => m.greet())
    .catch(console.error);
