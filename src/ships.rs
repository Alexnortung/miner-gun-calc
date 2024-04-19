use array2d::Array2D;

use crate::ship_grid::ShipGridSpace;

fn create_starter() -> Array2D<ShipGridSpace> {
    Array2D::<ShipGridSpace>::from_rows(&vec![
        vec![ShipGridSpace::Output],
        vec![ShipGridSpace::Slot],
        vec![ShipGridSpace::Generator],
    ])
    .unwrap()
}
