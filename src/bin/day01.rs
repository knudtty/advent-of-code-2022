fn sum_calories(cals: &str) -> u32 {
    cals.lines().map(|l| l.parse::<u32>().unwrap()).sum()
}

fn main() {
    let elves = std::fs::read_to_string("src/day01.txt").unwrap();
    let elf_list: Vec<_> = elves.split("\n\n").collect();
    let elf_cal_list = elf_list.iter().fold(Vec::new(), |mut cals, elf_cal| {
        cals.push(sum_calories(elf_cal));
        cals
    });
    println!("Part A: {:?}", elf_cal_list.iter().max())
}
