use std::fmt::Debug;

//Test:
// const MIN_X: i32 = 20;
// const MAX_X: i32 = 30;

// const MIN_Y: i32 = -10;
// const MAX_Y: i32 = -5;

//Actual:
const MIN_X: i32 = 85;
const MAX_X: i32 = 145;

const MIN_Y: i32 = -163;
const MAX_Y: i32 = -108;

#[derive(Debug, Copy, Clone)]
struct Possible {
    value: i32,
    min_t: i32,
    max_t: i32
}

impl Default for Possible {
    fn default() -> Possible { 
        Possible { 
            value: 0,
            min_t: 0,
            max_t: i32::MAX,
        }
    }
}

fn main() {
    /* At any time t, horizontal distance increases by x and x reduces by 1 till 0.
     * At any time t, vertical distance decreases by y and y reduces by 1 till neg inf.
     */

    /* for a value of x from 0 to MAX_X (because we cannot reverse with pos. distance):
        for a value of t from 0 to MAX_X (because assumption is that if x == 1, we will cross MAX_X at time t = MAX_X)
            create the possible set. */

    let mut possible_xs = vec!();

    for x in 1..MAX_X {
        let mut min_t = 0;
        let mut max_t = 0;
        let mut found = false;

        let mut possible = Possible::default();

        for t in 0..=x {
            let distance = t * x - t * (t - 1)/2;
            
            if distance >= MIN_X && distance <= MAX_X {
                if !found {
                    min_t = t;
                    max_t = t;
                    found = true;
                }
                else {
                    max_t = t;
                }
            }

            if distance >= MIN_X && distance <= MAX_X && t == x {
                println!("Possible: x: {}, min_t: {}, max_t: {}", x, min_t, max_t);
                max_t = i32::MAX;
            }

            if found {
                //println!("Possible: x: {}, min_t: {}, max_t: {}", x, min_t, max_t);
                possible = Possible {value: x, min_t: min_t, max_t: max_t};
            }
        }

        if found {
            possible_xs.push(possible);
        }
    }
    
    println!("{:?}", possible_xs);

    let mut possible_ys = vec!();

    for y in 0..-MIN_Y {
        println!("Y: {}", y);
        for t in 1..i32::MAX {
            let distance = y * t - (t * (t - 1)) / 2;
            if distance >= MIN_Y && distance <= MAX_Y {
                println!("Possible: y: {}, t: {}", y, t);
                let possible = Possible{value: y, min_t: t, max_t: t};
                possible_ys.push(possible);
            }

            if distance < MIN_Y {
                break;
            }
        }
    }
}

/* I solved this by looking at the 2 possible_xs and possible_ys. If there exists a dx s.t. it ends
* in the target area with a velocity 0 after time t, then the largest starting value of y that crosses the 
* target area with any time >= t will solve part 1.
*
* I wrote the next code to verify that it does touch the target area. We can solve for the value of max. 
* height by observing at for a value of initial value y, it will hit velocity 0 at time t = y.
* Max height = y * y - y * (y - 1) /2. For value of 162 it will be equal to 13203 which is the right answer.
*/
#[test]
fn test_part1() {
    let mut x = 0;
    let mut y = 0;
    let mut del_x = 13;
    let mut del_y = 162;

    for _ in 0..1000 {
        x += del_x;
        if del_x > 0 {
            del_x -= 1;
        }
        y += del_y;
        del_y -= 1;

        println!("Y: {}", y);

        if x >= MIN_X && x <= MAX_X && y >= MIN_Y && y <= MAX_Y {
            println!("Reached!: X:{}, Y:{}", x, y);
            break;
        }
    }

    println!("Part 1: {}", 162 * 162 - (162 * 161) / 2);
}
