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

for i in $(seq 1 25); do
    DAY=$(printf %02d $i)
    FILE="src/years/y$YEAR/day$DAY.rs"
    echo " - Day $DAY ";
    echo "$PART_A" > $FILE;
    if [ "$i" != 25 ]; then
        echo "$PART_B" >> $FILE
    fi
done;

sed -i "s#// insert: new year create#macro_create_year_mod!($YEAR);\n// insert: new year create#" src/years.rs
sed -i "s#// insert: new year run#$YEAR => y$YEAR::run(day, part, input),\n        // insert: new year run#" src/years.rs
