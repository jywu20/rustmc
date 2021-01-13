use crate::config::*;
use rand;
use rand::Rng;
use std::string::ToString;

pub struct IsingField {
    /// The list of sites in the lattice.
    /// The indexing rule is as follows:
    /// Suppose the site is at point (i, j), where i denotes the row and j denotes the column, then the index of the site is i * SIDE + j
    pub configuration: [i32;SITE_NUM],
    /// Commonly named `list` in many projects.
    pub coordinate_list: [[usize;2];SITE_NUM],
    /// Commonly named `invlist` in many projects
    pub index_list: [[usize;SIDE];SIDE],
    /// Commonly named `nnlist` in many projects
    pub neighbor_list: [[usize;8];SITE_NUM]
}

impl ToString for IsingField {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0 .. SIDE {
            for j in 0 .. SIDE {
                result.push_str(&(format!("{:^3}", self.configuration[self.index_list[i][j]])));
            }
            result.push('\n'); 
        }
        result
    }
}

impl IsingField {
    pub fn new() -> Self {
        let mut coordinate_list = [[0;2];SITE_NUM];
        let mut index_list = [[0;SIDE];SIDE]; 
        for i in 0 .. SIDE {
            for j in 0 .. SIDE {
                coordinate_list[i * SIDE + j][0] = i;
                coordinate_list[i * SIDE + j][1] = j;
                index_list[i][j] = i * SIDE + j;
            }
        }

        let mut configuration = [0;SITE_NUM];
        for i in 0 .. SITE_NUM {
            let choice = rand::thread_rng().gen_range(0..ISING_VALUES_NUM);
            configuration[i] = ISING_VALUES[choice];
        }

        let mut neighbor_list = [[0;8];SITE_NUM];
        for i in 0 .. SIDE {
            for j in 0 .. SIDE {
                neighbor_list[index_list[i][j]][0] = index_list[(i + SIDE - 1) % SIDE][j];
                neighbor_list[index_list[i][j]][1] = index_list[i][(j + 1) % SIDE];
                neighbor_list[index_list[i][j]][2] = index_list[(i + 1) % SIDE][j];
                neighbor_list[index_list[i][j]][3] = index_list[i][(j + SIDE - 1) % SIDE];
                neighbor_list[index_list[i][j]][4] = index_list[(i + SIDE - 1) % SIDE][(j + 1) % SIDE];
                neighbor_list[index_list[i][j]][5] = index_list[(i + 1) % SIDE][(j + 1) % SIDE];
                neighbor_list[index_list[i][j]][6] = index_list[(i + 1) % SIDE][(j + SIDE - 1) % SIDE];
                neighbor_list[index_list[i][j]][7] = index_list[(i + SIDE - 1) % SIDE][(j + SIDE - 1) % SIDE];
            }
        }

        Self {
            coordinate_list: coordinate_list,
            index_list: index_list,
            configuration,
            neighbor_list: neighbor_list
        }
    }
    #[inline]
    pub fn site_index_to_coordinate(&self, site_index: usize) -> (usize, usize) {
        (self.coordinate_list[site_index][0], self.coordinate_list[site_index][1])
    }
    // TODO: return type inconsistent
    #[inline]
    pub fn neighbor(&self, site: usize, index: usize) -> usize {
        self.neighbor_list[site][index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coordinate_list() {
        println!("Testing coordinate list.");
        let lattice = IsingField::new();
        for idx in 0 .. SITE_NUM {
            println!("{:?}", lattice.coordinate_list[idx]);
        }
    }

    #[test]
    fn test_neighbor_list_inner_point() {
        println!("Testing neighbor list.");
        let lattice = IsingField::new();
        
    }

    #[test]
    fn test_neighbor_list_edge() {

    }

    #[test]
    fn test_to_string() {
        let lattice = IsingField::new();
        println!("{:?}", lattice.configuration);
        println!("{}", lattice.to_string());
    }

}