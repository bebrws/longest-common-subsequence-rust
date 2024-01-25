use itertools::Itertools;
use std::array::IntoIter;
use std::collections::HashSet;
use wasm_bindgen::JsValue;
use wasm_react::hooks::use_state;
use wasm_react::{export_components, h, Component, VNode};

struct LCSTable {}

impl Component for LCSTable {
    fn render(&self) -> VNode {
        let row = use_state(|| 1);

        let mut children = VNode::new();
        for i in 0..*row.value() {
            children.push(&h!(td).build(i));
        }
        let t = h!(table).build(h!(tr).build(children));
        t
    }
}

struct App;

impl Component for App {
    fn render(&self) -> VNode {
        h!(div).build((LCSTable {}.build(),))
    }
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(_: JsValue) -> Result<Self, Self::Error> {
        Ok(App)
    }
}

export_components! { App }

fn lcs<T>(x: T, y: T) -> Vec<Vec<<T as IntoIterator>::Item>>
where
    T: IntoIterator,
    T::Item:
        PartialEq + Eq + Default + Clone + std::fmt::Debug + std::fmt::Display + std::hash::Hash,
{
    let v1: Vec<T::Item> = x.into_iter().collect();
    let v2: Vec<T::Item> = y.into_iter().collect();
    let mut table: Vec<Vec<Vec<Vec<T::Item>>>> =
        vec![vec![vec![vec![T::Item::default(); 0]; 0]; v2.len() + 1]; v1.len() + 1];

    for j in 1..=v2.len() {
        for i in 1..=v1.len() {
            println!("cmp {} {}", v2[j - 1], v1[i - 1]);
            if v1[i - 1] == v2[j - 1] {
                println!("got match at {} {} for {}", i, j, v1[i - 1]);
                let mut temp = table[i - 1][j - 1].clone();
                temp.iter_mut().for_each(|x| x.push(v1[i - 1].clone()));
                if temp.len() == 0 {
                    println!("New match");
                    table[i][j] = vec![vec![v1[i - 1].clone()]];
                } else {
                    table[i][j] = temp;
                }
            } else {
                let mut temp = table[i][j].clone();
                let mut l1 = table[i - 1][j].clone();
                let mut l2 = table[i][j - 1].clone();
                let l1_longest = l1
                    .iter()
                    .fold(0, |acc, x| if x.len() > acc { x.len() } else { acc });
                let l2_longest = l2
                    .iter()
                    .fold(0, |acc, x| if x.len() > acc { x.len() } else { acc });
                if l1_longest == l2_longest {
                    temp.append(&mut l1);
                    temp.append(&mut l2);
                } else if l1_longest > l2_longest {
                    temp.append(&mut l1);
                } else {
                    temp.append(&mut l2);
                }
                println!("Loading from diagonal {:?}", temp);
                table[i][j] = temp;
            }
            println!("After iter table is {:?}\n", table);
        }
    }
    // let with_duplicates = table[v1.len()][v2.len()].clone();
    let mut set = HashSet::new();
    table[v1.len()][v2.len()].iter().for_each(|val| {
        set.insert(val.clone());
    });
    return set.into_iter().collect();
}

// fn main() {
//     println!(
//         "Result: {:?}",
//         lcs(vec!["a", "g", "c", "a", "t"], vec!["g", "a", "c"])
//     );
// }
