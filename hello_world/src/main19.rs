
#[derive(Debug)]
enum Connection{
    Pending,
    Connect,
    Disconnect
}


fn get_connect(current: &mut Connection){
    match current {
        Connection::Disconnect => {
            *current = Connection::Pending},
        Connection::Connect => {
            *current = Connection::Disconnect},
        Connection::Pending  => {
            *current = Connection::Connect}
    }
}


fn main(){
    let mut connection_status = Connection::Disconnect;
    println!("Connection Status: {:?}", &connection_status);
    get_connect(&mut connection_status);
    println!("Connection Status: {:?}", &connection_status);
    get_connect(&mut connection_status);
    println!("Connection Status: {:?}", &connection_status);
    get_connect(&mut connection_status);
    println!("Connection Status: {:?}", &connection_status);
    get_connect(&mut connection_status);
    println!("Connection Status: {:?}", &connection_status);
    get_connect(&mut connection_status);
    println!("Connection Status: {:?}", &connection_status);
    

}