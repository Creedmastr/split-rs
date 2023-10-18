pub fn split_vec(vec: Vec<u8>, nb: usize) -> Vec<Vec<u8>> {
    let mut index: u128 = 0;
    let mut result = vec![];
    let mut medium_vec = vec![];

    let area_split = vec.len() / nb;

    for i in vec {
        match index == (area_split - 1).try_into().unwrap() {
            true => {
                index = 0;
                result.push(medium_vec);
                medium_vec = vec![];
                medium_vec.push(i)
            }

            false => {
                index += 1;
                medium_vec.push(i);
            }
        }
    }

    if !medium_vec.is_empty() {
        result.push(medium_vec);
    }

    for (i, u) in result.clone().iter().enumerate() {
        if u.is_empty() {
            result.remove(i);
        }
    }

    return result;
}

pub fn join_vec(vec: Vec<Vec<u8>>) -> Vec<u8> {
    let mut result = vec![];

    for i in vec {
        for y in i {
            result.push(y);
        }
    }

    return result;
}