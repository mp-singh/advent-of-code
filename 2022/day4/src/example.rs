// pub fn part2(arr: &[&str]) -> i32 {
//     let result = arr
//         .iter()
//         .map(|elm| elm.split(',').collect::<Vec<_>>())
//         .map(|e| (e[0], e[1]))
//         .map(|(a, b)| {
//             (
//                 a.split('-').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>(),
//                 b.split('-').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>(),
//             )
//         })
//         .collect::<Vec<_>>();

//     let mut count = 0;
//     for (a, b) in &result {
//         // println!("{:?}", a);
//         let start_a = a[0];
//         let end_a = a[1];
//         let start_b = b[0];
//         let end_b = b[1];

//         if start_a == start_b || end_a == end_b || start_a == end_b || end_a == start_b {
//             count += 1;
//             continue;
//         } else if start_a <= start_b && end_a >= end_b {
//             count += 1;
//             continue;
//         } else if start_a >= start_b && end_a <= end_b {
//             count += 1;
//             continue;
//         } else if start_a <= start_b && end_a >= start_b {
//             count += 1;
//             continue;
//         } else if start_a <= end_b && end_a >= end_b {
//             count += 1;
//             continue;
//         } else if start_b <= start_a && end_b >= start_a {
//             count += 1;
//             continue;
//         } else if start_b <= end_a && end_b >= end_a {
//             count += 1;
//             continue;
//         }
//     }
//     count
// }



// pub fn part2(arr: &[&str]) -> i32 {
//     let result = arr
//         .iter()
//         .map(|elm| elm.split(',').collect::<Vec<_>>())
//         .map(|e| (e[0], e[1]))
//         .collect::<Vec<_>>();

//     let mut count = 0;
//     for (a, b) in &result {
//         let split_a: Vec<&str> = a.split('-').collect();
//         let split_b: Vec<&str> = b.split('-').collect();
//         let start_a = split_a[0].parse::<i32>().unwrap();
//         let end_a = split_a[1].parse::<i32>().unwrap();
//         let start_b = split_b[0].parse::<i32>().unwrap();
//         let end_b = split_b[1].parse::<i32>().unwrap();

//         if start_a == start_b || end_a == end_b || start_a == end_b || end_a == start_b {
//             count += 1;
//             break;
//         } else if start_a <= start_b && end_a >= end_b {
//             count += 1;
//             break;
//         } else if start_a >= start_b && end_a <= end_b {
//             count += 1;
//             break;
//         } else if start_a <= start_b && end_a >= start_b {
//             count += 1;
//             break;
//         } else if start_a <= end_b && end_a >= end_b {
//             count += 1;
//             break;
//         } else if start_b <= start_a && end_b >= start_a {
//             count += 1;
//             break;
//         } else if start_b <= end_a && end_b >= end_a {
//             count += 1;
//             break;
//         }
//     }
//     count
// }




// pub fn part1(arr: &[&str]) -> i32 {
//     let result = arr
//         .iter()
//         .map(|elm| elm.split(',').collect::<Vec<_>>())
//         .map(|e| (e[0], e[1]))
//         .collect::<Vec<_>>();

//     let mut count = 0;
//     for (a, b) in &result {
//         let split_a: Vec<&str> = a.split('-').collect();
//         let split_b: Vec<&str> = b.split('-').collect();
//         let start_a = split_a[0].parse::<i32>().unwrap();
//         let end_a = split_a[1].parse::<i32>().unwrap();
//         let start_b = split_b[0].parse::<i32>().unwrap();
//         let end_b = split_b[1].parse::<i32>().unwrap();

//         let length_a = end_a - start_a + 1;
//         let length_b = end_b - start_b + 1;

//         if length_a == length_b {
//             if start_a != start_b && end_a != end_b {
//                 continue;
//             } else {
//                 count += 1;
//             }
//         } else if length_a > length_b {
//             if start_a <= start_b && end_a >= end_b {
//                 count = count + 1;
//             }
//         } else {
//             if start_b <= start_a && end_b >= end_a {
//                 count = count + 1;
//             }
//         }
//     }

//     count
// }

// pub fn part3(arr: &[&str]) -> i32 {
//     let result = arr
//         .iter()
//         .map(|elm| elm.split(',').collect::<Vec<_>>())
//         .map(|e| {
//             (
//                 e[0].split('-')
//                     .map(|s| s.parse::<i32>().unwrap())
//                     .collect::<Vec<i32>>(),
//                 e[1].split('-')
//                     .map(|s| s.parse::<i32>().unwrap())
//                     .collect::<Vec<i32>>(),
//             )
//         })
//         .collect::<Vec<_>>();

//     let mut count = 0;
//     for (a, b) in &result {
//         if a[0] == b[0] || a[1] == b[1] || a[0] == b[1] || a[1] == b[0] {
//             count += 1;
//         } else if a[0] <= b[0] && a[1] >= b[1] {
//             count += 1;
//         } else if a[0] >= b[0] && a[1] <= b[1] {
//             count += 1;
//         } else if a[0] <= b[0] && a[1] >= b[0] {
//             count += 1;
//         } else if a[0] <= b[1] && a[1] >= b[1] {
//             count += 1;
//         } else if b[0] <= a[0] && b[1] >= a[0] {
//             count += 1;
//         } else if b[0] <= a[1] && b[1] >= a[1] {
//             count += 1;
//         }
//     }
//     count
// }
