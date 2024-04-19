use array2d::Array2D;

#[derive(Clone, Copy)]
pub enum ShipGridSpace {
    Slot,
    Wall,
    Generator,
    Output,
}

pub fn create_grid(x: usize, y: usize) -> Array2D<ShipGridSpace> {
    let prefilled = Array2D::<ShipGridSpace>::filled_with(ShipGridSpace::Wall, y, x);
    prefilled
}
