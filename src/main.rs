fn main() {
    let input = "eyr:2028 iyr:2016 byr:1995 ecl:oth
pid:543685203 hcl:#c0946f
hgt:152cm
cid:252

hcl:#733820 hgt:155cm
iyr:2013 byr:1989 pid:728471979
ecl:grn eyr:2022

hgt:171cm
iyr:2013 pid:214368857 hcl:#cfa07d byr:1986 eyr:2028 ecl:grn

hgt:167cm cid:210 ecl:brn pid:429131951 hcl:#cfa07d eyr:2029 iyr:2010
byr:1945

hcl:#888785 iyr:2015
hgt:170cm pid:893805464 ecl:amb byr:1966 eyr:2028

hgt:170cm ecl:amb
hcl:#c0946f eyr:2020 iyr:2016 pid:725010548
byr:1928";

    println!("{}",input.split("\n\n").count())
}
