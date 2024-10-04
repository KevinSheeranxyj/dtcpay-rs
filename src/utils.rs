

pub fn main() {
    let mut numbers: [u8; 4] = [1, 3, 33, 44];
    {
        let all: &[u8] = &numbers[..];
        println!("All of them: {:?}", all);
    }
    {
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[1] = 99;
        first_two[0] = 22;
    }

    println!("Look ma! I can modify through slices: {:?}", numbers);
}
