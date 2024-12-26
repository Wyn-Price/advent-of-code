#!/usr/bin/env bash

YEAR=${1:-$(date +%Y)}

echo "Creating $YEAR";

mkdir "src/years/y$YEAR"

PART_A=$(cat <<EOF
pub fn part_a(input: &str) -> i64 {
    panic!("Part A not implemented yet");
}
EOF
)

PART_B=$(cat <<EOF
pub fn part_b(input: &str) -> i64 {
    panic!("Part B not implemented yet");
}
EOF
)

PART_B_25=$(cat <<EOF
pub fn part_b(input: &str) -> String {
    return "0".to_owned();
}
EOF
)

for i in $(seq 1 25); do
    DAY=$(printf %02d $i)
    case "$i" in
        25) PRINT_B=$PART_B_25 ;;
        *)  PRINT_B=$PART_B ;;
    esac
    echo " - Day $DAY ";
    cat <<EOF > src/years/y$YEAR/day$DAY.rs
$PART_A

$PRINT_B
EOF
done;

sed -i "s#// insert: new year create#macro_create_year_mod!($YEAR);\n// insert: new year create#" src/years.rs
sed -i "s#// insert: new year run#$YEAR => y$YEAR::run(day, part, input),\n        // insert: new year run#" src/years.rs
