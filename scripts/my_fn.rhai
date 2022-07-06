// import another script with some alias
// import "process" as proc;           // this is evaluated every time
let my_var = 3;
fn hello(s, y) {
    // hopefully 'my_var' is in scope when this is called
    print("my_local_var: " + my_local_var); // from rust caller
    s.len + y 
}