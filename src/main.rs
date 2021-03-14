use rand::Rng;

fn gen_order_groups(people_count: usize, meal_count: usize) -> Vec<Vec<(u8, bool)>> {
    let mut rng = rand::thread_rng();
    let mut order_groups: Vec<Vec<(u8, bool)>> = vec![vec![]; people_count];
    for order_group in &mut order_groups {
        for _meal_id in 0..meal_count - 1 {
            order_group.push((rng.gen_range(8u8..25u8), false));
        }
    }

    for meal_id in 0..meal_count - 1 {
        let payor = rng.gen_range(0..people_count);
        order_groups[payor][meal_id].1 = true;
    }

    order_groups
}

fn gen_balances(people_count: usize, meal_count: usize) -> Vec<i64> {
    let mut order_sums = vec![0i64; people_count];

    let order_groups = gen_order_groups(people_count, meal_count);
    for (i, order_group) in order_groups.iter().enumerate() {
        order_sums[i] = order_group.iter().fold(0, |acc, x| acc + x.0 as i64);
    }

    for i in 0..meal_count - 1 {
        let day_cost = order_groups.iter().fold(0, |acc, x| acc + x[i].0 as i64);
        let payor_index = order_groups.iter().position(|x| x[i].1).expect("Data Construction Error: No payors found.");
        order_sums[payor_index] -= day_cost;
    }

    order_sums
}

//each transaction balances one side to zero
fn balance_first_transaction(balances: &mut Vec<i64>, i: usize, j: usize) {
    balances[j] += balances[i];
    balances[i] = 0;
}

fn linear_transaction_balancing(mut balances: Vec<i64>) -> usize {
    let mut transactions = 0;
    for i in 0..balances.len() - 1 {
        if balances[i] != 0 {
            let mut j = i+1;
            while j < balances.len() && balances[j] == 0 {
                j += 1;
            }
            if balances[j] != 0 {
                balance_first_transaction(&mut balances, i, j);
                transactions += 1;
            }
        }
    }
    transactions
}

pub fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let balances = gen_balances(20, rng.gen_range(30..300));
        println!("balances {:?}", balances);
        println!("min transactions: {:?}", linear_transaction_balancing(balances));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn are_balances_balanced_2_people_1_meals() {
        let balances = gen_balances(2, 1);
        assert_eq!(balances.iter().sum::<i64>(), 0);
    }

    #[test]
    fn are_balances_balanced_5_people_1_meals() {
        let balances = gen_balances(5, 1);
        assert_eq!(balances.iter().sum::<i64>(), 0);
    }

    #[test]
    fn are_balances_balanced_2_people_30_meals() {
        let balances = gen_balances(2, 30);
        assert_eq!(balances.iter().sum::<i64>(), 0);
    }

    #[test]
    fn are_balances_balanced_5_people_30_meals() {
        let balances = gen_balances(5, 30);
        assert_eq!(balances.iter().sum::<i64>(), 0);
    }

    #[test]
    fn are_transactions_balanced_0_0() {
        let mut balances = vec![0, 0];
        balance_first_transaction(&mut balances, 0, 1);
        assert_eq!(balances[0], 0);
        assert_eq!(balances[1], 0);
    }

    #[test]
    fn are_transactions_balanced_3_4() {
        let mut balances = vec![3, 4];
        balance_first_transaction(&mut balances, 0, 1);
        assert_eq!(balances[0], 0);
        assert_eq!(balances[1], 7);
    }

    #[test]
    fn are_transactions_balanced_n3_2() {
        let mut balances = vec![-3, 2];
        balance_first_transaction(&mut balances, 0, 1);
        assert_eq!(balances[0], 0);
        assert_eq!(balances[1], -1);
    }
}