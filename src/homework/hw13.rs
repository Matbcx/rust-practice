use std::collections::BTreeMap;

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut events = vec![];

    for rect in xs {
        let (x1, x2) = (rect.a.x.min(rect.b.x), rect.a.x.max(rect.b.x));
        let (y1, y2) = (rect.a.y.min(rect.b.y), rect.a.y.max(rect.b.y));
        events.push((x1, 1, y1, y2));
        events.push((x2, -1, y1, y2));
    }

    events.sort();
    let mut active: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    let mut area = 0;
    let mut prev_x = events[0].0;

    for &(x, typ, y1, y2) in &events {
        let mut intervals: Vec<(i32, i32)> = active
            .iter()
            .filter(|(_, v)| **v > 0)
            .map(|(&(a, b), _)| (a, b))
            .collect();
        intervals.sort();

        let mut height = 0;
        let mut cur_start = -1;
        let mut cur_end = -1;

        for (a, b) in intervals {
            if a > cur_end {
                height += cur_end - cur_start;
                cur_start = a;
                cur_end = b;
            } else {
                cur_end = cur_end.max(b);
            }
        }
        height += cur_end - cur_start;
        area += height * (x - prev_x);
        prev_x = x;

        let entry = active.entry((y1, y2)).or_insert(0);
        *entry += typ;
        if *entry == 0 {
            active.remove(&(y1, y2));
        }
    }

    area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Фактична зайнята площа: {}", area_occupied(&test_data()));
}
