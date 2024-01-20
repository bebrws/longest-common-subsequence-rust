use std::array::IntoIter;

fn lcs<T>(x: T, y: T) -> Vec<Vec<<T as IntoIterator>::Item>>
where
    T: IntoIterator,
    T::Item: PartialEq + Default + Clone + std::fmt::Debug + std::fmt::Display,
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
                for vi in 0..table[i - 1][j].len() {
                    println!(
                        "Current values in table[i - 1][j][vi] {:?}",
                        table[i - 1][j][vi]
                    );
                    let mut temp = table[i - 1][j][vi].clone();
                    if table[i][j].len() == 0 {
                        table[i][j] = vec![temp];
                    } else {
                        table[i][j].push(temp);
                    }
                }

                for vi in 0..table[i][j - 1].len() {
                    println!(
                        "Current values in table[i][j-1][vi] {:?}",
                        table[i][j - 1][vi]
                    );
                    let mut temp = table[i][j - 1][vi].clone();
                    if table[i][j].len() == 0 {
                        table[i][j] = vec![temp];
                    } else {
                        table[i][j].push(temp);
                    }
                }
            }
            println!("After iter table is {:?}\n", table);
        }
    }
    return table[v1.len()][v2.len()].clone();
}

fn main() {
    println!(
        "Result: {:?}",
        lcs(vec!["a", "g", "c", "a", "t"], vec!["g", "a", "c"])
    );
}
