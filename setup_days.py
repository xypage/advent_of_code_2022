for i in range(6, 26):
    label = f"{i:02}"
    with open(f"./input/day{label}", "w") as f:
        f.write("")
    with open(f"./input/day{label}-test", "w") as f:
        f.write("first_test_output---second_test_output\ntest_input")
    with open(f"./src/solutions/day{label}.rs", "w") as f:
        f.writelines(
            [
                "use crate::solutions::Solution;\n\n",
                f"pub struct Day{label};\n",
                f"impl Solution for Day{label} {'{'}\n",
                "    #[allow(unused_variables)]\n",
                "    fn part1(&self, _input: &String) -> String {\n",
                '        String::from("Part 1")\n',
                "    }\n\n",
                "    #[allow(unused_variables)]\n",
                "    fn part2(&self, _input: &String) -> String {\n",
                '        String::from("Part 2")\n',
                "    }\n",
                "}",
            ]
        )
