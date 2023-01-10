pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    } else if input.len() == 1 && input[0].0==input[0].1 {
        return Some(input.to_vec());
    }
    
    for tile in input {
        let other_tiles = filter_tiles(&tile, &input.to_vec());
        if let Some(chain) = try_chain(*tile, other_tiles) {
            let last = chain.len();
            if chain[0].0 == chain[last-1].1 {
                return Some(chain);
            }
        }
    }
    None
}

fn try_chain(start_tile: (u8,u8), other_tiles: Vec<(u8,u8)>) -> Option<Vec<(u8, u8)>> {
    let mut direction = 0;
    let mut result_chain = vec![start_tile];

    loop {
        let mut next_tiles: Vec<(u8,u8)> = other_tiles.iter().filter_map(|(fr,bk)| {
            if *fr == start_tile.1 && direction < 1 {
                Some((*fr,*bk))
            } else if *bk == start_tile.1 && direction > 0 {
                Some((*bk,*fr))
            } else {
                None
            }}).collect();

        if next_tiles.is_empty() {
            if direction > 0 {
                return None;
            }
            direction += 1;
        
        } else if other_tiles.len() == 1 {
            result_chain.append(&mut next_tiles);
            return Some(result_chain);

        } else {
            for tile in next_tiles {
                let new_tiles = filter_tiles(&tile, &other_tiles);
                
                if let Some(mut chain) = try_chain(tile, new_tiles) {
                    result_chain.append(&mut chain);
                    return Some(result_chain);
                }
            }
            return None;
        }
    }
    
}

fn filter_tiles(tile: &(u8,u8), others: &Vec<(u8,u8)>) -> Vec<(u8,u8)> {
    if let Some(pos) = others.iter().position(|x| x==tile || (tile.0 == x.1 && tile.1 == x.0)){
        let mut new_tiles: Vec<(u8,u8)> = others.clone();
        new_tiles.remove(pos);
        new_tiles
    } else {
        vec![]
    }
}

