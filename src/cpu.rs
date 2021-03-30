struct CPU{
    let SP:u16;
    let PC:u16;

    //16 V registers
    let V[u16, 0xF];
}