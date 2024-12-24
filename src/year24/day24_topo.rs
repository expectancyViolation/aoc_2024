// fn get_topo_sort(gates: &Vec<crate::year24::day24_::Day24Gate>, values: &Vec<crate::year24::day24_::Value>) -> Option<Vec<usize>> {
//     let mut res = vec![];
//     let mut seen = vec![false; gates.len()];
//     for i in 0..values.len() {
//         if gates[i].arg1 == usize::MAX {
//             seen[i] = true;
//             //res.push(i);
//         }
//         if i >= values.len() - crate::year24::day24_::OUTPUT_BITS {
//             seen[i] = true;
//             res.push(i);
//         }
//     }
//     for i in (0..gates.len()).rev() {
//         if seen[i] {
//             continue;
//         }
//         let mut stack = vec![i];
//         while !stack.is_empty() {
//             if stack.len() > values.len() {
//                 // unsortable
//                 return None;
//             }
//             let last = *stack.last().unwrap();
//             if seen[last] {
//                 stack.pop();
//                 continue;
//             }
//             let arg1 = gates[last].arg1;
//             if !seen[arg1] {
//                 stack.push(arg1);
//                 continue;
//             }
//             let arg2 = gates[last].arg2;
//             if !seen[arg2] {
//                 stack.push(arg2);
//                 continue;
//             }
//             res.push(last);
//             seen[last] = true;
//         };
//     }
//     Some(res)
// }
