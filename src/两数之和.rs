fn main() {
    println!("{:?}", sum_tow(vec![1,3,3],6));

}
//  for (key,num) in nums.iter().enumerate() {
//             println!("key{}",key);
//         }
fn sum_tow(num: Vec<i32>, target: i32) -> Vec<i32> {
     for index in 0..num.len() {
         for index2 in index+1..num.len() {
             if num[index]+num[index2]==target{
             return  vec![index as i32,index2 as i32]
             }
         }
         }
    vec![]
}
