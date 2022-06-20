pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut rt = vec![];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            //行最大
            if input[i].iter().all(|&x| x<= input[i][j]) {
                //列最小
                if input.iter().map(|x|x[j]).all(|x|x>=input[i][j]){
                    rt.push((i,j));
                }
            }
        }
    }
    rt
}
