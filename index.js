const rust = import('./pkg/graemarch.js');

rust
    .then(m => m.greet())
    .catch(console.error);
