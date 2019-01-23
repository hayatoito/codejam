use bit_vec::BitVec;
use codejam::{Scanner, Solve};
use ordered_float::OrderedFloat;

use std::collections::HashMap;
use std::default::Default;
use std::io::{Read, Result, Write};

// https://code.google.com/codejam/contest/32003/dashboard#s=p3
// Problem D. Shipping Plan

type Items = BitVec;
type Pos = usize;

type MustGoHome = bool;

type State = (Items, Pos, MustGoHome);
type Cache = HashMap<State, f64>;

#[derive(Debug)]
struct ShoppingPlan {
    items: Vec<Item>,
    stores: Vec<Store>,
    // dist: Array<f64, (usize, usize)>,
    dist: Vec<Vec<f64>>,
}

impl ShoppingPlan {
    fn new() -> ShoppingPlan {
        ShoppingPlan {
            dist: Default::default(),
            items: Default::default(),
            stores: Default::default(),
        }
    }

    fn new_with(price_of_gas: f64, items: Vec<Item>, mut stores: Vec<Store>) -> ShoppingPlan {
        stores.insert(
            0,
            Store {
                x: 0.0,
                y: 0.0,
                items: Vec::new(),
            },
        );
        ShoppingPlan {
            items,
            dist: {
                let n = stores.len();
                let mut a = vec![vec![0.0; n]; n];
                for i in 0..n {
                    for j in 0..n {
                        a[i][j] = stores[i].dist(&stores[j]) * price_of_gas
                    }
                }
                a
            },
            stores,
        }
    }

    fn doit2(&self, cache: &mut Cache, state: State) -> f64 {
        let (owned, pos, go_home) = state;
        let mut cands: Vec<f64> = self.stores[pos]
            .items
            .iter()
            .filter(|item| !owned.get(item.index).unwrap())
            .map(|item| {
                let mut new_owned = owned.clone();
                new_owned.set(item.index, true);
                item.price
                    + self.doit(
                        cache,
                        (new_owned, pos, go_home || self.items[item.index].perishable),
                    )
            })
            .collect();
        if go_home {
            assert!(pos != 0);
            cands.push(self.dist[pos][0] + self.doit(cache, (owned, 0, false)));
        } else {
            let mut cands1 = self
                .stores
                .iter()
                .enumerate()
                .flat_map(|(store_i, store)| {
                    store
                        .items
                        .iter()
                        .filter(|item| !owned.get(item.index).unwrap())
                        .map(|item| {
                            let mut new_owned = owned.clone();
                            new_owned.set(item.index, true);
                            item.price
                                + self.dist[pos][store_i]
                                + self.doit(
                                    cache,
                                    (new_owned, store_i, self.items[item.index].perishable),
                                )
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
            cands.append(&mut cands1);
        }
        cands.into_iter().map(OrderedFloat).min().unwrap().0
    }

    fn doit(&self, cache: &mut Cache, state: State) -> f64 {
        let (owned, pos, _) = state.clone();
        if owned.all() {
            self.dist[pos][0]
        } else if let Some(r) = cache.get(&state).cloned() {
            r
        } else {
            let r = self.doit2(cache, state.clone());
            cache.insert(state, r);
            r
        }
    }
}

#[derive(Debug)]
struct Item {
    name: String,
    perishable: bool,
}

#[derive(Debug)]
struct Store {
    x: f64,
    y: f64,
    items: Vec<StoreItem>,
}

impl Store {
    fn dist(&self, r: &Store) -> f64 {
        (self.x - r.x).hypot(self.y - r.y)
    }
}

#[derive(Debug)]
struct StoreItem {
    index: usize,
    price: f64,
}

impl Solve for ShoppingPlan {
    fn solve<R: Read, W: Write>(&mut self, read: R, write: &mut W) -> Result<()> {
        let mut s = Scanner::new(read);
        let n: usize = s.next();
        for i in 0..n {
            let num_items: usize = s.next();
            let num_stores: usize = s.next();
            let price_of_gas: f64 = s.next();
            let items = (0..num_items)
                .map(|_| {
                    let name: String = s.next();
                    if *name.as_bytes().last().unwrap() == b'!' {
                        Item {
                            name: std::str::from_utf8(&name.as_bytes()[0..(name.len() - 1)])
                                .unwrap()
                                .to_string(),
                            perishable: true,
                        }
                    } else {
                        Item {
                            name,
                            perishable: false,
                        }
                    }
                })
                .collect::<Vec<_>>();
            let stores = (0..num_stores)
                .map(|_| Store {
                    x: s.next(),
                    y: s.next(),
                    items: s
                        .next_to_end_of_line::<String>()
                        .iter()
                        .map(|store_item| {
                            let a = store_item.split(':').collect::<Vec<_>>();
                            let name = a[0].to_string();
                            StoreItem {
                                index: items.iter().position(|i| i.name == name).unwrap(),
                                price: a[1].parse().unwrap(),
                            }
                        })
                        .collect(),
                })
                .collect::<Vec<_>>();
            *self = ShoppingPlan::new_with(price_of_gas, items, stores);
            let r = self.doit(
                &mut Cache::new(),
                (BitVec::from_elem(num_items, false), 0, false),
            );
            writeln!(write, "Case #{}: {:.7}", i + 1, r)?;
        }
        Ok(())
    }
}

#[test]
fn test() {
    codejam::assert_output(
        ShoppingPlan::new(),
        "2008-practice/D-sample-practice.in",
        "2008-practice/D-sample-practice.expected",
    );
    codejam::assert_output(
        ShoppingPlan::new(),
        "2008-practice/D-small-practice.in",
        "2008-practice/D-small-practice.expected",
    );
    // gcj::assert_output(ShoppingPlan::new(),
    //                    "2008-practice/D-large-practice.in",
    //                    "2008-practice/D-large-practice.expected");
}

fn main() {
    env_logger::init();
    ShoppingPlan::new()
        .solve(std::io::stdin(), &mut std::io::stdout())
        .unwrap();
}
