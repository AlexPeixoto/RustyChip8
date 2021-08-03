pub struct BusState{
    //Decrement at 60hz
    pub delay:u8,
    //Beeps if not 8
    pub sound:u8,
    
    //stops processing until a key is pressed
    pub lock_until_pressed: bool,
}


