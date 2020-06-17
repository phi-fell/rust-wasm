awk -F '= ' '/BUILD_NUMBER: u32/{$2=$2+1";"}1' OFS='= ' src/version.rs > src/version_next.rs
mv src/version_next.rs src/version.rs
