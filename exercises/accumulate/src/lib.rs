/// What should the type of _function be?
pub fn map<T,E,F>(input: Vec<T>,mut _function: F) -> Vec<E>
    where F:FnMut(T)->E, {
    let mut result = vec![];
    for x in input{
        result.push(_function(x));
    }
    result
}
