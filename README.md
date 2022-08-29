# Sekver
A interpreted programming language written in rust.


**WARNING**
This language should not be used for anything serious. It's a hobby project to learn more about compilers and interpreters

## Example code
```
imp std frm "std";

func main(): emp {
    var my_variable: str = "Hello World";
    var i: num = 0;

    while i < 3 {
        std::print_ln(my_variable);
        i += 1;
    }
}
```
