use break_infinity::{self, Decimal};
use crate::{global_funcs, Wawa, CONTAINMENT_BOUNDS, WAWA_SIZE};

pub fn update(app: &mut Wawa) {
    let distance = 1.0;
    let accessible_bounds_x  = crate::CONTAINMENT_BOUNDS.x - (CONTAINMENT_BOUNDS.x - WAWA_SIZE.x/2.0)..CONTAINMENT_BOUNDS.x - WAWA_SIZE.x/2.0;
    let accessible_bounds_y =  CONTAINMENT_BOUNDS.y - (CONTAINMENT_BOUNDS.y - WAWA_SIZE.y/2.0)..CONTAINMENT_BOUNDS.y - WAWA_SIZE.x/2.0;
    let velocity_range =-2.0..2.0;

    if app.wawapos.len() < app.max_wawas {
        let randx = rand::random_range(accessible_bounds_x.clone());
        let randy = rand::random_range(accessible_bounds_y.clone());
        let velx = rand::random_range(velocity_range.clone());
        let vely = rand::random_range(velocity_range.clone());
        app.wawapos.push(((randx,randy),(velx,vely)));
    }

    // temporary variable to collect indices to remove
    let mut ind_to_rmv: Vec<usize> = Vec::new();

    for (i,((x, y), (vx,vy))) in app.wawapos.iter_mut().enumerate() {
        // eprintln!("{:?}: {:?},{:?} {:?},{:?}",i,x,y,vx,vy);

        let  new_x = *x + distance * *vx;
        let new_y = *y + distance * *vy;

        if new_x < accessible_bounds_x.start { *vx = -*vx;}
        else if new_x > accessible_bounds_x.end { *vx = -*vx;}
        if new_y < accessible_bounds_y.start { *vy = -*vy;}
        else if new_y > accessible_bounds_y.end { *vy = -*vy;} // reflect the wawa
        *x = new_x;
        *y = new_y;

        if let Some((x1, y1)) = app.click_pos {
            // see if the clicked point is close enough to (x, y)
            if (x1 - *x).abs() <= WAWA_SIZE.x/1.5 && (y1 - *y).abs() <= WAWA_SIZE.y/1.5 {
                ind_to_rmv.push(i);  // Collect the index for removal
                app.cash += app.upgradecounts[1]; 
                app.wawas_clicked += Decimal::new(1.);
            }
        }
    } // this handles the clicking and rendering

    // remove clicked wawas
    for index in ind_to_rmv.iter().rev() {
        app.wawapos.remove(*index);
        global_funcs::play_sound("../assets/owSfx.ogg");
    }
}