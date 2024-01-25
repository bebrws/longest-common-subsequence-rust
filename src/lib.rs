use core::fmt;
use itertools::Itertools;
use std::array::IntoIter;
use std::collections::HashSet;
use wasm_bindgen::JsValue;
use wasm_react::hooks::{use_state, Deps};
use wasm_react::{clones, export_components, h, Callback, Component, VNode};

struct LCSTable {
    v1: Vec<String>,
    v2: Vec<String>,
    table: Vec<Vec<Vec<Vec<String>>>>,
    result: Vec<Vec<String>>,
    row: usize,
}

impl Component for LCSTable {
    fn render(&self) -> VNode {
        let row = use_state(|| self.row);
        let table = use_state(|| self.table.clone());
        let result = use_state(|| self.result.clone());

        let button = h!(button)
            .on_click(&Callback::new({
                clones!(self.v1, self.v2, mut table, mut result, mut row);
                move |_| {
                    row.set(|c| c + 1);
                    let result_and_table = lcs(
                        v1.clone(),
                        v2.clone(),
                        table.value().clone(),
                        *row.value() + 1,
                    );
                    match result_and_table {
                        (Some(result_from_lcs), new_table) => {
                            table.set(|_| new_table);
                            result.set(|_| result_from_lcs);
                            // println!("Result: {:?}", result);
                        }
                        (None, new_table) => {
                            table.set(|_| new_table);
                        }
                    }
                }
            }))
            .build(format!("Increment {}", row.value()));

        let mut top_row = VNode::new();
        top_row.push(&h!(th).build("[]"));
        top_row.push(&h!(th).build("[]"));
        for i in 0..self.v1.len() {
            let val = self.v1[i].clone();
            top_row.push(&h!(th).build(val));
        }

        let mut rows = VNode::new();
        rows.push(&h!(thead).build(h!(tr).build(top_row)));

        let mut second_row = VNode::new();
        second_row.push(&h!(td).build("[]"));
        second_row.push(&h!(td).build("[]"));
        for i in 0..self.v1.len() {
            second_row.push(&h!(td).build("[]"));
        }

        rows.push(&h!(tr).build(second_row));

        for j in 0..self.v2.len() {
            let mut next_row = VNode::new();
            for i in 0..self.v1.len() {
                if i == 0 {
                    next_row.push(&h!(td).build(self.v2[j].clone()));
                    next_row.push(&h!(td).build("[]"));
                }
                next_row.push(&h!(td).build(format!("{:?}", table.value()[i][j])));
            }
            rows.push(&h!(tr).build(next_row));
        }

        let t = h!(table).build(rows);

        let comp = h!(div).build((t, button));
        comp
    }
}

struct App;

impl Component for App {
    fn render(&self) -> VNode {
        let v1 = vec!["g".to_string(), "a".to_string(), "c".to_string()];
        let v2 = vec![
            "a".to_string(),
            "g".to_string(),
            "c".to_string(),
            "a".to_string(),
            "t".to_string(),
        ];
        let mut table: Vec<Vec<Vec<Vec<String>>>> =
            vec![vec![vec![vec![String::default(); 0]; 0]; v2.len() + 1]; v1.len() + 1];

        h!(div).build((LCSTable {
            v1: v1,
            v2: v2,
            table: table,
            result: vec![],
            row: 1,
        }
        .build(),))
    }
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(_: JsValue) -> Result<Self, Self::Error> {
        Ok(App)
    }
}

export_components! { App }

fn lcs<T>(
    x: T,
    y: T,
    old_table: Vec<Vec<Vec<Vec<T::Item>>>>,
    row: usize,
) -> (
    Option<Vec<Vec<<T as IntoIterator>::Item>>>,
    Vec<Vec<Vec<Vec<T::Item>>>>,
)
where
    T: IntoIterator,
    T::Item:
        PartialEq + Eq + Default + Clone + std::fmt::Debug + std::fmt::Display + std::hash::Hash,
{
    let v1: Vec<T::Item> = x.into_iter().collect();
    let v2: Vec<T::Item> = y.into_iter().collect();
    // let mut table: Vec<Vec<Vec<Vec<T::Item>>>> =
    // vec![vec![vec![vec![T::Item::default(); 0]; 0]; v2.len() + 1]; v1.len() + 1];
    let mut table = old_table;

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

    if row == v2.len() {
        // let with_duplicates = table[v1.len()][v2.len()].clone();
        let mut set = HashSet::new();
        table[v1.len()][v2.len()].iter().for_each(|val| {
            set.insert(val.clone());
        });
        return (Some(set.into_iter().collect()), table);
    } else {
        return (None, table);
    }
}

// fn main() {
//     println!(
//         "Result: {:?}",
//         lcs(vec!["a", "g", "c", "a", "t"], vec!["g", "a", "c"])
//     );
// }
