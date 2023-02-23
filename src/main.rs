enum IpAddrKind {
    V4,
    V6
}

struct IpAddr{
  kind: IpAddrKind,
  address: String
}

fn main(){
  //Enums
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
}

fn route(ip_Kind: IpAddrKind){

}