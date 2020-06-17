awk -F '= ' '/BUILD_NUMBER: u32/{$2=$2+1";"}1' OFS='= ' src/build_number.rs > src/build_number_next.rs
mv src/build_number_next.rs src/build_number.rs
