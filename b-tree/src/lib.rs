// calculates minimum cost B-Tree based on sorted keys and corresponding access frequencies

#[derive(Debug)]
pub struct MinCostB3Tree<const N: usize> {
    keys: [u64; N],
    freq: [u64; N],
    sum_freq: [[u64; N]; N],
    opt_cost: [[u64; N]; N],
    root: [[(u64,u64); N]; N]
}

impl<const N: usize> MinCostB3Tree<N> {
    pub fn new(keys: [u64; N], freq: [u64; N]) -> Self {
        let mut sum_table = [[0 as u64; N]; N];
        for i in 0..N {
            sum_table[i][0] = freq[i];
            for j in 1..N {
                sum_table[i][j] = sum_table[i][j-1];
            }
        }
        MinCostB3Tree {
            keys,
            freq,
            sum_freq: sum_table,
            opt_cost: [[0; N]; N],
            root: [[(0,0); N]; N]
        }
    }

    pub fn calculate(&mut self) -> u64 {
        for row in 0..N {
            for col in row..N {
                let i = col - row; 
                let j = col; 
                
                self.opt_cost[i][j] = self.calc_opt_cost(i,j);
            }
        }
        self.opt_cost[0][N-1]
        
    }

    fn calc_opt_cost(&mut self, n: usize, m: usize) -> u64 {
        println!("({}, {})", n, m);
        if n > m {
            return 0; 
        } else if m == n {
            return self.freq[m];
        } else {
            let mut min_cost = u64::MAX;
            for i in n..=m {
                let mut first = 0;
                if i > n {
                    first = self.opt_cost[n][i-1];
                }

                let mut second = 0; 
                if i < m {
                    second = self.opt_cost[n][i+1];
                }
                let cost = self.sum_freq[n][m] + first + second;
                if cost < min_cost {
                    min_cost = cost
                }
            }
            println!("{}", min_cost);
            return min_cost;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_elements() {
        const size: usize = 5;

        let keys:[u64; size] = [0,1,2,3,4];
        let values:[u64; size] = [2,4,6,8,100];
        // println!("{:?}", MinCostB3Tree::<size>::new(keys, values));
        let mut b_tree = MinCostB3Tree::new(keys,values);
        println!("Min Cost B3Tree: {:?}", b_tree.calculate());
        println!("table: {:?}", b_tree.opt_cost);
    }
}
