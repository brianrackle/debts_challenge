use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use rand::Rng;

fn gen_order_groups(people_count: usize, meal_count: usize) -> Vec<Vec<(u8,bool)>> {
    let mut rng = rand::thread_rng();
    let mut order_groups : Vec<Vec<(u8,bool)>> = vec![vec![]; people_count];
    for order_group in &mut order_groups {
        for _meal_id in 0..meal_count-1 {
            order_group.push((rng.gen_range(8u8..25u8), false));
        }
    }

    for meal_id in 0..meal_count-1 {
        let payor = rng.gen_range(0..people_count);
        order_groups[payor][meal_id].1 = true;
    }

    order_groups
}

fn gen_balances(people_count: usize, meal_count: usize) -> Vec<i64> {
    //dont need two values can just .0 - .1 = balance
    let mut order_sums = vec![0i64;people_count];

    let order_groups = gen_order_groups(people_count, meal_count);
    for (i, order_group) in order_groups.iter().enumerate() {
        order_sums[i] = order_group.iter().fold(0, |acc, x| acc + x.0 as i64);
    }

    for i in 0..meal_count-1 {
        let day_cost = order_groups.iter().fold(0, |acc, x| acc + x[i].0 as i64);
        let payor_index = order_groups.iter().position(|x| x[i].1).expect("Data Construction Error: No payors found.");
        order_sums[payor_index] -= day_cost;
    }

    order_sums
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    //generate random debts and credits
    //iterate best possible solution (brute force)
    //log the n value, minimum number of transactions, ratio transactions / n, set of transactions and data set

    //let mut f = File::create("logs.txt").await?;
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let balances = gen_balances(20, rng.gen_range(30..300));
        println!("balances {:?}", balances);
        println!("balances sum {:?}", balances.iter().sum::<i64>())
    }

    // read up to 10 bytes
    // let n = f.read(&mut buffer[..]).await?;
    //
    // println!("The bytes: {:?}", &buffer[..n]);
    //
    // // Open a connection to the mini-redis address.
    // let mut client = client::connect("127.0.0.1:6379").await?;
    //
    // // Set the key "hello" with value "world"
    // client.set("hello", "world".into()).await?;
    //
    // // Get key "hello"
    // let result = client.get("hello").await?;
    //
    // println!("got value from the server; result={:?}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::gen_balances;

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
}