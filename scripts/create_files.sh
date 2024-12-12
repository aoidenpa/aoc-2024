
FOLDER="src"
INPUTS="input"

function create_day() {
    DAY=$1
    mkdir -p "$FOLDER/day$DAY"
    touch "$FOLDER/day$DAY/mod.rs"

    echo "pub mod day$DAY;" >> "$FOLDER/lib.rs"

    {
	echo "pub fn run(part: u32, input: String) -> String {" ;
	echo "    match part {"
	echo "        1 => {"
	echo "            part1(input)"
	echo "        },"
	echo "        2 => {"
	echo "            part2(input)"
	echo "        },"
	echo "        _ => panic!()"
	echo "    }";
	echo "}";

	echo "" ;
	
	echo "pub fn part1(input: String) -> String {" ;
	echo "    \"\".to_string()";
	echo "}";

	echo "" ;

	echo "pub fn part2(input: String) -> String {" ;
	echo "    \"\".to_string()";
	echo "}"; 
    } > "$FOLDER/day$DAY/mod.rs"
}


if [ $# -eq 0 ]
then
    for i in {3..25}
    do
        create_day "$i"
    done
else
        create_day "$1"
fi


