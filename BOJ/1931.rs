use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_ints()[0];
    let mut meetings: Vec<_> = (0..n).map(|_| read_ints()).collect();
    meetings.sort();

    let mut current_meeting = (0, 0);
    let mut meetings_available_count = 0;

    for meeting in &meetings {
        if current_meeting.1 <= meeting[0] {
            meetings_available_count += 1;
            current_meeting = (meeting[0], meeting[1]);
        } else if current_meeting.1 >= meeting[1] {
            current_meeting = (meeting[0], meeting[1]);
        }
    }

    print!("{meetings_available_count}");
}
