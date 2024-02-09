#![allow(dead_code, unused-variables)]
struct credentials{
    username: String,
    password: String,
}
enum status
{
    Connected,
    Interrupted,
}

fn connect_to_database()-> status{
    return status::Connected;
}

fn authenticate(creds:Credentials)
{
    if let status::Connected = connect_to_database()
    {
        login(creds);
    }
}

fn login(creds: Credentials)
{
    get_user();
}

fn get_user()
{

}

fn logout(){
    
}
